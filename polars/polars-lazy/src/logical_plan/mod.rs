pub(crate) mod iterator;
pub(crate) mod optimizer;

use crate::logical_plan::LogicalPlan::CsvScan;
use crate::utils::{
    combine_predicates_expr, expr_to_root_column_exprs, expr_to_root_column_name,
    expr_to_root_column_names, has_expr, rename_expr_root_name,
};
use crate::{prelude::*, utils};
use ahash::RandomState;
use itertools::Itertools;
use polars_core::frame::hash_join::JoinType;
use polars_core::prelude::*;
use polars_io::csv_core::utils::infer_file_schema;
use polars_io::prelude::*;
use std::collections::HashSet;
use std::{
    cell::Cell,
    fmt::{self, Debug, Formatter, Write},
    sync::Arc,
};

#[cfg_attr(docsrs, doc(cfg(feature = "temporal")))]
#[cfg(feature = "temporal")]
use polars_core::utils::chrono::NaiveDateTime;

// Will be set/ unset in the fetch operation to communicate overwriting the number of rows to scan.
thread_local! {pub(crate) static FETCH_ROWS: Cell<Option<usize>> = Cell::new(None)}

#[derive(Clone, Copy)]
pub enum Context {
    Aggregation,
    Other,
}

pub trait DataFrameUdf: Send + Sync {
    fn call_udf(&self, df: DataFrame) -> Result<DataFrame>;
}

impl<F> DataFrameUdf for F
where
    F: Fn(DataFrame) -> Result<DataFrame> + Send + Sync,
{
    fn call_udf(&self, df: DataFrame) -> Result<DataFrame> {
        self(df)
    }
}

impl Debug for dyn DataFrameUdf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "udf")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralValue {
    Null,
    /// A binary true or false.
    Boolean(bool),
    /// A UTF8 encoded string type.
    Utf8(String),
    /// An unsigned 8-bit integer number.
    UInt8(u8),
    /// An unsigned 16-bit integer number.
    UInt16(u16),
    /// An unsigned 32-bit integer number.
    UInt32(u32),
    /// An unsigned 64-bit integer number.
    UInt64(u64),
    /// An 8-bit integer number.
    #[cfg(feature = "dtype-i8")]
    Int8(i8),
    /// A 16-bit integer number.
    #[cfg(feature = "dtype-i16")]
    Int16(i16),
    /// A 32-bit integer number.
    Int32(i32),
    /// A 64-bit integer number.
    Int64(i64),
    /// A 32-bit floating point number.
    Float32(f32),
    /// A 64-bit floating point number.
    Float64(f64),
    Range {
        low: i64,
        high: i64,
        data_type: DataType,
    },
    #[cfg(all(feature = "temporal", feature = "dtype-date64"))]
    DateTime(NaiveDateTime),
}

impl LiteralValue {
    /// Getter for the `DataType` of the value
    pub fn get_datatype(&self) -> DataType {
        match self {
            LiteralValue::Boolean(_) => DataType::Boolean,
            LiteralValue::UInt8(_) => DataType::UInt8,
            LiteralValue::UInt16(_) => DataType::UInt16,
            LiteralValue::UInt32(_) => DataType::UInt32,
            LiteralValue::UInt64(_) => DataType::UInt64,
            #[cfg(feature = "dtype-i8")]
            LiteralValue::Int8(_) => DataType::Int8,
            #[cfg(feature = "dtype-i16")]
            LiteralValue::Int16(_) => DataType::Int16,
            LiteralValue::Int32(_) => DataType::Int32,
            LiteralValue::Int64(_) => DataType::Int64,
            LiteralValue::Float32(_) => DataType::Float32,
            LiteralValue::Float64(_) => DataType::Float64,
            LiteralValue::Utf8(_) => DataType::Utf8,
            LiteralValue::Range { data_type, .. } => data_type.clone(),
            #[cfg(all(feature = "temporal", feature = "dtype-date64"))]
            LiteralValue::DateTime(_) => DataType::Date64,
            _ => panic!("Cannot treat {:?} as scalar value", self),
        }
    }
}

