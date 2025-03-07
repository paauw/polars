use crate::frame::select::Selection;
use crate::prelude::*;
use crate::utils::{split_ca, NoNull};
use crate::vector_hasher::{
    create_hash_and_keys_threaded_vectorized, prepare_hashed_relation,
    prepare_hashed_relation_threaded,
};
use crate::POOL;
use ahash::RandomState;
use hashbrown::HashMap;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;
use unsafe_unwrap::UnsafeUnwrap;

macro_rules! det_hash_prone_order {
    ($self:expr, $other:expr) => {{
        // The shortest relation will be used to create a hash table.
        let left_first = $self.len() > $other.len();
        let a;
        let b;
        if left_first {
            a = $self;
            b = $other;
        } else {
            b = $self;
            a = $other;
        }

        (a, b, !left_first)
    }};
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JoinType {
    Left,
    Inner,
    Outer,
}

unsafe fn get_hash_tbl<T>(
    h: u64,
    hash_tables: &[HashMap<T, Vec<u32>, RandomState>],
    len: u64,
) -> &HashMap<T, Vec<u32>, RandomState>
where
    T: Send + Hash + Eq + Sync + Copy,
{
    let mut idx = 0;
    for i in 0..len {
        if (h + i) % len == 0 {
            idx = i as usize;
        }
    }
    hash_tables.get_unchecked(idx)
}

#[allow(clippy::needless_collect)]
fn hash_join_tuples_inner_threaded<T, I, J>(
    a: Vec<I>,
    b: Vec<J>,
    // Because b should be the shorter relation we could need to swap to keep left left and right right.
    swap: bool,
) -> Vec<(u32, u32)>
where
    I: Iterator<Item = T> + Send,
    J: Iterator<Item = T> + Send,
    T: Send + Hash + Eq + Sync + Copy + Debug,
{
    // first we hash one relation
    let hash_tbls = prepare_hashed_relation_threaded(b);
    let random_state = hash_tbls[0].hasher().clone();
    let (probe_hashes, _) = create_hash_and_keys_threaded_vectorized(a, Some(random_state));

    let n_tables = hash_tbls.len() as u64;
    let offsets = probe_hashes
        .iter()
        .map(|ph| ph.len())
        .scan(0, |state, val| {
            let out = *state;
            *state += val;
            Some(out)
        })
        .collect::<Vec<_>>();
    // next we probe the other relation
    // code duplication is because we want to only do the swap check once
    POOL.install(|| {
        probe_hashes
            .into_par_iter()
            .zip(offsets)
            .map(|(probe_hashes, offset)| {
                // local reference
                let hash_tbls = &hash_tbls;
                let mut results =
                    Vec::with_capacity(probe_hashes.len() / POOL.current_num_threads());
                let local_offset = offset;
                // code duplication is to hoist swap out of the inner loop.
                if swap {
                    probe_hashes.iter().enumerate().for_each(|(idx_a, (h, k))| {
                        let idx_a = (idx_a + local_offset) as u32;
                        // probe table that contains the hashed value
                        let current_probe_table = unsafe { get_hash_tbl(*h, hash_tbls, n_tables) };

                        let entry = current_probe_table
                            .raw_entry()
                            .from_key_hashed_nocheck(*h, k);

                        if let Some((_, indexes_b)) = entry {
                            let tuples = indexes_b.iter().map(|&idx_b| (idx_b, idx_a));
                            results.extend(tuples);
                        }
                    });
                } else {
                    probe_hashes.iter().enumerate().for_each(|(idx_a, (h, k))| {
                        let idx_a = (idx_a + local_offset) as u32;
                        // probe table that contains the hashed value
                        let current_probe_table = unsafe { get_hash_tbl(*h, hash_tbls, n_tables) };

                        let entry = current_probe_table
                            .raw_entry()
                            .from_key_hashed_nocheck(*h, k);

                        if let Some((_, indexes_b)) = entry {
                            let tuples = indexes_b.iter().map(|&idx_b| (idx_a, idx_b));
                            results.extend(tuples);
                        }
                    });
                }

                results
            })
            .flatten()
            .collect()
    })
}

fn hash_join_tuples_left_threaded<T, I, J>(a: Vec<I>, b: Vec<J>) -> Vec<(u32, Option<u32>)>
where
    I: Iterator<Item = T> + Send,
    J: Iterator<Item = T> + Send,
    T: Send + Hash + Eq + Sync + Copy + Debug,
{
    // first we hash one relation
    let hash_tbls = prepare_hashed_relation_threaded(b);
    let random_state = hash_tbls[0].hasher().clone();
    let (probe_hashes, _) = create_hash_and_keys_threaded_vectorized(a, Some(random_state));

    let offsets = probe_hashes
        .iter()
        .map(|ph| ph.len())
        .scan(0, |state, val| {
            let out = *state;
            *state += val;
            Some(out)
        })
        .collect::<Vec<_>>();

    let n_tables = hash_tbls.len() as u64;

    // next we probe the other relation
    // code duplication is because we want to only do the swap check once
    POOL.install(|| {
        probe_hashes
            .into_par_iter()
            .zip(offsets)
            .map(|(probe_hashes, offset)| {
                // local reference
                let hash_tbls = &hash_tbls;
                let mut results =
                    Vec::with_capacity(probe_hashes.len() / POOL.current_num_threads());

                probe_hashes.iter().enumerate().for_each(|(idx_a, (h, k))| {
                    let idx_a = (idx_a + offset) as u32;
                    // probe table that contains the hashed value
                    let current_probe_table = unsafe { get_hash_tbl(*h, hash_tbls, n_tables) };

                    let entry = current_probe_table
                        .raw_entry()
                        .from_key_hashed_nocheck(*h, k);

                    match entry {
                        // left and right matches
                        Some((_, indexes_b)) => {
                            results.extend(indexes_b.iter().map(|&idx_b| (idx_a, Some(idx_b))))
                        }
                        // only left values, right = null
                        None => results.push((idx_a, None)),
                    }
                });
                results
            })
            .flatten()
            .collect()
    })
}

/// Hash join a and b.
///     b should be the shorter relation.
/// NOTE that T also can be an Option<T>. Nulls are seen as equal.
fn hash_join_tuples_inner<T>(
    a: impl Iterator<Item = T>,
    b: impl Iterator<Item = T>,
    // Because b should be the shorter relation we could need to swap to keep left left and right right.
    swap: bool,
) -> Vec<(u32, u32)>
where
    T: Hash + Eq + Copy,
{
    let mut results = Vec::new();
    // First we hash one relation
    let hash_tbl = prepare_hashed_relation(b);

    // Next we probe the other relation in the hash table
    // code duplication is because we want to only do the swap check once
    if swap {
        a.enumerate().for_each(|(idx_a, key)| {
            let idx_a = idx_a as u32;
            if let Some(indexes_b) = hash_tbl.get(&key) {
                let tuples = indexes_b.iter().map(|&idx_b| (idx_b, idx_a));
                results.extend(tuples)
            }
        });
    } else {
        a.enumerate().for_each(|(idx_a, key)| {
            let idx_a = idx_a as u32;
            if let Some(indexes_b) = hash_tbl.get(&key) {
                let tuples = indexes_b.iter().map(|&idx_b| (idx_a, idx_b));
                results.extend(tuples)
            }
        });
    }
    results
}

/// Hash join left. None/ Nulls are regarded as Equal
/// All left values are joined so no Option<usize> there.
fn hash_join_tuples_left<T>(
    a: impl Iterator<Item = T>,
    b: impl Iterator<Item = T>,
) -> Vec<(u32, Option<u32>)>
where
    T: Hash + Eq + Copy,
{
    let mut results = Vec::new();
    // First we hash one relation
    let hash_tbl = prepare_hashed_relation(b);

    // Next we probe the other relation in the hash table
    a.enumerate().for_each(|(idx_a, key)| {
        let idx_a = idx_a as u32;
        match hash_tbl.get(&key) {
            // left and right matches
            Some(indexes_b) => results.extend(indexes_b.iter().map(|&idx_b| (idx_a, Some(idx_b)))),
            // only left values, right = null
            None => results.push((idx_a, None)),
        }
    });
    results
}

/// Hash join outer. Both left and right can have no match so Options
/// We accept a closure as we need to do two passes over the same iterators.
fn hash_join_tuples_outer<T, I, J>(a: I, b: J, swap: bool) -> Vec<(Option<u32>, Option<u32>)>
where
    I: Iterator<Item = T>,
    J: Iterator<Item = T>,
    T: Hash + Eq + Copy + Sync,
{
    let mut results = Vec::with_capacity(a.size_hint().0 + b.size_hint().0);

    // prepare hash table
    let mut hash_tbl = prepare_hashed_relation(b);

    // probe the hash table.
    // Note: indexes from b that are not matched will be None, Some(idx_b)
    // Therefore we remove the matches and the remaining will be joined from the right

    // code duplication is because we want to only do the swap check once
    if swap {
        a.enumerate().for_each(|(idx_a, key)| {
            let idx_a = idx_a as u32;
            match hash_tbl.remove(&key) {
                // left and right matches
                Some(indexes_b) => {
                    results.extend(indexes_b.iter().map(|&idx_b| (Some(idx_b), Some(idx_a))))
                }
                // only left values, right = null
                None => {
                    results.push((None, Some(idx_a)));
                }
            }
        });
        hash_tbl.iter().for_each(|(_k, indexes_b)| {
            // remaining joined values from the right table
            results.extend(indexes_b.iter().map(|&idx_b| (Some(idx_b), None)))
        });
    } else {
        a.enumerate().for_each(|(idx_a, key)| {
            let idx_a = idx_a as u32;
            match hash_tbl.remove(&key) {
                // left and right matches
                Some(indexes_b) => {
                    results.extend(indexes_b.iter().map(|&idx_b| (Some(idx_a), Some(idx_b))))
                }
                // only left values, right = null
                None => {
                    results.push((Some(idx_a), None));
                }
            }
        });
        hash_tbl.iter().for_each(|(_k, indexes_b)| {
            // remaining joined values from the right table
            results.extend(indexes_b.iter().map(|&idx_b| (None, Some(idx_b))))
        });
    };

    results
}

pub(crate) trait HashJoin<T> {
    fn hash_join_inner(&self, _other: &ChunkedArray<T>) -> Vec<(u32, u32)> {
        unimplemented!()
    }
    fn hash_join_left(&self, _other: &ChunkedArray<T>) -> Vec<(u32, Option<u32>)> {
        unimplemented!()
    }
    fn hash_join_outer(&self, _other: &ChunkedArray<T>) -> Vec<(Option<u32>, Option<u32>)> {
        unimplemented!()
    }
}

macro_rules! impl_float_hash_join {
    ($type: ty, $ca: ty) => {
        impl HashJoin<$type> for $ca {
            fn hash_join_inner(&self, other: &$ca) -> Vec<(u32, u32)> {
                let (a, b, swap) = det_hash_prone_order!(self, other);

                let n_threads = n_join_threads();
                let splitted_a = split_ca(a, n_threads).unwrap();
                let splitted_b = split_ca(b, n_threads).unwrap();

                match (a.null_count(), b.null_count()) {
                    (0, 0) => {
                        let iters_a = splitted_a
                            .iter()
                            .map(|ca| ca.into_no_null_iter().map(|v| v.to_bits()))
                            .collect_vec();
                        let iters_b = splitted_b
                            .iter()
                            .map(|ca| ca.into_no_null_iter().map(|v| v.to_bits()))
                            .collect_vec();
                        hash_join_tuples_inner_threaded(iters_a, iters_b, swap)
                    }
                    _ => {
                        let iters_a = splitted_a
                            .iter()
                            .map(|ca| ca.into_iter().map(|opt_v| opt_v.map(|v| v.to_bits())))
                            .collect_vec();
                        let iters_b = splitted_b
                            .iter()
                            .map(|ca| ca.into_iter().map(|opt_v| opt_v.map(|v| v.to_bits())))
                            .collect_vec();
                        hash_join_tuples_inner_threaded(iters_a, iters_b, swap)
                    }
                }
            }
            fn hash_join_left(&self, other: &$ca) -> Vec<(u32, Option<u32>)> {
                let n_threads = n_join_threads();

                let a = self;
                let b = other;
                let splitted_a = split_ca(a, n_threads).unwrap();
                let splitted_b = split_ca(b, n_threads).unwrap();

                match (a.null_count(), b.null_count()) {
                    (0, 0) => {
                        let iters_a = splitted_a
                            .iter()
                            .map(|ca| ca.into_no_null_iter().map(|v| v.to_bits()))
                            .collect_vec();
                        let iters_b = splitted_b
                            .iter()
                            .map(|ca| ca.into_no_null_iter().map(|v| v.to_bits()))
                            .collect_vec();
                        hash_join_tuples_left_threaded(iters_a, iters_b)
                    }
                    _ => {
                        let iters_a = splitted_a
                            .iter()
                            .map(|ca| ca.into_iter().map(|opt_v| opt_v.map(|v| v.to_bits())))
                            .collect_vec();
                        let iters_b = splitted_b
                            .iter()
                            .map(|ca| ca.into_iter().map(|opt_v| opt_v.map(|v| v.to_bits())))
                            .collect_vec();
                        hash_join_tuples_left_threaded(iters_a, iters_b)
                    }
                }
            }
            fn hash_join_outer(&self, other: &$ca) -> Vec<(Option<u32>, Option<u32>)> {
                let (a, b, swap) = det_hash_prone_order!(self, other);

                match (a.null_count() == 0, b.null_count() == 0) {
                    (true, true) => hash_join_tuples_outer(
                        a.into_no_null_iter().map(|v| v.to_bits()),
                        b.into_no_null_iter().map(|v| v.to_bits()),
                        swap,
                    ),
                    _ => hash_join_tuples_outer(
                        a.into_iter().map(|opt_v| opt_v.map(|v| v.to_bits())),
                        b.into_iter().map(|opt_v| opt_v.map(|v| v.to_bits())),
                        swap,
                    ),
                }
            }
        }
    };
}

impl_float_hash_join!(Float32Type, Float32Chunked);
impl_float_hash_join!(Float64Type, Float64Chunked);

impl HashJoin<ListType> for ListChunked {}
impl HashJoin<CategoricalType> for CategoricalChunked {
    fn hash_join_inner(&self, other: &CategoricalChunked) -> Vec<(u32, u32)> {
        self.deref().hash_join_inner(&other.cast().unwrap())
    }
    fn hash_join_left(&self, other: &CategoricalChunked) -> Vec<(u32, Option<u32>)> {
        self.deref().hash_join_left(&other.cast().unwrap())
    }
    fn hash_join_outer(&self, other: &CategoricalChunked) -> Vec<(Option<u32>, Option<u32>)> {
        self.deref().hash_join_outer(&other.cast().unwrap())
    }
}

fn n_join_threads() -> usize {
    let max = std::env::var("POLARS_MAX_THREADS")
        .map(|s| s.parse::<usize>().expect("integer"))
        .unwrap_or(usize::MAX);
    std::cmp::min(num_cpus::get(), max)
}

impl<T> HashJoin<T> for ChunkedArray<T>
where
    T: PolarsIntegerType + Sync,
    T::Native: Eq + Hash,
{
    fn hash_join_inner(&self, other: &ChunkedArray<T>) -> Vec<(u32, u32)> {
        let (a, b, swap) = det_hash_prone_order!(self, other);

        let n_threads = n_join_threads();
        let splitted_a = split_ca(a, n_threads).unwrap();
        let splitted_b = split_ca(b, n_threads).unwrap();

        match (a.null_count(), b.null_count()) {
            (0, 0) => {
                let iters_a = splitted_a
                    .iter()
                    .map(|ca| ca.into_no_null_iter())
                    .collect_vec();
                let iters_b = splitted_b
                    .iter()
                    .map(|ca| ca.into_no_null_iter())
                    .collect_vec();
                hash_join_tuples_inner_threaded(iters_a, iters_b, swap)
            }
            _ => {
                let iters_a = splitted_a.iter().map(|ca| ca.into_iter()).collect_vec();
                let iters_b = splitted_b.iter().map(|ca| ca.into_iter()).collect_vec();
                hash_join_tuples_inner_threaded(iters_a, iters_b, swap)
            }
        }
    }

    fn hash_join_left(&self, other: &ChunkedArray<T>) -> Vec<(u32, Option<u32>)> {
        let n_threads = n_join_threads();

        let a = self;
        let b = other;
        let splitted_a = split_ca(a, n_threads).unwrap();
        let splitted_b = split_ca(b, n_threads).unwrap();

        match (a.null_count(), b.null_count()) {
            (0, 0) => {
                let iters_a = splitted_a
                    .iter()
                    .map(|ca| ca.into_no_null_iter())
                    .collect_vec();
                let iters_b = splitted_b
                    .iter()
                    .map(|ca| ca.into_no_null_iter())
                    .collect_vec();
                hash_join_tuples_left_threaded(iters_a, iters_b)
            }
            _ => {
                let iters_a = splitted_a.iter().map(|ca| ca.into_iter()).collect_vec();
                let iters_b = splitted_b.iter().map(|ca| ca.into_iter()).collect_vec();
                hash_join_tuples_left_threaded(iters_a, iters_b)
            }
        }
    }

    fn hash_join_outer(&self, other: &ChunkedArray<T>) -> Vec<(Option<u32>, Option<u32>)> {
        let (a, b, swap) = det_hash_prone_order!(self, other);

        match (a.null_count() == 0, b.null_count() == 0) {
            (true, true) => {
                hash_join_tuples_outer(a.into_no_null_iter(), b.into_no_null_iter(), swap)
            }
            _ => hash_join_tuples_outer(a.into_iter(), b.into_iter(), swap),
        }
    }
}

impl HashJoin<BooleanType> for BooleanChunked {
    fn hash_join_inner(&self, other: &BooleanChunked) -> Vec<(u32, u32)> {
        let (a, b, swap) = det_hash_prone_order!(self, other);

        // Create the join tuples
        match (a.null_count() == 0, b.null_count() == 0) {
            (true, true) => {
                hash_join_tuples_inner(a.into_no_null_iter(), b.into_no_null_iter(), swap)
            }
            _ => hash_join_tuples_inner(a.into_iter(), b.into_iter(), swap),
        }
    }

    fn hash_join_left(&self, other: &BooleanChunked) -> Vec<(u32, Option<u32>)> {
        match (self.null_count() == 0, other.null_count() == 0) {
            (true, true) => {
                hash_join_tuples_left(self.into_no_null_iter(), other.into_no_null_iter())
            }
            _ => hash_join_tuples_left(self.into_iter(), other.into_iter()),
        }
    }

    fn hash_join_outer(&self, other: &BooleanChunked) -> Vec<(Option<u32>, Option<u32>)> {
        let (a, b, swap) = det_hash_prone_order!(self, other);
        match (a.null_count() == 0, b.null_count() == 0) {
            (true, true) => {
                hash_join_tuples_outer(a.into_no_null_iter(), b.into_no_null_iter(), swap)
            }
            _ => hash_join_tuples_outer(a.into_iter(), b.into_iter(), swap),
        }
    }
}

impl HashJoin<Utf8Type> for Utf8Chunked {
    fn hash_join_inner(&self, other: &Utf8Chunked) -> Vec<(u32, u32)> {
        let (a, b, swap) = det_hash_prone_order!(self, other);

        let n_threads = n_join_threads();
        let splitted_a = split_ca(a, n_threads).unwrap();
        let splitted_b = split_ca(b, n_threads).unwrap();

        match (a.null_count(), b.null_count()) {
            (0, 0) => {
                let iters_a = splitted_a
                    .iter()
                    .map(|ca| ca.into_no_null_iter())
                    .collect_vec();
                let iters_b = splitted_b
                    .iter()
                    .map(|ca| ca.into_no_null_iter())
                    .collect_vec();
                hash_join_tuples_inner_threaded(iters_a, iters_b, swap)
            }
            _ => {
                let iters_a = splitted_a.iter().map(|ca| ca.into_iter()).collect_vec();
                let iters_b = splitted_b.iter().map(|ca| ca.into_iter()).collect_vec();
                hash_join_tuples_inner_threaded(iters_a, iters_b, swap)
            }
        }
    }

    fn hash_join_left(&self, other: &Utf8Chunked) -> Vec<(u32, Option<u32>)> {
        let n_threads = n_join_threads();

        let a = self;
        let b = other;
        let splitted_a = split_ca(a, n_threads).unwrap();
        let splitted_b = split_ca(b, n_threads).unwrap();

        match (a.null_count(), b.null_count()) {
            (0, 0) => {
                let iters_a = splitted_a
                    .iter()
                    .map(|ca| ca.into_no_null_iter())
                    .collect_vec();
                let iters_b = splitted_b
                    .iter()
                    .map(|ca| ca.into_no_null_iter())
                    .collect_vec();
                hash_join_tuples_left_threaded(iters_a, iters_b)
            }
            _ => {
                let iters_a = splitted_a.iter().map(|ca| ca.into_iter()).collect_vec();
                let iters_b = splitted_b.iter().map(|ca| ca.into_iter()).collect_vec();
                hash_join_tuples_left_threaded(iters_a, iters_b)
            }
        }
    }

    fn hash_join_outer(&self, other: &Utf8Chunked) -> Vec<(Option<u32>, Option<u32>)> {
        let (a, b, swap) = det_hash_prone_order!(self, other);
        match (a.null_count() == 0, b.null_count() == 0) {
            (true, true) => {
                hash_join_tuples_outer(a.into_no_null_iter(), b.into_no_null_iter(), swap)
            }
            _ => hash_join_tuples_outer(a.into_iter(), b.into_iter(), swap),
        }
    }
}

pub trait ZipOuterJoinColumn {
    fn zip_outer_join_column(
        &self,
        _right_column: &Series,
        _opt_join_tuples: &[(Option<u32>, Option<u32>)],
    ) -> Series {
        unimplemented!()
    }
}

impl<T> ZipOuterJoinColumn for ChunkedArray<T>
where
    T: PolarsIntegerType,
    ChunkedArray<T>: IntoSeries,
{
    fn zip_outer_join_column(
        &self,
        right_column: &Series,
        opt_join_tuples: &[(Option<u32>, Option<u32>)],
    ) -> Series {
        let right_ca = self.unpack_series_matching_type(right_column).unwrap();

        let left_rand_access = self.take_rand();
        let right_rand_access = right_ca.take_rand();

        opt_join_tuples
            .iter()
            .map(|(opt_left_idx, opt_right_idx)| {
                if let Some(left_idx) = opt_left_idx {
                    unsafe { left_rand_access.get_unchecked(*left_idx as usize) }
                } else {
                    unsafe {
                        let right_idx = opt_right_idx.unsafe_unwrap();
                        right_rand_access.get_unchecked(right_idx as usize)
                    }
                }
            })
            .collect::<NoNull<ChunkedArray<T>>>()
            .into_inner()
            .into_series()
    }
}

impl ZipOuterJoinColumn for Float32Chunked {}
impl ZipOuterJoinColumn for Float64Chunked {}
impl ZipOuterJoinColumn for ListChunked {}
impl ZipOuterJoinColumn for CategoricalChunked {}
#[cfg(feature = "object")]
impl<T> ZipOuterJoinColumn for ObjectChunked<T> {}

macro_rules! impl_zip_outer_join {
    ($chunkedtype:ident) => {
        impl ZipOuterJoinColumn for $chunkedtype {
            fn zip_outer_join_column(
                &self,
                right_column: &Series,
                opt_join_tuples: &[(Option<u32>, Option<u32>)],
            ) -> Series {
                let right_ca = self.unpack_series_matching_type(right_column).unwrap();

                let left_rand_access = self.take_rand();
                let right_rand_access = right_ca.take_rand();

                opt_join_tuples
                    .iter()
                    .map(|(opt_left_idx, opt_right_idx)| {
                        if let Some(left_idx) = opt_left_idx {
                            unsafe { left_rand_access.get_unchecked(*left_idx as usize) }
                        } else {
                            unsafe {
                                let right_idx = opt_right_idx.unsafe_unwrap();
                                right_rand_access.get_unchecked(right_idx as usize)
                            }
                        }
                    })
                    .collect::<$chunkedtype>()
                    .into_series()
            }
        }
    };
}
impl_zip_outer_join!(BooleanChunked);
impl_zip_outer_join!(Utf8Chunked);

impl DataFrame {
    /// Utility method to finish a join.
    fn finish_join(&self, mut df_left: DataFrame, mut df_right: DataFrame) -> Result<DataFrame> {
        let mut left_names = HashSet::with_capacity_and_hasher(df_left.width(), RandomState::new());

        df_left.columns.iter().for_each(|series| {
            left_names.insert(series.name());
        });

        let mut rename_strs = Vec::with_capacity(df_right.width());

        df_right.columns.iter().for_each(|series| {
            if left_names.contains(series.name()) {
                rename_strs.push(series.name().to_owned())
            }
        });

        for name in rename_strs {
            df_right.rename(&name, &format!("{}_right", name))?;
        }

        df_left.hstack_mut(&df_right.columns)?;
        Ok(df_left)
    }

    fn create_left_df<B: Sync>(&self, join_tuples: &[(u32, B)]) -> DataFrame {
        unsafe { self.take_iter_unchecked(join_tuples.iter().map(|(left, _right)| *left as usize)) }
    }

    /// Generic join method. Can be used to join on multiple columns.
    pub fn join<'a, J, S1: Selection<'a, J>, S2: Selection<'a, J>>(
        &self,
        other: &DataFrame,
        left_on: S1,
        right_on: S2,
        how: JoinType,
    ) -> Result<DataFrame> {
        let selected_left = self.select_series(left_on)?;
        let selected_right = other.select_series(right_on)?;
        assert_eq!(selected_right.len(), selected_left.len());

        if selected_left.len() == 1 {
            return match how {
                JoinType::Inner => {
                    self.inner_join(other, selected_left[0].name(), selected_right[0].name())
                }
                JoinType::Left => {
                    self.left_join(other, selected_left[0].name(), selected_right[0].name())
                }
                JoinType::Outer => {
                    self.outer_join(other, selected_left[0].name(), selected_right[0].name())
                }
            };
        }

        macro_rules! det_hash_prone_order2 {
            ($self:expr, $other:expr) => {{
                // The shortest relation will be used to create a hash table.
                let left_first = $self.size_hint().0 > $other.size_hint().0;
                let a;
                let b;
                if left_first {
                    a = $self;
                    b = $other;
                } else {
                    b = $self;
                    a = $other;
                }

                (a, b, !left_first)
            }};
        }

        fn remove_selected(df: &DataFrame, selected: &[Series]) -> DataFrame {
            let mut new = None;
            for s in selected {
                new = match new {
                    None => Some(df.drop(s.name()).unwrap()),
                    Some(new) => Some(new.drop(s.name()).unwrap()),
                }
            }
            new.unwrap()
        }

        match how {
            JoinType::Inner => {
                let join_tuples = match selected_left.len() {
                    2 => {
                        let a = static_zip!(selected_left, 1);
                        let b = static_zip!(selected_right, 1);
                        let (a, b, swap) = det_hash_prone_order2!(a, b);
                        hash_join_tuples_inner(a, b, swap)
                    }
                    3 => {
                        let a = static_zip!(selected_left, 2);
                        let b = static_zip!(selected_right, 2);
                        let (a, b, swap) = det_hash_prone_order2!(a, b);
                        hash_join_tuples_inner(a, b, swap)
                    }
                    4 => {
                        let a = static_zip!(selected_left, 3);
                        let b = static_zip!(selected_right, 3);
                        let (a, b, swap) = det_hash_prone_order2!(a, b);
                        hash_join_tuples_inner(a, b, swap)
                    }
                    5 => {
                        let a = static_zip!(selected_left, 4);
                        let b = static_zip!(selected_right, 4);
                        let (a, b, swap) = det_hash_prone_order2!(a, b);
                        hash_join_tuples_inner(a, b, swap)
                    }
                    6 => {
                        let a = static_zip!(selected_left, 5);
                        let b = static_zip!(selected_right, 5);
                        let (a, b, swap) = det_hash_prone_order2!(a, b);
                        hash_join_tuples_inner(a, b, swap)
                    }
                    _ => todo!(),
                };

                let (df_left, df_right) = POOL.join(
                    || self.create_left_df(&join_tuples),
                    || unsafe {
                        // remove join columns
                        remove_selected(other, &selected_right).take_iter_unchecked(
                            join_tuples.iter().map(|(_left, right)| *right as usize),
                        )
                    },
                );
                self.finish_join(df_left, df_right)
            }
            JoinType::Left => {
                let join_tuples = match selected_left.len() {
                    2 => {
                        let a = static_zip!(selected_left, 1);
                        let b = static_zip!(selected_right, 1);
                        hash_join_tuples_left(a, b)
                    }
                    3 => {
                        let a = static_zip!(selected_left, 2);
                        let b = static_zip!(selected_right, 2);
                        hash_join_tuples_left(a, b)
                    }
                    4 => {
                        let a = static_zip!(selected_left, 3);
                        let b = static_zip!(selected_right, 3);
                        hash_join_tuples_left(a, b)
                    }
                    5 => {
                        let a = static_zip!(selected_left, 4);
                        let b = static_zip!(selected_right, 4);
                        hash_join_tuples_left(a, b)
                    }
                    6 => {
                        let a = static_zip!(selected_left, 5);
                        let b = static_zip!(selected_right, 5);
                        hash_join_tuples_left(a, b)
                    }
                    _ => todo!(),
                };

                let (df_left, df_right) = POOL.join(
                    || self.create_left_df(&join_tuples),
                    || unsafe {
                        // remove join columns
                        remove_selected(other, &selected_right).take_opt_iter_unchecked(
                            join_tuples
                                .iter()
                                .map(|(_left, right)| right.map(|i| i as usize)),
                        )
                    },
                );
                self.finish_join(df_left, df_right)
            }
            JoinType::Outer => {
                let opt_join_tuples = match selected_left.len() {
                    2 => {
                        let a = static_zip!(selected_left, 1);
                        let b = static_zip!(selected_right, 1);
                        let (a, b, swap) = det_hash_prone_order2!(a, b);
                        hash_join_tuples_outer(a, b, swap)
                    }
                    3 => {
                        let a = static_zip!(selected_left, 2);
                        let b = static_zip!(selected_right, 2);
                        let (a, b, swap) = det_hash_prone_order2!(a, b);
                        hash_join_tuples_outer(a, b, swap)
                    }
                    4 => {
                        let a = static_zip!(selected_left, 3);
                        let b = static_zip!(selected_right, 3);
                        let (a, b, swap) = det_hash_prone_order2!(a, b);
                        hash_join_tuples_outer(a, b, swap)
                    }
                    5 => {
                        let a = static_zip!(selected_left, 4);
                        let b = static_zip!(selected_right, 4);
                        let (a, b, swap) = det_hash_prone_order2!(a, b);
                        hash_join_tuples_outer(a, b, swap)
                    }
                    6 => {
                        let a = static_zip!(selected_left, 5);
                        let b = static_zip!(selected_right, 5);
                        let (a, b, swap) = det_hash_prone_order2!(a, b);
                        hash_join_tuples_outer(a, b, swap)
                    }
                    _ => todo!(),
                };

                // Take the left and right dataframes by join tuples
                let (mut df_left, df_right) = POOL.join(
                    || unsafe {
                        remove_selected(self, &selected_left).take_opt_iter_unchecked(
                            opt_join_tuples
                                .iter()
                                .map(|(left, _right)| left.map(|i| i as usize)),
                        )
                    },
                    || unsafe {
                        remove_selected(other, &selected_right).take_opt_iter_unchecked(
                            opt_join_tuples
                                .iter()
                                .map(|(_left, right)| right.map(|i| i as usize)),
                        )
                    },
                );
                for (s_left, s_right) in selected_left.iter().zip(&selected_right) {
                    let mut s = s_left.zip_outer_join_column(s_right, &opt_join_tuples);
                    s.rename(s_left.name());
                    df_left.hstack_mut(&[s])?;
                }
                self.finish_join(df_left, df_right)
            }
        }
    }

    /// Perform an inner join on two DataFrames.
    ///
    /// # Example
    ///
    /// ```
    /// use polars_core::prelude::*;
    /// fn join_dfs(left: &DataFrame, right: &DataFrame) -> Result<DataFrame> {
    ///     left.inner_join(right, "join_column_left", "join_column_right")
    /// }
    /// ```
    pub fn inner_join(
        &self,
        other: &DataFrame,
        left_on: &str,
        right_on: &str,
    ) -> Result<DataFrame> {
        let s_left = self.column(left_on)?;
        let s_right = other.column(right_on)?;
        self.inner_join_from_series(other, s_left, s_right)
    }

    pub(crate) fn inner_join_from_series(
        &self,
        other: &DataFrame,
        s_left: &Series,
        s_right: &Series,
    ) -> Result<DataFrame> {
        let join_tuples = s_left.hash_join_inner(s_right);

        let (df_left, df_right) = POOL.join(
            || self.create_left_df(&join_tuples),
            || unsafe {
                other
                    .drop(s_right.name())
                    .unwrap()
                    .take_iter_unchecked(join_tuples.iter().map(|(_left, right)| *right as usize))
            },
        );
        self.finish_join(df_left, df_right)
    }

    /// Perform a left join on two DataFrames
    /// # Example
    ///
    /// ```
    /// use polars_core::prelude::*;
    /// fn join_dfs(left: &DataFrame, right: &DataFrame) -> Result<DataFrame> {
    ///     left.left_join(right, "join_column_left", "join_column_right")
    /// }
    /// ```
    pub fn left_join(&self, other: &DataFrame, left_on: &str, right_on: &str) -> Result<DataFrame> {
        let s_left = self.column(left_on)?;
        let s_right = other.column(right_on)?;
        self.left_join_from_series(other, s_left, s_right)
    }

    pub(crate) fn left_join_from_series(
        &self,
        other: &DataFrame,
        s_left: &Series,
        s_right: &Series,
    ) -> Result<DataFrame> {
        let opt_join_tuples = s_left.hash_join_left(s_right);

        let (df_left, df_right) = POOL.join(
            || self.create_left_df(&opt_join_tuples),
            || unsafe {
                other.drop(s_right.name()).unwrap().take_opt_iter_unchecked(
                    opt_join_tuples
                        .iter()
                        .map(|(_left, right)| right.map(|i| i as usize)),
                )
            },
        );
        self.finish_join(df_left, df_right)
    }

    /// Perform an outer join on two DataFrames
    /// # Example
    ///
    /// ```
    /// use polars_core::prelude::*;
    /// fn join_dfs(left: &DataFrame, right: &DataFrame) -> Result<DataFrame> {
    ///     left.outer_join(right, "join_column_left", "join_column_right")
    /// }
    /// ```
    pub fn outer_join(
        &self,
        other: &DataFrame,
        left_on: &str,
        right_on: &str,
    ) -> Result<DataFrame> {
        let s_left = self.column(left_on)?;
        let s_right = other.column(right_on)?;
        self.outer_join_from_series(other, s_left, s_right)
    }
    pub(crate) fn outer_join_from_series(
        &self,
        other: &DataFrame,
        s_left: &Series,
        s_right: &Series,
    ) -> Result<DataFrame> {
        // Get the indexes of the joined relations
        let opt_join_tuples = s_left.hash_join_outer(s_right);

        // Take the left and right dataframes by join tuples
        let (mut df_left, df_right) = POOL.join(
            || unsafe {
                self.drop(s_left.name()).unwrap().take_opt_iter_unchecked(
                    opt_join_tuples
                        .iter()
                        .map(|(left, _right)| left.map(|i| i as usize)),
                )
            },
            || unsafe {
                other.drop(s_right.name()).unwrap().take_opt_iter_unchecked(
                    opt_join_tuples
                        .iter()
                        .map(|(_left, right)| right.map(|i| i as usize)),
                )
            },
        );
        let mut s = s_left.zip_outer_join_column(s_right, &opt_join_tuples);
        s.rename(s_left.name());
        df_left.hstack_mut(&[s])?;
        self.finish_join(df_left, df_right)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use crate::toggle_string_cache;

    fn create_frames() -> (DataFrame, DataFrame) {
        let s0 = Series::new("days", &[0, 1, 2]);
        let s1 = Series::new("temp", &[22.1, 19.9, 7.]);
        let s2 = Series::new("rain", &[0.2, 0.1, 0.3]);
        let temp = DataFrame::new(vec![s0, s1, s2]).unwrap();

        let s0 = Series::new("days", &[1, 2, 3, 1]);
        let s1 = Series::new("rain", &[0.1, 0.2, 0.3, 0.4]);
        let rain = DataFrame::new(vec![s0, s1]).unwrap();
        (temp, rain)
    }

    #[test]
    fn test_inner_join() {
        let (temp, rain) = create_frames();

        for i in 1..8 {
            std::env::set_var("POLARS_MAX_THREADS", format!("{}", i));
            let joined = temp.inner_join(&rain, "days", "days").unwrap();

            let join_col_days = Series::new("days", &[1, 2, 1]);
            let join_col_temp = Series::new("temp", &[19.9, 7., 19.9]);
            let join_col_rain = Series::new("rain", &[0.1, 0.3, 0.1]);
            let join_col_rain_right = Series::new("rain_right", [0.1, 0.2, 0.4].as_ref());
            let true_df = DataFrame::new(vec![
                join_col_days,
                join_col_temp,
                join_col_rain,
                join_col_rain_right,
            ])
            .unwrap();

            println!("{}", joined);
            assert!(joined.frame_equal(&true_df));
        }
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_left_join() {
        for i in 1..8 {
            std::env::set_var("POLARS_MAX_THREADS", format!("{}", i));
            let s0 = Series::new("days", &[0, 1, 2, 3, 4]);
            let s1 = Series::new("temp", &[22.1, 19.9, 7., 2., 3.]);
            let temp = DataFrame::new(vec![s0, s1]).unwrap();

            let s0 = Series::new("days", &[1, 2]);
            let s1 = Series::new("rain", &[0.1, 0.2]);
            let rain = DataFrame::new(vec![s0, s1]).unwrap();
            let joined = temp.left_join(&rain, "days", "days").unwrap();
            println!("{}", &joined);
            assert_eq!(
                (joined.column("rain").unwrap().sum::<f32>().unwrap() * 10.).round(),
                3.
            );
            assert_eq!(joined.column("rain").unwrap().null_count(), 3);

            // test join on utf8
            let s0 = Series::new("days", &["mo", "tue", "wed", "thu", "fri"]);
            let s1 = Series::new("temp", &[22.1, 19.9, 7., 2., 3.]);
            let temp = DataFrame::new(vec![s0, s1]).unwrap();

            let s0 = Series::new("days", &["tue", "wed"]);
            let s1 = Series::new("rain", &[0.1, 0.2]);
            let rain = DataFrame::new(vec![s0, s1]).unwrap();
            let joined = temp.left_join(&rain, "days", "days").unwrap();
            println!("{}", &joined);
            assert_eq!(
                (joined.column("rain").unwrap().sum::<f32>().unwrap() * 10.).round(),
                3.
            );
            assert_eq!(joined.column("rain").unwrap().null_count(), 3);
        }
    }

    #[test]
    fn test_outer_join() {
        let (temp, rain) = create_frames();
        let joined = temp.outer_join(&rain, "days", "days").unwrap();
        println!("{:?}", &joined);
        assert_eq!(joined.height(), 5);
        assert_eq!(joined.column("days").unwrap().sum::<i32>(), Some(7));
    }

    #[test]
    fn test_join_with_nulls() {
        let dts = &[20, 21, 22, 23, 24, 25, 27, 28];
        let vals = &[1.2, 2.4, 4.67, 5.8, 4.4, 3.6, 7.6, 6.5];
        let df = DataFrame::new(vec![Series::new("date", dts), Series::new("val", vals)]).unwrap();

        let vals2 = &[Some(1.1), None, Some(3.3), None, None];
        let df2 = DataFrame::new(vec![
            Series::new("date", &dts[3..]),
            Series::new("val2", vals2),
        ])
        .unwrap();

        let joined = df.left_join(&df2, "date", "date").unwrap();
        assert_eq!(
            joined
                .column("val2")
                .unwrap()
                .f64()
                .unwrap()
                .get(joined.height() - 1),
            None
        );
    }

    fn get_dfs() -> (DataFrame, DataFrame) {
        let df_a = df! {
            "a" => &[1, 2, 1, 1],
            "b" => &["a", "b", "c", "c"],
            "c" => &[0, 1, 2, 3]
        }
        .unwrap();

        let df_b = df! {
            "foo" => &[1, 1, 1],
            "bar" => &["a", "c", "c"],
            "ham" => &["let", "var", "const"]
        }
        .unwrap();
        (df_a, df_b)
    }

    #[test]
    fn test_join_multiple_columns() {
        let (df_a, df_b) = get_dfs();

        // First do a hack with concatenated string dummy column
        let mut s = df_a
            .column("a")
            .unwrap()
            .cast::<Utf8Type>()
            .unwrap()
            .utf8()
            .unwrap()
            + df_a.column("b").unwrap().utf8().unwrap();
        s.rename("dummy");

        let df_a = df_a.with_column(s).unwrap();
        let mut s = df_b
            .column("foo")
            .unwrap()
            .cast::<Utf8Type>()
            .unwrap()
            .utf8()
            .unwrap()
            + df_b.column("bar").unwrap().utf8().unwrap();
        s.rename("dummy");
        let df_b = df_b.with_column(s).unwrap();

        let joined = df_a.left_join(&df_b, "dummy", "dummy").unwrap();
        let ham_col = joined.column("ham").unwrap();
        let ca = ham_col.utf8().unwrap();

        let correct_ham = &[
            Some("let"),
            None,
            Some("var"),
            Some("const"),
            Some("var"),
            Some("const"),
        ];

        assert_eq!(Vec::from(ca), correct_ham);

        // now check the join with multiple columns
        let joined = df_a
            .join(&df_b, &["a", "b"], &["foo", "bar"], JoinType::Left)
            .unwrap();
        let ca = joined.column("ham").unwrap().utf8().unwrap();
        assert_eq!(Vec::from(ca), correct_ham);
        let joined_inner_hack = df_a.inner_join(&df_b, "dummy", "dummy").unwrap();
        let joined_inner = df_a
            .join(&df_b, &["a", "b"], &["foo", "bar"], JoinType::Inner)
            .unwrap();
        assert!(joined_inner_hack
            .column("ham")
            .unwrap()
            .series_equal_missing(joined_inner.column("ham").unwrap()));

        let joined_outer_hack = df_a.outer_join(&df_b, "dummy", "dummy").unwrap();
        let joined_outer = df_a
            .join(&df_b, &["a", "b"], &["foo", "bar"], JoinType::Outer)
            .unwrap();
        assert!(joined_outer_hack
            .column("ham")
            .unwrap()
            .series_equal_missing(joined_outer.column("ham").unwrap()));
    }

    #[test]
    fn test_join_categorical() {
        toggle_string_cache(true);

        let (mut df_a, mut df_b) = get_dfs();

        df_a.may_apply("b", |s| s.cast_with_datatype(&DataType::Categorical))
            .unwrap();
        df_b.may_apply("bar", |s| s.cast_with_datatype(&DataType::Categorical))
            .unwrap();

        let out = df_a.join(&df_b, "b", "bar", JoinType::Left).unwrap();
        assert_eq!(out.shape(), (6, 5));
        let correct_ham = &[
            Some("let"),
            None,
            Some("var"),
            Some("const"),
            Some("var"),
            Some("const"),
        ];
        let ham_col = out.column("ham").unwrap();
        let ca = ham_col.utf8().unwrap();

        assert_eq!(Vec::from(ca), correct_ham);
    }

    #[test]
    fn empty_df_join() {
        let empty: Vec<String> = vec![];
        let left = DataFrame::new(vec![
            Series::new("key", &empty),
            Series::new("lval", &empty),
        ])
        .unwrap();

        let right = DataFrame::new(vec![
            Series::new("key", &["foo"]),
            Series::new("rval", &[4]),
        ])
        .unwrap();

        let res = left.inner_join(&right, "key", "key");

        assert!(res.is_ok());
        assert_eq!(res.unwrap().height(), 0);
        right.left_join(&left, "key", "key").unwrap();
        right.inner_join(&left, "key", "key").unwrap();
        right.outer_join(&left, "key", "key").unwrap();
    }
}