// https://stackoverflow.com/questions/1031076/what-are-projection-and-selection
#[derive(Clone)]
pub enum LogicalPlan {
    // filter on a boolean mask
    Selection {
        input: Box<LogicalPlan>,
        predicate: Expr,
    },
    Cache {
        input: Box<LogicalPlan>,
    },
    CsvScan {
        path: String,
        schema: SchemaRef,
        has_header: bool,
        delimiter: u8,
        ignore_errors: bool,
        skip_rows: usize,
        stop_after_n_rows: Option<usize>,
        with_columns: Option<Vec<String>>,
        /// Filters at the scan level
        predicate: Option<Expr>,
        /// Aggregations at the scan level
        aggregate: Vec<Expr>,
        cache: bool,
    },
    #[cfg(feature = "parquet")]
    #[cfg_attr(docsrs, doc(cfg(feature = "parquet")))]
    ParquetScan {
        path: String,
        schema: SchemaRef,
        with_columns: Option<Vec<String>>,
        predicate: Option<Expr>,
        aggregate: Vec<Expr>,
        stop_after_n_rows: Option<usize>,
        cache: bool,
    },
    // we keep track of the projection and selection as it is cheaper to first project and then filter
    DataFrameScan {
        df: Arc<DataFrame>,
        schema: SchemaRef,
        projection: Option<Vec<Expr>>,
        selection: Option<Expr>,
    },
    // a projection that doesn't have to be optimized
    // or may drop projected columns if they aren't in current schema (after optimization)
    LocalProjection {
        expr: Vec<Expr>,
        input: Box<LogicalPlan>,
        schema: SchemaRef,
    },
    // vertical selection
    Projection {
        expr: Vec<Expr>,
        input: Box<LogicalPlan>,
        schema: SchemaRef,
    },
    Aggregate {
        input: Box<LogicalPlan>,
        keys: Arc<Vec<Expr>>,
        aggs: Vec<Expr>,
        schema: SchemaRef,
        apply: Option<Arc<dyn DataFrameUdf>>,
    },
    Join {
        input_left: Box<LogicalPlan>,
        input_right: Box<LogicalPlan>,
        schema: SchemaRef,
        how: JoinType,
        left_on: Vec<Expr>,
        right_on: Vec<Expr>,
        allow_par: bool,
        force_par: bool,
    },
    HStack {
        input: Box<LogicalPlan>,
        exprs: Vec<Expr>,
        schema: SchemaRef,
    },
    Distinct {
        input: Box<LogicalPlan>,
        maintain_order: bool,
        subset: Arc<Option<Vec<String>>>,
    },
    Sort {
        input: Box<LogicalPlan>,
        by_column: String,
        reverse: bool,
    },
    Explode {
        input: Box<LogicalPlan>,
        columns: Vec<String>,
    },
    Slice {
        input: Box<LogicalPlan>,
        offset: usize,
        len: usize,
    },
    Melt {
        input: Box<LogicalPlan>,
        id_vars: Arc<Vec<String>>,
        value_vars: Arc<Vec<String>>,
        schema: SchemaRef,
    },
    Udf {
        input: Box<LogicalPlan>,
        function: Arc<dyn DataFrameUdf>,
        ///  allow predicate pushdown optimizations
        predicate_pd: bool,
        ///  allow projection pushdown optimizations
        projection_pd: bool,
        schema: Option<SchemaRef>,
    },
}

impl Default for LogicalPlan {
    fn default() -> Self {
        CsvScan {
            path: "".to_string(),
            schema: Arc::new(Schema::new(vec![Field::new("", DataType::Null)])),
            has_header: false,
            delimiter: b',',
            ignore_errors: false,
            skip_rows: 0,
            stop_after_n_rows: None,
            with_columns: None,
            predicate: None,
            aggregate: vec![],
            cache: true,
        }
    }
}

impl fmt::Debug for LogicalPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LogicalPlan::*;
        match self {
            Cache { input } => write!(f, "CACHE {:?}", input),
            #[cfg(feature = "parquet")]
            ParquetScan {
                path,
                schema,
                with_columns,
                predicate,
                ..
            } => {
                let total_columns = schema.fields().len();
                let mut n_columns = "*".to_string();
                if let Some(columns) = with_columns {
                    n_columns = format!("{}", columns.len());
                }
                write!(
                    f,
                    "PARQUET SCAN {}; PROJECT {}/{} COLUMNS; SELECTION: {:?}",
                    path, n_columns, total_columns, predicate
                )
            }
            Selection { predicate, input } => {
                write!(f, "FILTER\n\t{:?}\nFROM\n\t{:?}", predicate, input)
            }
            Melt { input, .. } => {
                write!(f, "MELT\n\t{:?}", input)
            }
            CsvScan {
                path,
                with_columns,
                schema,
                predicate,
                ..
            } => {
                let total_columns = schema.fields().len();
                let mut n_columns = "*".to_string();
                if let Some(columns) = with_columns {
                    n_columns = format!("{}", columns.len());
                }
                write!(
                    f,
                    "CSV SCAN {}; PROJECT {}/{} COLUMNS; SELECTION: {:?}",
                    path, n_columns, total_columns, predicate
                )
            }
            DataFrameScan {
                schema,
                projection,
                selection,
                ..
            } => {
                let total_columns = schema.fields().len();
                let mut n_columns = "*".to_string();
                if let Some(columns) = projection {
                    n_columns = format!("{}", columns.len());
                }

                write!(
                    f,
                    "TABLE: {:?}; PROJECT {}/{} COLUMNS; SELECTION: {:?}",
                    schema
                        .fields()
                        .iter()
                        .map(|f| f.name())
                        .take(4)
                        .collect::<Vec<_>>(),
                    n_columns,
                    total_columns,
                    selection
                )
            }
            Projection { expr, input, .. } => {
                write!(f, "SELECT {:?} COLUMNS \nFROM\n{:?}", expr.len(), input)
            }
            LocalProjection { expr, input, .. } => {
                write!(
                    f,
                    "LOCAL SELECT {:?} COLUMNS \nFROM\n{:?}",
                    expr.len(),
                    input
                )
            }
            Sort {
                input, by_column, ..
            } => write!(f, "SORT {:?} BY COLUMN {}", input, by_column),
            Explode { input, columns, .. } => {
                write!(f, "EXPLODE COLUMN(S) {:?} OF {:?}", columns, input)
            }
            Aggregate {
                input, keys, aggs, ..
            } => write!(f, "Aggregate\n\t{:?} BY {:?} FROM {:?}", aggs, keys, input),
            Join {
                input_left,
                input_right,
                left_on,
                right_on,
                ..
            } => write!(
                f,
                "JOIN\n\t({:?})\nWITH\n\t({:?})\nON (left: {:?} right: {:?})",
                input_left, input_right, left_on, right_on
            ),
            HStack { input, exprs, .. } => {
                write!(f, "STACK [{:?}\n\tWITH COLUMN(S)\n{:?}\n]", input, exprs)
            }
            Distinct { input, .. } => write!(f, "DISTINCT {:?}", input),
            Slice { input, offset, len } => {
                write!(f, "SLICE {:?}, offset: {}, len: {}", input, offset, len)
            }
            Udf { input, .. } => write!(f, "UDF {:?}", input),
        }
    }
}

fn fmt_predicate(predicate: Option<&Expr>) -> String {
    if let Some(predicate) = predicate {
        let n = 25;
        let mut pred_fmt = format!("{:?}", predicate);
        pred_fmt = pred_fmt.replace("[", "");
        pred_fmt = pred_fmt.replace("]", "");
        if pred_fmt.len() > n {
            pred_fmt.truncate(n);
            pred_fmt.push_str("...")
        }
        pred_fmt
    } else {
        "-".to_string()
    }
}

impl LogicalPlan {
    fn write_dot(
        &self,
        acc_str: &mut String,
        prev_node: &str,
        current_node: &str,
        id: usize,
    ) -> std::fmt::Result {
        if id == 0 {
            writeln!(acc_str, "graph  polars_query {{")
        } else {
            writeln!(acc_str, "\"{}\" -- \"{}\"", prev_node, current_node)
        }
    }

    pub(crate) fn dot(&self, acc_str: &mut String, id: usize, prev_node: &str) -> std::fmt::Result {
        use LogicalPlan::*;
        match self {
            Cache { input } => {
                let current_node = format!("CACHE [{}]", id);
                self.write_dot(acc_str, prev_node, &current_node, id)?;
                input.dot(acc_str, id + 1, &current_node)
            }
            Selection { predicate, input } => {
                let pred = fmt_predicate(Some(predicate));
                let current_node = format!("FILTER BY {} [{}]", pred, id);
                self.write_dot(acc_str, prev_node, &current_node, id)?;
                input.dot(acc_str, id + 1, &current_node)
            }
            CsvScan {
                path,
                with_columns,
                schema,
                predicate,
                ..
            } => {
                let total_columns = schema.fields().len();
                let mut n_columns = "*".to_string();
                if let Some(columns) = with_columns {
                    n_columns = format!("{}", columns.len());
                }
                let pred = fmt_predicate(predicate.as_ref());

                let current_node = format!(
                    "CSV SCAN {};\nπ {}/{};\nσ {}\n[{}]",
                    path, n_columns, total_columns, pred, id
                );
                if id == 0 {
                    self.write_dot(acc_str, prev_node, &current_node, id)?;
                    write!(acc_str, "\"{}\"", current_node)
                } else {
                    self.write_dot(acc_str, prev_node, &current_node, id)
                }
            }
            DataFrameScan {
                schema,
                projection,
                selection,
                ..
            } => {
                let total_columns = schema.fields().len();
                let mut n_columns = "*".to_string();
                if let Some(columns) = projection {
                    n_columns = format!("{}", columns.len());
                }

                let pred = fmt_predicate(selection.as_ref());
                let current_node = format!(
                    "TABLE\nπ {}/{};\nσ {}\n[{}]",
                    n_columns, total_columns, pred, id
                );
                if id == 0 {
                    self.write_dot(acc_str, prev_node, &current_node, id)?;
                    write!(acc_str, "\"{}\"", current_node)
                } else {
                    self.write_dot(acc_str, prev_node, &current_node, id)
                }
            }
            Projection { expr, input, .. } => {
                let current_node = format!(
                    "π {}/{} [{}]",
                    expr.len(),
                    input.schema().fields().len(),
                    id
                );
                self.write_dot(acc_str, prev_node, &current_node, id)?;
                input.dot(acc_str, id + 1, &current_node)
            }
            Sort {
                input, by_column, ..
            } => {
                let current_node = format!("SORT by {} [{}]", by_column, id);
                self.write_dot(acc_str, prev_node, &current_node, id)?;
                input.dot(acc_str, id + 1, &current_node)
            }
            LocalProjection { expr, input, .. } => {
                let current_node = format!(
                    "LOCAL π {}/{} [{}]",
                    expr.len(),
                    input.schema().fields().len(),
                    id
                );
                self.write_dot(acc_str, prev_node, &current_node, id)?;
                input.dot(acc_str, id + 1, &current_node)
            }
            Explode { input, columns, .. } => {
                let current_node = format!("EXPLODE {:?} [{}]", columns, id);
                self.write_dot(acc_str, prev_node, &current_node, id)?;
                input.dot(acc_str, id + 1, &current_node)
            }
            Melt { input, .. } => {
                let current_node = format!("MELT [{}]", id);
                self.write_dot(acc_str, prev_node, &current_node, id)?;
                input.dot(acc_str, id + 1, &current_node)
            }
            Aggregate {
                input, keys, aggs, ..
            } => {
                let mut s_keys = String::with_capacity(128);
                for key in keys.iter() {
                    s_keys.push_str(&format!("{:?}", key));
                }
                let current_node = format!("AGG {:?} BY {} [{}]", aggs, s_keys, id);
                self.write_dot(acc_str, prev_node, &current_node, id)?;
                input.dot(acc_str, id + 1, &current_node)
            }
            HStack { input, exprs, .. } => {
                let mut current_node = String::with_capacity(128);
                current_node.push_str("STACK");
                for e in exprs {
                    if let Expr::Alias(_, name) = e {
                        current_node.push_str(&format!(" {},", name));
                    } else {
                        for name in expr_to_root_column_names(e).iter().take(1) {
                            current_node.push_str(&format!(" {},", name));
                        }
                    }
                }
                current_node.push_str(&format!(" [{}]", id));
                self.write_dot(acc_str, prev_node, &current_node, id)?;
                input.dot(acc_str, id + 1, &current_node)
            }
            Slice { input, offset, len } => {
                let current_node = format!("SLICE offset: {}; len: {} [{}]", offset, len, id);
                self.write_dot(acc_str, prev_node, &current_node, id)?;
                input.dot(acc_str, id + 1, &current_node)
            }
            Distinct { input, subset, .. } => {
                let mut current_node = String::with_capacity(128);
                current_node.push_str("DISTINCT");
                if let Some(subset) = &**subset {
                    current_node.push_str(" BY ");
                    for name in subset.iter() {
                        current_node.push_str(&format!("{}, ", name));
                    }
                }
                current_node.push_str(&format!(" [{}]", id));

                self.write_dot(acc_str, prev_node, &current_node, id)?;
                input.dot(acc_str, id + 1, &current_node)
            }
            #[cfg(feature = "parquet")]
            ParquetScan {
                path,
                schema,
                with_columns,
                predicate,
                ..
            } => {
                let total_columns = schema.fields().len();
                let mut n_columns = "*".to_string();
                if let Some(columns) = with_columns {
                    n_columns = format!("{}", columns.len());
                }

                let pred = fmt_predicate(predicate.as_ref());
                let current_node = format!(
                    "PARQUET SCAN {};\nπ {}/{};\nσ {} [{}]",
                    path, n_columns, total_columns, pred, id
                );
                if id == 0 {
                    self.write_dot(acc_str, prev_node, &current_node, id)?;
                    write!(acc_str, "\"{}\"", current_node)
                } else {
                    self.write_dot(acc_str, prev_node, &current_node, id)
                }
            }
            Join {
                input_left,
                input_right,
                left_on,
                right_on,
                ..
            } => {
                let current_node =
                    format!("JOIN left {:?}; right: {:?} [{}]", left_on, right_on, id);
                self.write_dot(acc_str, prev_node, &current_node, id)?;
                input_left.dot(acc_str, id + 1, &current_node)?;
                input_right.dot(acc_str, id + 1, &current_node)
            }
            Udf { input, .. } => {
                let current_node = format!("UDF [{}]", id);
                self.write_dot(acc_str, prev_node, &current_node, id)?;
                input.dot(acc_str, id + 1, &current_node)
            }
        }
    }
}

fn replace_wildcard_with_column(expr: Expr, column_name: Arc<String>) -> Expr {
    match expr {
        Expr::Window {
            function,
            partition_by,
            order_by,
        } => Expr::Window {
            function: Box::new(replace_wildcard_with_column(*function, column_name)),
            partition_by,
            order_by,
        },
        Expr::Unique(expr) => {
            Expr::Unique(Box::new(replace_wildcard_with_column(*expr, column_name)))
        }
        Expr::Duplicated(expr) => {
            Expr::Duplicated(Box::new(replace_wildcard_with_column(*expr, column_name)))
        }
        Expr::Reverse(expr) => {
            Expr::Reverse(Box::new(replace_wildcard_with_column(*expr, column_name)))
        }
        Expr::Explode(expr) => {
            Expr::Explode(Box::new(replace_wildcard_with_column(*expr, column_name)))
        }
        Expr::Ternary {
            predicate,
            truthy,
            falsy,
        } => Expr::Ternary {
            predicate: Box::new(replace_wildcard_with_column(*predicate, column_name)),
            truthy,
            falsy,
        },
        Expr::Udf {
            input,
            function,
            output_type,
        } => Expr::Udf {
            input: Box::new(replace_wildcard_with_column(*input, column_name)),
            function,
            output_type,
        },
        Expr::BinaryFunction {
            input_a,
            input_b,
            function,
            output_field,
        } => Expr::BinaryFunction {
            input_a: Box::new(replace_wildcard_with_column(*input_a, column_name.clone())),
            input_b: Box::new(replace_wildcard_with_column(*input_b, column_name)),
            function,
            output_field,
        },
        Expr::BinaryExpr { left, op, right } => Expr::BinaryExpr {
            left: Box::new(replace_wildcard_with_column(*left, column_name.clone())),
            op,
            right: Box::new(replace_wildcard_with_column(*right, column_name)),
        },
        Expr::Wildcard => Expr::Column(column_name),
        Expr::IsNotNull(e) => {
            Expr::IsNotNull(Box::new(replace_wildcard_with_column(*e, column_name)))
        }
        Expr::IsNull(e) => Expr::IsNull(Box::new(replace_wildcard_with_column(*e, column_name))),
        Expr::Not(e) => Expr::Not(Box::new(replace_wildcard_with_column(*e, column_name))),
        Expr::Alias(e, name) => Expr::Alias(
            Box::new(replace_wildcard_with_column(*e, column_name)),
            name,
        ),
        Expr::Agg(agg) => match agg {
            AggExpr::Mean(e) => {
                AggExpr::Mean(Box::new(replace_wildcard_with_column(*e, column_name)))
            }
            AggExpr::Median(e) => {
                AggExpr::Median(Box::new(replace_wildcard_with_column(*e, column_name)))
            }
            AggExpr::Max(e) => {
                AggExpr::Max(Box::new(replace_wildcard_with_column(*e, column_name)))
            }
            AggExpr::Min(e) => {
                AggExpr::Min(Box::new(replace_wildcard_with_column(*e, column_name)))
            }
            AggExpr::Sum(e) => {
                AggExpr::Sum(Box::new(replace_wildcard_with_column(*e, column_name)))
            }
            AggExpr::Count(e) => {
                AggExpr::Count(Box::new(replace_wildcard_with_column(*e, column_name)))
            }
            AggExpr::Last(e) => {
                AggExpr::Last(Box::new(replace_wildcard_with_column(*e, column_name)))
            }
            AggExpr::First(e) => {
                AggExpr::First(Box::new(replace_wildcard_with_column(*e, column_name)))
            }
            AggExpr::NUnique(e) => {
                AggExpr::NUnique(Box::new(replace_wildcard_with_column(*e, column_name)))
            }
            AggExpr::AggGroups(e) => {
                AggExpr::AggGroups(Box::new(replace_wildcard_with_column(*e, column_name)))
            }
            AggExpr::Quantile { expr, quantile } => AggExpr::Quantile {
                expr: Box::new(replace_wildcard_with_column(*expr, column_name)),
                quantile,
            },
            AggExpr::List(e) => {
                AggExpr::List(Box::new(replace_wildcard_with_column(*e, column_name)))
            }
            AggExpr::Var(e) => {
                AggExpr::Var(Box::new(replace_wildcard_with_column(*e, column_name)))
            }
            AggExpr::Std(e) => {
                AggExpr::Std(Box::new(replace_wildcard_with_column(*e, column_name)))
            }
        }
        .into(),
        Expr::Shift { input, periods } => Expr::Shift {
            input: Box::new(replace_wildcard_with_column(*input, column_name)),
            periods,
        },
        Expr::Slice {
            input,
            offset,
            length,
        } => Expr::Slice {
            input: Box::new(replace_wildcard_with_column(*input, column_name)),
            offset,
            length,
        },
        Expr::Sort { expr, reverse } => Expr::Sort {
            expr: Box::new(replace_wildcard_with_column(*expr, column_name)),
            reverse,
        },
        Expr::Cast { expr, data_type } => Expr::Cast {
            expr: Box::new(replace_wildcard_with_column(*expr, column_name)),
            data_type,
        },
        Expr::Column(_) => expr,
        Expr::Literal(_) => expr,
        Expr::Except(_) => expr,
    }
}

/// In case of single col(*) -> do nothing, no selection is the same as select all
/// In other cases replace the wildcard with an expression with all columns
fn rewrite_projections(exprs: Vec<Expr>, schema: &Schema) -> Vec<Expr> {
    let mut result = Vec::with_capacity(exprs.len() + schema.fields().len());
    let mut exclude = vec![];
    for expr in exprs {
        // Columns that are excepted are later removed from the projection.
        // This can be ergonomical in combination with a wildcard expression.
        if let Expr::Except(column) = &expr {
            if let Expr::Column(name) = &**column {
                exclude.push(name.clone());
                continue;
            } else {
                panic!("Except expression should have column name")
            }
        }

        let mut has_wildcard = false;
        let roots = expr_to_root_column_exprs(&expr);
        for e in roots {
            if matches!(e, Expr::Wildcard) {
                has_wildcard = true;
                break;
            }
        }

        if has_wildcard {
            // if count wildcard. count one column
            let dummy = &Expr::Agg(AggExpr::Count(Box::new(Expr::Wildcard)));
            if has_expr(&expr, dummy) {
                let new_name = Arc::new(schema.field(0).unwrap().name().clone());
                let expr = rename_expr_root_name(&expr, new_name).unwrap();

                let expr = if let Expr::Alias(_, _) = &expr {
                    expr
                } else {
                    Expr::Alias(Box::new(expr), Arc::new("count".to_string()))
                };
                result.push(expr);

                continue;
            }

            for field in schema.fields() {
                let name = field.name();
                let new_expr = replace_wildcard_with_column(expr.clone(), Arc::new(name.clone()));
                result.push(new_expr)
            }
        } else {
            result.push(expr)
        };
    }
    if !exclude.is_empty() {
        for name in exclude {
            let idx = result
                .iter()
                .position(|expr| match expr_to_root_column_name(expr) {
                    Ok(column_name) => column_name == name,
                    Err(_) => false,
                });
            if let Some(idx) = idx {
                result.swap_remove(idx);
            }
        }
    }
    result
}

pub struct LogicalPlanBuilder(LogicalPlan);

impl LogicalPlan {
    pub(crate) fn schema(&self) -> &Schema {
        use LogicalPlan::*;
        match self {
            Cache { input } => input.schema(),
            Sort { input, .. } => input.schema(),
            Explode { input, .. } => input.schema(),
            #[cfg(feature = "parquet")]
            ParquetScan { schema, .. } => schema,
            DataFrameScan { schema, .. } => schema,
            Selection { input, .. } => input.schema(),
            CsvScan { schema, .. } => schema,
            Projection { schema, .. } => schema,
            LocalProjection { schema, .. } => schema,
            Aggregate { schema, .. } => schema,
            Join { schema, .. } => schema,
            HStack { schema, .. } => schema,
            Distinct { input, .. } => input.schema(),
            Slice { input, .. } => input.schema(),
            Melt { schema, .. } => schema,
            Udf { input, schema, .. } => match schema {
                Some(schema) => schema,
                None => input.schema(),
            },
        }
    }
    pub fn describe(&self) -> String {
        format!("{:#?}", self)
    }
}

impl From<LogicalPlan> for LogicalPlanBuilder {
    fn from(lp: LogicalPlan) -> Self {
        LogicalPlanBuilder(lp)
    }
}

pub(crate) fn prepare_projection(exprs: Vec<Expr>, schema: &Schema) -> (Vec<Expr>, Schema) {
    let exprs = rewrite_projections(exprs, schema);
    let schema = utils::expressions_to_schema(&exprs, schema, Context::Other);
    (exprs, schema)
}

impl LogicalPlanBuilder {
    #[cfg(feature = "parquet")]
    #[cfg_attr(docsrs, doc(cfg(feature = "parquet")))]
    pub fn scan_parquet(path: String, stop_after_n_rows: Option<usize>, cache: bool) -> Self {
        let file = std::fs::File::open(&path).expect("could not open file");
        let schema = Arc::new(
            ParquetReader::new(file)
                .schema()
                .expect("could not get parquet schema"),
        );

        LogicalPlan::ParquetScan {
            path,
            schema,
            stop_after_n_rows,
            with_columns: None,
            predicate: None,
            aggregate: vec![],
            cache,
        }
        .into()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn scan_csv(
        path: String,
        delimiter: u8,
        has_header: bool,
        ignore_errors: bool,
        skip_rows: usize,
        stop_after_n_rows: Option<usize>,
        cache: bool,
        schema: Option<Arc<Schema>>,
        schema_overwrite: Option<&Schema>,
    ) -> Self {
        let mut file = std::fs::File::open(&path).expect("could not open file");

        let schema = schema.unwrap_or_else(|| {
            let (schema, _) = infer_file_schema(
                &mut file,
                delimiter,
                Some(100),
                has_header,
                schema_overwrite,
            )
            .expect("could not read schema");
            Arc::new(schema)
        });
        LogicalPlan::CsvScan {
            path,
            schema,
            has_header,
            delimiter,
            ignore_errors,
            skip_rows,
            stop_after_n_rows,
            with_columns: None,
            predicate: None,
            aggregate: vec![],
            cache,
        }
        .into()
    }

    pub fn cache(self) -> Self {
        LogicalPlan::Cache {
            input: Box::new(self.0),
        }
        .into()
    }

    pub fn project(self, exprs: Vec<Expr>) -> Self {
        let (exprs, schema) = prepare_projection(exprs, &self.0.schema());

        // if len == 0, no projection has to be done. This is a select all operation.
        if !exprs.is_empty() {
            LogicalPlan::Projection {
                expr: exprs,
                input: Box::new(self.0),
                schema: Arc::new(schema),
            }
            .into()
        } else {
            self
        }
    }

    pub fn project_local(self, exprs: Vec<Expr>) -> Self {
        let (exprs, schema) = prepare_projection(exprs, &self.0.schema());
        if !exprs.is_empty() {
            LogicalPlan::LocalProjection {
                expr: exprs,
                input: Box::new(self.0),
                schema: Arc::new(schema),
            }
            .into()
        } else {
            self
        }
    }

    pub fn fill_none(self, fill_value: Expr) -> Self {
        let schema = self.0.schema();
        let exprs = schema
            .fields()
            .iter()
            .map(|field| {
                let name = field.name();
                when(col(name).is_null())
                    .then(fill_value.clone())
                    .otherwise(col(name))
                    .alias(name)
            })
            .collect();
        self.project_local(exprs)
    }

    pub fn with_columns(self, exprs: Vec<Expr>) -> Self {
        // current schema
        let schema = self.0.schema();

        let mut new_fields = schema.fields().clone();

        for e in &exprs {
            let field = e.to_field(schema, Context::Other).unwrap();
            match schema.index_of(field.name()) {
                Ok(idx) => {
                    new_fields[idx] = field;
                }
                Err(_) => new_fields.push(field),
            }
        }

        let new_schema = Schema::new(new_fields);

        LogicalPlan::HStack {
            input: Box::new(self.0),
            exprs,
            schema: Arc::new(new_schema),
        }
        .into()
    }

    /// Apply a filter
    pub fn filter(self, predicate: Expr) -> Self {
        let predicate = if has_expr(&predicate, &Expr::Wildcard) {
            let it = self.0.schema().fields().iter().map(|field| {
                replace_wildcard_with_column(predicate.clone(), Arc::new(field.name().clone()))
            });
            combine_predicates_expr(it)
        } else {
            predicate
        };
        LogicalPlan::Selection {
            predicate,
            input: Box::new(self.0),
        }
        .into()
    }

    pub fn groupby(
        self,
        keys: Arc<Vec<Expr>>,
        aggs: Vec<Expr>,
        apply: Option<Arc<dyn DataFrameUdf>>,
    ) -> Self {
        debug_assert!(!keys.is_empty());
        let current_schema = self.0.schema();
        let aggs = rewrite_projections(aggs, current_schema);

        let schema1 = utils::expressions_to_schema(&keys, current_schema, Context::Other);
        let schema2 = utils::expressions_to_schema(&aggs, current_schema, Context::Aggregation);
        let schema = Schema::try_merge(&[schema1, schema2]).unwrap();

        LogicalPlan::Aggregate {
            input: Box::new(self.0),
            keys,
            aggs,
            schema: Arc::new(schema),
            apply,
        }
        .into()
    }

    pub fn build(self) -> LogicalPlan {
        self.0
    }

    pub fn from_existing_df(df: DataFrame) -> Self {
        let schema = Arc::new(df.schema());
        LogicalPlan::DataFrameScan {
            df: Arc::new(df),
            schema,
            projection: None,
            selection: None,
        }
        .into()
    }

    pub fn sort(self, by_column: String, reverse: bool) -> Self {
        LogicalPlan::Sort {
            input: Box::new(self.0),
            by_column,
            reverse,
        }
        .into()
    }

    pub fn explode(self, columns: Vec<String>) -> Self {
        LogicalPlan::Explode {
            input: Box::new(self.0),
            columns,
        }
        .into()
    }

    pub fn melt(self, id_vars: Arc<Vec<String>>, value_vars: Arc<Vec<String>>) -> Self {
        let schema = det_melt_schema(&value_vars, self.0.schema());
        LogicalPlan::Melt {
            input: Box::new(self.0),
            id_vars,
            value_vars,
            schema,
        }
        .into()
    }

    pub fn drop_duplicates(self, maintain_order: bool, subset: Option<Vec<String>>) -> Self {
        LogicalPlan::Distinct {
            input: Box::new(self.0),
            maintain_order,
            subset: Arc::new(subset),
        }
        .into()
    }

    pub fn slice(self, offset: usize, len: usize) -> Self {
        LogicalPlan::Slice {
            input: Box::new(self.0),
            offset,
            len,
        }
        .into()
    }

    pub fn join(
        self,
        other: LogicalPlan,
        how: JoinType,
        left_on: Vec<Expr>,
        right_on: Vec<Expr>,
        allow_par: bool,
        force_par: bool,
    ) -> Self {
        let schema_left = self.0.schema();
        let schema_right = other.schema();

        // column names of left table
        let mut names: HashSet<&String, RandomState> = HashSet::default();
        // fields of new schema
        let mut fields = vec![];

        for f in schema_left.fields() {
            names.insert(f.name());
            fields.push(f.clone());
        }

        let right_names: HashSet<_, RandomState> = right_on
            .iter()
            .map(|e| utils::output_name(e).expect("could not find name"))
            .collect();

        for f in schema_right.fields() {
            let name = f.name();

            if !right_names.contains(name) {
                if names.contains(name) {
                    let new_name = format!("{}_right", name);
                    let field = Field::new(&new_name, f.data_type().clone());
                    fields.push(field)
                } else {
                    fields.push(f.clone())
                }
            }
        }

        let schema = Arc::new(Schema::new(fields));

        LogicalPlan::Join {
            input_left: Box::new(self.0),
            input_right: Box::new(other),
            how,
            schema,
            left_on,
            right_on,
            allow_par,
            force_par,
        }
        .into()
    }
    pub fn map<F>(
        self,
        function: F,
        optimizations: AllowedOptimizations,
        schema: Option<SchemaRef>,
    ) -> Self
    where
        F: DataFrameUdf + 'static,
    {
        LogicalPlan::Udf {
            input: Box::new(self.0),
            function: Arc::new(function),
            predicate_pd: optimizations.predicate_pushdown,
            projection_pd: optimizations.projection_pushdown,
            schema,
        }
        .into()
    }
}

pub(crate) fn det_melt_schema(value_vars: &[String], input_schema: &Schema) -> SchemaRef {
    let mut fields = input_schema
        .fields()
        .iter()
        .filter(|field| !value_vars.contains(field.name()))
        .cloned()
        .collect_vec();

    fields.reserve(2);

    let value_dtype = input_schema
        .field_with_name(&value_vars[0])
        .expect("field not found")
        .data_type();

    fields.push(Field::new("variable", DataType::Utf8));
    fields.push(Field::new("value", value_dtype.clone()));

    Arc::new(Schema::new(fields))
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use crate::tests::get_df;
    use polars_core::df;
    use polars_core::prelude::*;

    fn print_plans(lf: &LazyFrame) {
        println!("LOGICAL PLAN\n\n{}\n", lf.describe_plan());
        println!(
            "OPTIMIZED LOGICAL PLAN\n\n{}\n",
            lf.describe_optimized_plan().unwrap()
        );
    }

    #[test]
    fn test_lazy_arithmetic() {
        let df = get_df();
        let lf = df
            .lazy()
            .select(&[((col("sepal.width") * lit(100)).alias("super_wide"))])
            .sort("super_wide", false);

        print_plans(&lf);

        let new = lf.collect().unwrap();
        println!("{:?}", new);
        assert_eq!(new.height(), 7);
        assert_eq!(
            new.column("super_wide").unwrap().f64().unwrap().get(0),
            Some(300.0)
        );
    }

    #[test]
    fn test_lazy_logical_plan_filter_and_alias_combined() {
        let df = get_df();
        let lf = df
            .lazy()
            .filter(col("sepal.width").lt(lit(3.5)))
            .select(&[col("variety").alias("foo")]);

        print_plans(&lf);
        let df = lf.collect().unwrap();
        println!("{:?}", df);
    }

    #[test]
    fn test_lazy_logical_plan_schema() {
        let df = get_df();
        let lp = df
            .clone()
            .lazy()
            .select(&[col("variety").alias("foo")])
            .logical_plan;

        println!("{:#?}", lp.schema().fields());
        assert!(lp.schema().field_with_name("foo").is_ok());

        let lp = df
            .lazy()
            .groupby(vec![col("variety")])
            .agg(vec![col("sepal.width").min()])
            .logical_plan;
        println!("{:#?}", lp.schema().fields());
        assert!(lp.schema().field_with_name("sepal.width_min").is_ok());
    }

    #[test]
    fn test_lazy_logical_plan_join() {
        let left = df!("days" => &[0, 1, 2, 3, 4],
        "temp" => [22.1, 19.9, 7., 2., 3.],
        "rain" => &[0.1, 0.2, 0.3, 0.4, 0.5]
        )
        .unwrap();

        let right = df!(
        "days" => &[1, 2],
        "rain" => &[0.1, 0.2]
        )
        .unwrap();

        // check if optimizations succeeds without selection
        {
            let lf =
                left.clone()
                    .lazy()
                    .left_join(right.clone().lazy(), col("days"), col("days"), None);

            print_plans(&lf);
            // implicitly checks logical plan == optimized logical plan
            let df = lf.collect().unwrap();
            println!("{:?}", df);
        }

        // check if optimization succeeds with selection
        {
            let lf = left
                .clone()
                .lazy()
                .left_join(right.clone().lazy(), col("days"), col("days"), None)
                .select(&[col("temp")]);

            print_plans(&lf);
            let df = lf.collect().unwrap();
            println!("{:?}", df);
        }

        // check if optimization succeeds with selection of a renamed column due to the join
        {
            let lf = left
                .clone()
                .lazy()
                .left_join(right.clone().lazy(), col("days"), col("days"), None)
                .select(&[col("temp"), col("rain_right")]);

            print_plans(&lf);
            let df = lf.collect().unwrap();
            println!("{:?}", df);
        }

        // check if optimization succeeds with selection of the left and the right (renamed)
        // column due to the join
        {
            let lf = left
                .clone()
                .lazy()
                .left_join(right.clone().lazy(), col("days"), col("days"), None)
                .select(&[col("temp"), col("rain"), col("rain_right")]);

            print_plans(&lf);
            let df = lf.collect().unwrap();
            println!("{:?}", df);
        }

        // check if optimization succeeds with selection of the left and the right (renamed)
        // column due to the join and an extra alias
        {
            let lf = left
                .clone()
                .lazy()
                .left_join(right.clone().lazy(), col("days"), col("days"), None)
                .select(&[col("temp"), col("rain").alias("foo"), col("rain_right")]);

            print_plans(&lf);
            let df = lf.collect().unwrap();
            println!("{:?}", df);
        }

        // check if optimization succeeds with selection of the left and the right (renamed)
        // column due to the join and an extra alias
        {
            let lf = left
                .lazy()
                .left_join(right.lazy(), col("days"), col("days"), None)
                .select(&[col("temp"), col("rain").alias("foo"), col("rain_right")])
                .filter(col("foo").lt(lit(0.3)));

            print_plans(&lf);
            let df = lf.collect().unwrap();
            println!("{:?}", df);
        }
    }

    #[test]
    fn test_dot() {
        let left = df!("days" => &[0, 1, 2, 3, 4],
        "temp" => [22.1, 19.9, 7., 2., 3.],
        "rain" => &[0.1, 0.2, 0.3, 0.4, 0.5]
        )
        .unwrap();
        let mut s = String::new();
        left.lazy()
            .select(&[col("days")])
            .logical_plan
            .dot(&mut s, 0, "")
            .unwrap();
        println!("{}", s);
    }
}
