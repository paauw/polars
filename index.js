URLS=[
"pypolars/index.html",
"pypolars/series.html",
"pypolars/frame.html",
"pypolars/datatypes.html",
"pypolars/functions.html",
"pypolars/html.html",
"pypolars/lazy/index.html",
"pypolars/lazy/agg.html"
];
INDEX=[
{
"ref":"pypolars",
"url":0,
"doc":""
},
{
"ref":"pypolars.series",
"url":1,
"doc":""
},
{
"ref":"pypolars.series.IdentityDict",
"url":1,
"doc":"dict() -> new empty dictionary dict(mapping) -> new dictionary initialized from a mapping object's (key, value) pairs dict(iterable) -> new dictionary initialized as if via: d = {} for k, v in iterable: d[k] = v dict( kwargs) -> new dictionary initialized with the name=value pairs in the keyword argument list. For example: dict(one=1, two=2)"
},
{
"ref":"pypolars.series.Series",
"url":1,
"doc":"Parameters      name Name of the series values Values of the series nullable If nullable. None values in a list will be interpreted as missing. NaN values in a numpy array will be interpreted as missing. Note that missing and NaNs are not the same in Polars Series creation may be faster if set to False and there are no null values."
},
{
"ref":"pypolars.series.Series.from_arrow",
"url":1,
"doc":"Create a Series from an arrow array. Parameters      name name of the Series. array Arrow array.",
"func":1
},
{
"ref":"pypolars.series.Series.inner",
"url":1,
"doc":"",
"func":1
},
{
"ref":"pypolars.series.Series.drop_nulls",
"url":1,
"doc":"",
"func":1
},
{
"ref":"pypolars.series.Series.dtype",
"url":1,
"doc":"Get the data type of this Series"
},
{
"ref":"pypolars.series.Series.sum",
"url":1,
"doc":"Reduce this Series to the sum value.",
"func":1
},
{
"ref":"pypolars.series.Series.mean",
"url":1,
"doc":"Reduce this Series to the mean value.",
"func":1
},
{
"ref":"pypolars.series.Series.min",
"url":1,
"doc":"Get the minimal value in this Series",
"func":1
},
{
"ref":"pypolars.series.Series.max",
"url":1,
"doc":"Get the maximum value in this Series",
"func":1
},
{
"ref":"pypolars.series.Series.std",
"url":1,
"doc":"Get standard deviation of this Series",
"func":1
},
{
"ref":"pypolars.series.Series.var",
"url":1,
"doc":"Get variance of this Series",
"func":1
},
{
"ref":"pypolars.series.Series.to_dummies",
"url":1,
"doc":"Get dummy variables",
"func":1
},
{
"ref":"pypolars.series.Series.value_counts",
"url":1,
"doc":"Count the unique values in a Series",
"func":1
},
{
"ref":"pypolars.series.Series.name",
"url":1,
"doc":"Get the name of this Series"
},
{
"ref":"pypolars.series.Series.rename",
"url":1,
"doc":"Rename this Series. Parameters      name New name",
"func":1
},
{
"ref":"pypolars.series.Series.chunk_lengths",
"url":1,
"doc":"",
"func":1
},
{
"ref":"pypolars.series.Series.n_chunks",
"url":1,
"doc":"Get the number of chunks that this Series contains.",
"func":1
},
{
"ref":"pypolars.series.Series.cum_sum",
"url":1,
"doc":"Get an array with the cumulative sum computed at every element Parameters      reverse reverse the operation",
"func":1
},
{
"ref":"pypolars.series.Series.cum_min",
"url":1,
"doc":"Get an array with the cumulative min computed at every element Parameters      reverse reverse the operation",
"func":1
},
{
"ref":"pypolars.series.Series.cum_max",
"url":1,
"doc":"Get an array with the cumulative max computed at every element Parameters      reverse reverse the operation",
"func":1
},
{
"ref":"pypolars.series.Series.limit",
"url":1,
"doc":"Take n elements from this Series. Parameters      num_elements Amount of elements to take.",
"func":1
},
{
"ref":"pypolars.series.Series.slice",
"url":1,
"doc":"Get a slice of this Series Parameters      offset Offset index. length Length of the slice.",
"func":1
},
{
"ref":"pypolars.series.Series.append",
"url":1,
"doc":"Append a Series to this one. Parameters      other Series to append",
"func":1
},
{
"ref":"pypolars.series.Series.filter",
"url":1,
"doc":"Filter elements by a boolean mask Parameters      filter Boolean mask",
"func":1
},
{
"ref":"pypolars.series.Series.head",
"url":1,
"doc":"Get first N elements as Series Parameters      length Length of the head",
"func":1
},
{
"ref":"pypolars.series.Series.tail",
"url":1,
"doc":"Get last N elements as Series Parameters      length Length of the tail",
"func":1
},
{
"ref":"pypolars.series.Series.take_every",
"url":1,
"doc":"Take every nth value in the Series and return as new Series.",
"func":1
},
{
"ref":"pypolars.series.Series.sort",
"url":1,
"doc":"Sort this Series. Parameters      in_place Sort in place. reverse Reverse sort",
"func":1
},
{
"ref":"pypolars.series.Series.argsort",
"url":1,
"doc":"Index location of the sorted variant of this Series. Returns    - indexes Indexes that can be used to sort this array.",
"func":1
},
{
"ref":"pypolars.series.Series.arg_unique",
"url":1,
"doc":"Get unique arguments. Returns    - indexes Indexes of the unique values",
"func":1
},
{
"ref":"pypolars.series.Series.unique",
"url":1,
"doc":"Get unique elements in series.",
"func":1
},
{
"ref":"pypolars.series.Series.take",
"url":1,
"doc":"Take values by index. Parameters      indices Index location used for selection.",
"func":1
},
{
"ref":"pypolars.series.Series.null_count",
"url":1,
"doc":"Count the null values in this Series",
"func":1
},
{
"ref":"pypolars.series.Series.is_null",
"url":1,
"doc":"Get mask of null values Returns    - Boolean Series",
"func":1
},
{
"ref":"pypolars.series.Series.is_not_null",
"url":1,
"doc":"Get mask of non null values Returns    - Boolean Series",
"func":1
},
{
"ref":"pypolars.series.Series.is_finite",
"url":1,
"doc":"Get mask of finite values if Series dtype is Float Returns    - Boolean Series",
"func":1
},
{
"ref":"pypolars.series.Series.is_infinite",
"url":1,
"doc":"Get mask of infinite values if Series dtype is Float Returns    - Boolean Series",
"func":1
},
{
"ref":"pypolars.series.Series.is_nan",
"url":1,
"doc":"Get mask of NaN values if Series dtype is Float Returns    - Boolean Series",
"func":1
},
{
"ref":"pypolars.series.Series.is_not_nan",
"url":1,
"doc":"Get negated mask of NaN values if Series dtype is_not Float Returns    - Boolean Series",
"func":1
},
{
"ref":"pypolars.series.Series.arg_true",
"url":1,
"doc":"Get index values where Boolean Series evaluate True Returns    - UInt32 Series",
"func":1
},
{
"ref":"pypolars.series.Series.is_unique",
"url":1,
"doc":"Get mask of all unique values Returns    - Boolean Series",
"func":1
},
{
"ref":"pypolars.series.Series.is_duplicated",
"url":1,
"doc":"Get mask of all duplicated values Returns    - Boolean Series",
"func":1
},
{
"ref":"pypolars.series.Series.explode",
"url":1,
"doc":"Explode a list or utf8 Series. This means that every item is expanded to a new row. Returns    - Exploded Series of same dtype",
"func":1
},
{
"ref":"pypolars.series.Series.series_equal",
"url":1,
"doc":"Check if series equal with another Series. Parameters      other Series to compare with. null_equal Consider null values as equal.",
"func":1
},
{
"ref":"pypolars.series.Series.len",
"url":1,
"doc":"Length of this Series",
"func":1
},
{
"ref":"pypolars.series.Series.cast",
"url":1,
"doc":"",
"func":1
},
{
"ref":"pypolars.series.Series.to_list",
"url":1,
"doc":"Convert this Series to a Python List. This operation clones data.",
"func":1
},
{
"ref":"pypolars.series.Series.rechunk",
"url":1,
"doc":"Create a single chunk of memory for this Series. Parameters      in_place In place or not.",
"func":1
},
{
"ref":"pypolars.series.Series.is_numeric",
"url":1,
"doc":"Check if this Series datatype is numeric.",
"func":1
},
{
"ref":"pypolars.series.Series.is_float",
"url":1,
"doc":"Check if this Series has floating point numbers",
"func":1
},
{
"ref":"pypolars.series.Series.view",
"url":1,
"doc":"Get a view into this Series data with a numpy array. This operation doesn't clone data, but does not include missing values. Don't use this unless you know what you are doing.  Safety. This function can lead to undefined behavior in the following cases:    returns a view to a piece of memory that is already dropped. pl.Series([1, 3, 5]).sort().view()  Sums invalid data that is missing. pl.Series([1, 2, None], nullable=True).view().sum()  ",
"func":1
},
{
"ref":"pypolars.series.Series.to_numpy",
"url":1,
"doc":"Convert this Series to numpy. This operation clones data but is completely safe. If you want a zero-copy view and know what you are doing, use  .view() . Parameters      args args will be sent to pyarrow.Array.to_numpy zero_copy_only If True, an exception will be raised if the conversion to a numpy array would require copying the underlying data (e.g. in presence of nulls, or for non-primitive types). kwargs kwargs will be sent to pyarrow.Array.to_numpy",
"func":1
},
{
"ref":"pypolars.series.Series.to_arrow",
"url":1,
"doc":"Get the underlying arrow array. If the Series contains only a single chunk this operation is zero copy.",
"func":1
},
{
"ref":"pypolars.series.Series.set",
"url":1,
"doc":"Set masked values. Parameters      filter Boolean mask value Value to replace the the masked values with.",
"func":1
},
{
"ref":"pypolars.series.Series.set_at_idx",
"url":1,
"doc":"Set values at the index locations. Parameters      idx Integers representing the index locations. value replacement values Returns    - New allocated Series",
"func":1
},
{
"ref":"pypolars.series.Series.clone",
"url":1,
"doc":"Cheap deep clones",
"func":1
},
{
"ref":"pypolars.series.Series.fill_none",
"url":1,
"doc":"Fill null values with a fill strategy. Parameters      strategy  \"backward\"  \"forward\"  \"min\"  \"max\"  \"mean\"",
"func":1
},
{
"ref":"pypolars.series.Series.apply",
"url":1,
"doc":"Apply a function over elements in this Series and return a new Series. If the function returns another datatype, the dtype_out arg should be set, otherwise the method will fail. Parameters      func function or lambda. dtype_out Output datatype. If none given the same datatype as this Series will be used. Returns    - Series",
"func":1
},
{
"ref":"pypolars.series.Series.shift",
"url":1,
"doc":"Shift the values by a given period and fill the parts that will be empty due to this operation with  Nones . Parameters      periods Number of places to shift (may be negative).",
"func":1
},
{
"ref":"pypolars.series.Series.zip_with",
"url":1,
"doc":"Where mask evaluates true take values from self. Where mask evaluates false, take values from other. Parameters      mask Boolean Series other Series of same type Returns    - New Series",
"func":1
},
{
"ref":"pypolars.series.Series.str_lengths",
"url":1,
"doc":"Get length of the string values in the Series. Returns    - Series[u32]",
"func":1
},
{
"ref":"pypolars.series.Series.str_contains",
"url":1,
"doc":"Check if strings in Series contain regex pattern Parameters      pattern A valid regex pattern Returns    - Boolean mask",
"func":1
},
{
"ref":"pypolars.series.Series.str_replace",
"url":1,
"doc":"Replace first regex math with a string value Parameters      pattern A valid regex pattern value Substring to replace",
"func":1
},
{
"ref":"pypolars.series.Series.str_replace_all",
"url":1,
"doc":"Replace all regex matches with a string value Parameters      pattern A valid regex pattern value Substring to replace",
"func":1
},
{
"ref":"pypolars.series.Series.str_to_lowercase",
"url":1,
"doc":"Modify the strings to their lowercase equivalent",
"func":1
},
{
"ref":"pypolars.series.Series.str_to_uppercase",
"url":1,
"doc":"Modify the strings to their uppercase equivalent",
"func":1
},
{
"ref":"pypolars.series.Series.str_rstrip",
"url":1,
"doc":"Remove trailing whitespace",
"func":1
},
{
"ref":"pypolars.series.Series.str_lstrip",
"url":1,
"doc":"Remove leading whitespace",
"func":1
},
{
"ref":"pypolars.series.Series.as_duration",
"url":1,
"doc":"If Series is a date32 or a date64 it can be turned into a duration.",
"func":1
},
{
"ref":"pypolars.series.Series.str_parse_date",
"url":1,
"doc":"Parse a Series of dtype Utf8 to a Date32/Date64 Series. Parameters      datatype polars.Date32 or polars.Date64 fmt formatting syntax. [Read more](https: docs.rs/chrono/0.4.19/chrono/format/strftime/index.html) Returns    -",
"func":1
},
{
"ref":"pypolars.series.Series.rolling_min",
"url":1,
"doc":"apply a rolling min (moving min) over the values in this array. a window of length  window_size will traverse the array. the values that fill this window will (optionally) be multiplied with the weights given by the  weight vector. the resultingParameters values will be aggregated to their sum.      window_size The length of the window weight An optional slice with the same length of the window that will be multiplied elementwise with the values in the window. ignore_null Toggle behavior of aggregation regarding null values in the window.  True -> Null values will be ignored.  False -> Any Null in the window leads to a Null in the aggregation result.",
"func":1
},
{
"ref":"pypolars.series.Series.rolling_max",
"url":1,
"doc":"apply a rolling max (moving max) over the values in this array. a window of length  window_size will traverse the array. the values that fill this window will (optionally) be multiplied with the weights given by the  weight vector. the resultingParameters values will be aggregated to their sum.      window_size The length of the window weight An optional slice with the same length of the window that will be multiplied elementwise with the values in the window. ignore_null Toggle behavior of aggregation regarding null values in the window.  True -> Null values will be ignored.  False -> Any Null in the window leads to a Null in the aggregation result.",
"func":1
},
{
"ref":"pypolars.series.Series.rolling_mean",
"url":1,
"doc":"apply a rolling mean (moving mean) over the values in this array. a window of length  window_size will traverse the array. the values that fill this window will (optionally) be multiplied with the weights given by the  weight vector. the resultingParameters values will be aggregated to their sum.      window_size The length of the window weight An optional slice with the same length of the window that will be multiplied elementwise with the values in the window. ignore_null Toggle behavior of aggregation regarding null values in the window.  True -> Null values will be ignored.  False -> Any Null in the window leads to a Null in the aggregation result.",
"func":1
},
{
"ref":"pypolars.series.Series.rolling_sum",
"url":1,
"doc":"apply a rolling sum (moving sum) over the values in this array. a window of length  window_size will traverse the array. the values that fill this window will (optionally) be multiplied with the weights given by the  weight vector. the resultingParameters values will be aggregated to their sum.      window_size The length of the window weight An optional slice with the same length of the window that will be multiplied elementwise with the values in the window. ignore_null Toggle behavior of aggregation regarding null values in the window.  True -> Null values will be ignored.  False -> Any Null in the window leads to a Null in the aggregation result.",
"func":1
},
{
"ref":"pypolars.series.Series.year",
"url":1,
"doc":"Extract year from underlying Date representation. Can be performed on Date32 and Date64 Returns the year number in the calendar date. Returns    - Year as Int32",
"func":1
},
{
"ref":"pypolars.series.Series.month",
"url":1,
"doc":"Extract month from underlying Date representation. Can be performed on Date32 and Date64 Returns the month number starting from 1. The return value ranges from 1 to 12. Returns    - Month as UInt32",
"func":1
},
{
"ref":"pypolars.series.Series.day",
"url":1,
"doc":"Extract day from underlying Date representation. Can be performed on Date32 and Date64 Returns the day of month starting from 1. The return value ranges from 1 to 31. (The last day of month differs by months.) Returns    - Day as UInt32",
"func":1
},
{
"ref":"pypolars.series.Series.ordinal_day",
"url":1,
"doc":"Extract ordinal day from underlying Date representation. Can be performed on Date32 and Date64 Returns the day of year starting from 1. The return value ranges from 1 to 366. (The last day of year differs by years.) Returns    - Day as UInt32",
"func":1
},
{
"ref":"pypolars.series.Series.hour",
"url":1,
"doc":"Extract day from underlying DateTime representation. Can be performed on Date64 Returns the hour number from 0 to 23. Returns    - Hour as UInt32",
"func":1
},
{
"ref":"pypolars.series.Series.minute",
"url":1,
"doc":"Extract minutes from underlying DateTime representation. Can be performed on Date64 Returns the minute number from 0 to 59. Returns    - Minute as UInt32",
"func":1
},
{
"ref":"pypolars.series.Series.second",
"url":1,
"doc":"Extract seconds from underlying DateTime representation. Can be performed on Date64 Returns the second number from 0 to 59. Returns    - Second as UInt32",
"func":1
},
{
"ref":"pypolars.series.Series.nanosecond",
"url":1,
"doc":"Extract seconds from underlying DateTime representation. Can be performed on Date64 Returns the number of nanoseconds since the whole non-leap second. The range from 1,000,000,000 to 1,999,999,999 represents the leap second. Returns    - Nanosecond as UInt32",
"func":1
},
{
"ref":"pypolars.series.Series.datetime_str_fmt",
"url":1,
"doc":"Format date32/date64 with a formatting rule: See [chrono strftime/strptime](https: docs.rs/chrono/0.4.19/chrono/format/strftime/index.html). Returns    - Utf8 Series",
"func":1
},
{
"ref":"pypolars.series.Series.parse_date",
"url":1,
"doc":"Deprecated.",
"func":1
},
{
"ref":"pypolars.series.Series.sample",
"url":1,
"doc":"Sample from this Series by setting either  n or  frac Parameters      n Number of samples < self.len() frac Fraction between 0.0 and 1.0 with_replacement sample with replacement",
"func":1
},
{
"ref":"pypolars.series.Series.peak_max",
"url":1,
"doc":"Get a boolean mask of the local maximum peaks.",
"func":1
},
{
"ref":"pypolars.series.Series.peak_min",
"url":1,
"doc":"Get a boolean mask of the local minimum peaks.",
"func":1
},
{
"ref":"pypolars.frame",
"url":2,
"doc":""
},
{
"ref":"pypolars.frame.prepare_other",
"url":2,
"doc":"",
"func":1
},
{
"ref":"pypolars.frame.DataFrame",
"url":2,
"doc":"A DataFrame is a two dimensional data structure that represents data as a table with rows and columns."
},
{
"ref":"pypolars.frame.DataFrame.read_csv",
"url":2,
"doc":"Read a CSV file into a Dataframe. Parameters  - file Path to a file or a file like object. Any valid filepath can be used. Example:  file.csv . sep Character to use as delimiter in the file. stop_after_n_rows Only read specified number of rows of the dataset. After  n stops reading. has_headers Indicate if first row of dataset is header or not. If set to False first row will be set to  column_x ,  x being an enumeration over every column in the dataset. encoding Allowed encodings:  utf8 ,  utf8-lossy . Lossy means that invalid utf8 values are replaced with  \ufffd character. Example  -   dataframe = pl.read_csv('file.csv', sep=';', stop_after_n_rows=25)   Returns  - DataFrame",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.read_parquet",
"url":2,
"doc":"Read into a DataFrame from a parquet file. Parameters  - file Path to a file or a file like object. Any valid filepath can be used. stop_after_n_rows Only read specified number of rows of the dataset. After  n stops reading. Returns  - DataFrame",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.read_ipc",
"url":2,
"doc":"Read into a DataFrame from Arrow IPC stream format. This is also called the feather format. Parameters      file Path to a file or a file like object. Returns    - DataFrame",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.from_arrow",
"url":2,
"doc":"Create DataFrame from arrow Table Parameters      table Arrow Table rechunk Make sure that all data is contiguous.",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.to_arrow",
"url":2,
"doc":"Collect the underlying arrow arrays in an Arrow Table. This operation is zero copy.",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.to_pandas",
"url":2,
"doc":"Cast to a Pandas DataFrame. This requires that Pandas is installed. This operation clones data. Parameters      args arguments will be sent to pyarrow.Table.to_pandas date_as_object Cast dates to objects. If False, convert to datetime64[ns] dtype kwargs arguments will be sent to pyarrow.Table.to_pandas Example  -   >>> import pandas >>> dataframe = pl.DataFrame({ \"foo\": [1, 2, 3], \"bar\": [6, 7, 8], \"ham\": ['a', 'b', 'c'] }) >>> pandas_df = dataframe.to_pandas() >>> type(pandas_df) pandas.core.frame.DataFrame  ",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.to_csv",
"url":2,
"doc":"Write Dataframe to comma-seperated values file (csv) Parameters  - file File path to which the file should be written. batch_size Size of the write buffer. Increase to have faster io. has_headers Whether or not to include header in the CSV output. delimiter Separate CSV fields with this symbol. Example  -   >>> dataframe = pl.DataFrame({ \"foo\": [1, 2, 3, 4, 5], \"bar\": [6, 7, 8, 9, 10], \"ham\": ['a', 'b', 'c', 'd','e'] }) >>> dataframe.to_csv('new_file.csv', sep=',')  ",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.to_ipc",
"url":2,
"doc":"Write to Arrow IPC binary stream, or a feather file. Parameters      file File path to which the file should be written.",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.to_parquet",
"url":2,
"doc":"Write the DataFrame disk in parquet format. Parameters      file File path to which the file should be written. compression Compression method (only supported if  use_pyarrow ) use_pyarrow Use C parquet implementation vs rust parquet implementation. At the moment C supports more features  kwargs are passed to pyarrow.parquet.write_table",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.to_numpy",
"url":2,
"doc":"Convert DataFrame to a 2d numpy array. This operation clones data. Example  -   >>> import pandas >>> dataframe = pl.DataFrame({ \"foo\": [1, 2, 3], \"bar\": [6, 7, 8], \"ham\": ['a', 'b', 'c'] }) >>> numpy_array = dataframe.to_numpy() >>> type(numpy_array) numpy.ndarray  ",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.insert_at_idx",
"url":2,
"doc":"",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.shape",
"url":2,
"doc":"Get shape of the DataFrame. Example  -   >>> dataframe = pl.DataFrame({\"foo\": [1, 2, 3, 4, 5]}) >>> dataframe.shape shape: (5, 1)  "
},
{
"ref":"pypolars.frame.DataFrame.height",
"url":2,
"doc":"Get height of the DataFrame. Example  -   >>> dataframe = pl.DataFrame({\"foo\": [1, 2, 3, 4, 5]}) >>> dataframe.height 5  "
},
{
"ref":"pypolars.frame.DataFrame.width",
"url":2,
"doc":"Get width of the DataFrame Example  -   >>> dataframe = pl.DataFrame({\"foo\": [1, 2, 3, 4, 5]}) >>> dataframe.width 1  "
},
{
"ref":"pypolars.frame.DataFrame.columns",
"url":2,
"doc":"Get or set column names Example  -   >>> dataframe = pl.DataFrame({ \"foo\": [1, 2, 3], \"bar\": [6, 7, 8], \"ham\": ['a', 'b', 'c'] }) >>> dataframe.columns ['foo', 'bar', 'ham']  Set column names >>> dataframe.columns = ['apple', 'banana', 'orange'] shape: (3, 3) \u256d   \u2500\u252c    \u252c    \u256e \u2502 apple \u2506 banana \u2506 orange \u2502 \u2502  - \u2506  - \u2506  - \u2502 \u2502 i64 \u2506 i64 \u2506 str \u2502 \u255e   \u2550\u256a    \u256a    \u2561 \u2502 1 \u2506 6 \u2506 \"a\" \u2502 \u251c   \u254c\u253c    \u253c    \u2524 \u2502 2 \u2506 7 \u2506 \"b\" \u2502 \u251c   \u254c\u253c    \u253c    \u2524 \u2502 3 \u2506 8 \u2506 \"c\" \u2502 \u2570   \u2500\u2534    \u2534    \u256f  "
},
{
"ref":"pypolars.frame.DataFrame.dtypes",
"url":2,
"doc":"Get dtypes of columns in DataFrame. Dtypes can also be found in column headers when printing the DataFrame. Example  -   >>> dataframe = pl.DataFrame({ \"foo\": [1, 2, 3], \"bar\": [6.0, 7.0, 8.0], \"ham\": ['a', 'b', 'c'] }) >>> dataframe.dtypes [pypolars.datatypes.Int64, pypolars.datatypes.Float64, pypolars.datatypes.Utf8] >>> dataframe shape: (3, 3) \u256d  \u2500\u252c  \u2500\u252c  \u2500\u256e \u2502 foo \u2506 bar \u2506 ham \u2502 \u2502  - \u2506  - \u2506  - \u2502 \u2502 i64 \u2506 f64 \u2506 str \u2502 \u255e  \u2550\u256a  \u2550\u256a  \u2550\u2561 \u2502 1 \u2506 6 \u2506 \"a\" \u2502 \u251c  \u254c\u253c  \u254c\u253c  \u254c\u2524 \u2502 2 \u2506 7 \u2506 \"b\" \u2502 \u251c  \u254c\u253c  \u254c\u253c  \u254c\u2524 \u2502 3 \u2506 8 \u2506 \"c\" \u2502 \u2570  \u2500\u2534  \u2500\u2534  \u2500\u256f  "
},
{
"ref":"pypolars.frame.DataFrame.replace_at_idx",
"url":2,
"doc":"Replace a column at an index location. Parameters      index Column index series Series that will replace the column",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.sort",
"url":2,
"doc":"Sort the DataFrame by column Parameters      by_column By which column to sort. Only accepts string. in_place Perform operation in-place. reverse Reverse/descending sort. Example  -   >>> pl.DataFrame({ \"foo\": [1, 2, 3], \"bar\": [6.0, 7.0, 8.0], \"ham\": ['a', 'b', 'c'] }) >>> dataframe.sort('foo', reverse=True) shape: (3, 3) \u256d  \u2500\u252c  \u2500\u252c  \u2500\u256e \u2502 foo \u2506 bar \u2506 ham \u2502 \u2502  - \u2506  - \u2506  - \u2502 \u2502 i64 \u2506 f64 \u2506 str \u2502 \u255e  \u2550\u256a  \u2550\u256a  \u2550\u2561 \u2502 3 \u2506 8 \u2506 \"c\" \u2502 \u251c  \u254c\u253c  \u254c\u253c  \u254c\u2524 \u2502 2 \u2506 7 \u2506 \"b\" \u2502 \u251c  \u254c\u253c  \u254c\u253c  \u254c\u2524 \u2502 1 \u2506 6 \u2506 \"a\" \u2502 \u2570  \u2500\u2534  \u2500\u2534  \u2500\u256f  ",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.frame_equal",
"url":2,
"doc":"Check if DataFrame is equal to other. Parameters      other DataFrame to compare with. null_equal Consider null values as equal. Example  -   >>> df1 = pl.DataFrame({ \"foo\": [1, 2, 3], \"bar\": [6.0, 7.0, 8.0], \"ham\": ['a', 'b', 'c'] }) >>> df2 = pl.DataFrame({ \"foo\": [3, 2, 1], \"bar\": [8.0, 7.0, 6.0], \"ham\": ['c', 'b', 'a'] }) >>> df1.frame_equal(df1) True >>> df1.frame_equal(df2) False  ",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.replace",
"url":2,
"doc":"Replace a column by a new Series. Parameters      column Column to replace. new_col New column to insert.",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.slice",
"url":2,
"doc":"Slice this DataFrame over the rows direction. Parameters      offset Offset index. length Length of the slice.",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.head",
"url":2,
"doc":"Get first N rows as DataFrame Parameters      length Length of the head Example  -   >>> dataframe = pl.DataFrame({ \"foo\": [1, 2, 3, 4, 5], \"bar\": [6, 7, 8, 9, 10], \"ham\": ['a', 'b', 'c', 'd','e'] }) >>> dataframe.head(3) shape: (3, 3) \u256d  \u2500\u252c  \u2500\u252c  \u2500\u256e \u2502 foo \u2506 bar \u2506 ham \u2502 \u2502  - \u2506  - \u2506  - \u2502 \u2502 i64 \u2506 i64 \u2506 str \u2502 \u255e  \u2550\u256a  \u2550\u256a  \u2550\u2561 \u2502 1 \u2506 6 \u2506 \"a\" \u2502 \u251c  \u254c\u253c  \u254c\u253c  \u254c\u2524 \u2502 2 \u2506 7 \u2506 \"b\" \u2502 \u251c  \u254c\u253c  \u254c\u253c  \u254c\u2524 \u2502 3 \u2506 8 \u2506 \"c\" \u2502 \u2570  \u2500\u2534  \u2500\u2534  \u2500\u256f  ",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.tail",
"url":2,
"doc":"Get last N rows as DataFrame Parameters      length Length of the tail Example  -   >>> dataframe = pl.DataFrame({ \"foo\": [1, 2, 3, 4, 5], \"bar\": [6, 7, 8, 9, 10], \"ham\": ['a', 'b', 'c', 'd','e'] }) >>> dataframe.tail(3) shape: (3, 3) \u256d  \u2500\u252c  \u2500\u252c  \u2500\u256e \u2502 foo \u2506 bar \u2506 ham \u2502 \u2502  - \u2506  - \u2506  - \u2502 \u2502 i64 \u2506 i64 \u2506 str \u2502 \u255e  \u2550\u256a  \u2550\u256a  \u2550\u2561 \u2502 3 \u2506 8 \u2506 \"c\" \u2502 \u251c  \u254c\u253c  \u254c\u253c  \u254c\u2524 \u2502 4 \u2506 9 \u2506 \"d\" \u2502 \u251c  \u254c\u253c  \u254c\u253c  \u254c\u2524 \u2502 5 \u2506 10 \u2506 \"e\" \u2502 \u2570  \u2500\u2534  \u2500\u2534  \u2500\u256f  ",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.drop_nulls",
"url":2,
"doc":"Return a new DataFrame where the null values are dropped",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.pipe",
"url":2,
"doc":"Apply a function on Self Parameters      func Callable args Arguments kwargs Keyword arguments",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.groupby",
"url":2,
"doc":"Start a groupby operation Parameters      by Column(s) to group by.",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.downsample",
"url":2,
"doc":"Start a downsampling groupby operation. Parameters      by Column that will be used as key in the groupby operation. This should be a date64/date32 column rule Units of the downscaling operation. Any of: - \"second\" - \"minute\" - \"hour\" - \"day\" n Number of units (e.g. 5 \"day\", 15 \"minute\"",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.join",
"url":2,
"doc":"SQL like joins Parameters      df DataFrame to join with left_on Name(s) of the left join column(s) right_on Name(s) of the right join column(s) on Name(s) of the join columns in both DataFrames how Join strategy - \"inner\" - \"left\" - \"outer\" Example  -   >>> dataframe = pl.DataFrame({ \"foo\": [1, 2, 3], \"bar\": [6.0, 7.0, 8.0], \"ham\": ['a', 'b', 'c'] }) >>> other_dataframe = pl.DataFrame({ \"apple\": ['x', 'y', 'z'], \"ham\": ['a', 'b', 'd'] }) >>> dataframe.join(other_dataframe, on='ham') shape: (2, 4) \u256d  \u2500\u252c  \u2500\u252c  \u2500\u252c   \u2500\u256e \u2502 foo \u2506 bar \u2506 ham \u2506 apple \u2502 \u2502  - \u2506  - \u2506  - \u2506  - \u2502 \u2502 i64 \u2506 f64 \u2506 str \u2506 str \u2502 \u255e  \u2550\u256a  \u2550\u256a  \u2550\u256a   \u2550\u2561 \u2502 1 \u2506 6 \u2506 \"a\" \u2506 \"x\" \u2502 \u251c  \u254c\u253c  \u254c\u253c  \u254c\u253c   \u254c\u2524 \u2502 2 \u2506 7 \u2506 \"b\" \u2506 \"y\" \u2502 \u2570  \u2500\u2534  \u2500\u2534  \u2500\u2534   \u2500\u256f >>> dataframe.join(other_dataframe, on='ham', how='outer') shape: (4, 4) \u256d   \u252c   \u252c  \u2500\u252c   \u2500\u256e \u2502 foo \u2506 bar \u2506 ham \u2506 apple \u2502 \u2502  - \u2506  - \u2506  - \u2506  - \u2502 \u2502 i64 \u2506 f64 \u2506 str \u2506 str \u2502 \u255e   \u256a   \u256a  \u2550\u256a   \u2550\u2561 \u2502 1 \u2506 6 \u2506 \"a\" \u2506 \"x\" \u2502 \u251c   \u253c   \u253c  \u254c\u253c   \u254c\u2524 \u2502 2 \u2506 7 \u2506 \"b\" \u2506 \"y\" \u2502 \u251c   \u253c   \u253c  \u254c\u253c   \u254c\u2524 \u2502 null \u2506 null \u2506 \"d\" \u2506 \"z\" \u2502 \u251c   \u253c   \u253c  \u254c\u253c   \u254c\u2524 \u2502 3 \u2506 8 \u2506 \"c\" \u2506 null \u2502 \u2570   \u2534   \u2534  \u2500\u2534   \u2500\u256f   Returns    - Joined DataFrame",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.hstack",
"url":2,
"doc":"Return a new DataFrame grown horizontally by stacking multiple Series to it. Parameters      columns Series to stack in_place Modify in place",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.vstack",
"url":2,
"doc":"Grow this DataFrame vertically by stacking a DataFrame to it. Parameters      df DataFrame to stack in_place Modify in place",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.drop",
"url":2,
"doc":"Remove column from DataFrame and return as new. Parameters      name Column(s) to drop Example  -   >>> dataframe = pl.DataFrame({ \"foo\": [1, 2, 3], \"bar\": [6.0, 7.0, 8.0], \"ham\": ['a', 'b', 'c'] }) >>> dataframe.drop('ham') shape: (3, 2) \u256d  \u2500\u252c  \u2500\u256e \u2502 foo \u2506 bar \u2502 \u2502  - \u2506  - \u2502 \u2502 i64 \u2506 f64 \u2502 \u255e  \u2550\u256a  \u2550\u2561 \u2502 1 \u2506 6 \u2502 \u251c  \u254c\u253c  \u254c\u2524 \u2502 2 \u2506 7 \u2502 \u251c  \u254c\u253c  \u254c\u2524 \u2502 3 \u2506 8 \u2502 \u2570  \u2500\u2534  \u2500\u256f  ",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.drop_in_place",
"url":2,
"doc":"Drop in place Parameters      name Column to drop",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.select_at_idx",
"url":2,
"doc":"Select column at index location. Parameters      idx Location of selection",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.clone",
"url":2,
"doc":"Very cheap deep clone",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.get_columns",
"url":2,
"doc":"Get the DataFrame as a List of Series",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.fill_none",
"url":2,
"doc":"Fill None values by a filling strategy. Parameters      strategy - \"backward\" - \"forward\" - \"mean\" - \"min' - \"max\" Returns    - DataFrame with None replaced with the filling strategy.",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.explode",
"url":2,
"doc":"Explode  DataFrame to long format by exploding a column with Lists. Parameters      columns Column of LargeList type Returns    - DataFrame",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.melt",
"url":2,
"doc":"Unpivot DataFrame to long format. Parameters      id_vars Columns to use as identifier variables value_vars Values to use as identifier variables Returns    -",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.shift",
"url":2,
"doc":"Shift the values by a given period and fill the parts that will be empty due to this operation with  Nones . Parameters      periods Number of places to shift (may be negative).",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.is_duplicated",
"url":2,
"doc":"Get a mask of all duplicated rows in this DataFrame",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.is_unique",
"url":2,
"doc":"Get a mask of all unique rows in this DataFrame",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.lazy",
"url":2,
"doc":"Start a lazy query from this point. This returns a  LazyFrame object. Operations on a  LazyFrame are not executed until this is requested by either calling:   .fetch() (run on a small number of rows)   .collect() (run on all data)   .describe_plan() (print unoptimized query plan)   .describe_optimized_plan() (print optimized query plan)   .show_graph() (show (un)optimized query plan) as graphiz graph. Lazy operations are advised because the allow for query optimization and more parallelization.",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.n_chunks",
"url":2,
"doc":"Get number of chunks used by the ChunkedArrays of this DataFrame",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.max",
"url":2,
"doc":"Aggregate the columns of this DataFrame to their maximum value",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.min",
"url":2,
"doc":"Aggregate the columns of this DataFrame to their minimum value",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.sum",
"url":2,
"doc":"Aggregate the columns of this DataFrame to their sum value",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.mean",
"url":2,
"doc":"Aggregate the columns of this DataFrame to their mean value",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.std",
"url":2,
"doc":"Aggregate the columns of this DataFrame to their standard deviation value",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.var",
"url":2,
"doc":"Aggregate the columns of this DataFrame to their variance value",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.median",
"url":2,
"doc":"Aggregate the columns of this DataFrame to their median value",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.quantile",
"url":2,
"doc":"Aggregate the columns of this DataFrame to their quantile value",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.to_dummies",
"url":2,
"doc":"Get one hot encoded dummy variables.",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.drop_duplicates",
"url":2,
"doc":"Drop duplicate rows from this DataFrame. Note that this fails if there is a column of type  List in the DataFrame.",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.rechunk",
"url":2,
"doc":"Rechunk the data in this DataFrame to a contiguous allocation. This will make sure all subsequent operations have optimal and predictable performance",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.null_count",
"url":2,
"doc":"Create a new DataFrame that shows the null counts per column.",
"func":1
},
{
"ref":"pypolars.frame.DataFrame.sample",
"url":2,
"doc":"Sample from this DataFrame by setting either  n or  frac Parameters      n Number of samples < self.len() frac Fraction between 0.0 and 1.0 with_replacement Sample with replacement",
"func":1
},
{
"ref":"pypolars.frame.GroupBy",
"url":2,
"doc":""
},
{
"ref":"pypolars.frame.GroupBy.apply",
"url":2,
"doc":"Apply a function over the groups as a sub-DataFrame. Parameters      f Custom function Returns    - DataFrame",
"func":1
},
{
"ref":"pypolars.frame.GroupBy.agg",
"url":2,
"doc":"Use multiple aggregations on columns Parameters      column_to_agg map column to aggregation functions Examples: [(\"foo\", [\"sum\", \"n_unique\", \"min\"]), (\"bar\": [\"max\"])] {\"foo\": [\"sum\", \"n_unique\", \"min\"], \"bar\": \"max\" } Returns    - Result of groupby split apply operations.",
"func":1
},
{
"ref":"pypolars.frame.GroupBy.select",
"url":2,
"doc":"Select the columns that will be aggregated. Parameters      columns One or multiple columns",
"func":1
},
{
"ref":"pypolars.frame.GroupBy.select_all",
"url":2,
"doc":"Select all columns for aggregation.",
"func":1
},
{
"ref":"pypolars.frame.GroupBy.pivot",
"url":2,
"doc":"Do a pivot operation based on the group key, a pivot column and an aggregation function on the values column. Parameters      pivot_column Column to pivot. values_column Column that will be aggregated",
"func":1
},
{
"ref":"pypolars.frame.GroupBy.first",
"url":2,
"doc":"Aggregate the first values in the group.",
"func":1
},
{
"ref":"pypolars.frame.GroupBy.last",
"url":2,
"doc":"Aggregate the last values in the group.",
"func":1
},
{
"ref":"pypolars.frame.GroupBy.sum",
"url":2,
"doc":"Reduce the groups to the sum.",
"func":1
},
{
"ref":"pypolars.frame.GroupBy.min",
"url":2,
"doc":"Reduce the groups to the minimal value.",
"func":1
},
{
"ref":"pypolars.frame.GroupBy.max",
"url":2,
"doc":"Reduce the groups to the maximal value.",
"func":1
},
{
"ref":"pypolars.frame.GroupBy.count",
"url":2,
"doc":"Count the number of values in each group.",
"func":1
},
{
"ref":"pypolars.frame.GroupBy.mean",
"url":2,
"doc":"Reduce the groups to the mean values.",
"func":1
},
{
"ref":"pypolars.frame.GroupBy.n_unique",
"url":2,
"doc":"Count the unique values per group.",
"func":1
},
{
"ref":"pypolars.frame.GroupBy.quantile",
"url":2,
"doc":"Count the unique values per group.",
"func":1
},
{
"ref":"pypolars.frame.GroupBy.median",
"url":2,
"doc":"Return the median per group.",
"func":1
},
{
"ref":"pypolars.frame.GroupBy.agg_list",
"url":2,
"doc":"Aggregate the groups into Series.",
"func":1
},
{
"ref":"pypolars.frame.PivotOps",
"url":2,
"doc":""
},
{
"ref":"pypolars.frame.PivotOps.first",
"url":2,
"doc":"Get the first value per group.",
"func":1
},
{
"ref":"pypolars.frame.PivotOps.sum",
"url":2,
"doc":"Get the sum per group.",
"func":1
},
{
"ref":"pypolars.frame.PivotOps.min",
"url":2,
"doc":"Get the minimal value per group.",
"func":1
},
{
"ref":"pypolars.frame.PivotOps.max",
"url":2,
"doc":"Get the maximal value per group.",
"func":1
},
{
"ref":"pypolars.frame.PivotOps.mean",
"url":2,
"doc":"Get the mean value per group.",
"func":1
},
{
"ref":"pypolars.frame.PivotOps.count",
"url":2,
"doc":"Count the values per group.",
"func":1
},
{
"ref":"pypolars.frame.PivotOps.median",
"url":2,
"doc":"Get the median value per group.",
"func":1
},
{
"ref":"pypolars.frame.GBSelection",
"url":2,
"doc":""
},
{
"ref":"pypolars.frame.GBSelection.first",
"url":2,
"doc":"Aggregate the first values in the group.",
"func":1
},
{
"ref":"pypolars.frame.GBSelection.last",
"url":2,
"doc":"Aggregate the last values in the group.",
"func":1
},
{
"ref":"pypolars.frame.GBSelection.sum",
"url":2,
"doc":"Reduce the groups to the sum.",
"func":1
},
{
"ref":"pypolars.frame.GBSelection.min",
"url":2,
"doc":"Reduce the groups to the minimal value.",
"func":1
},
{
"ref":"pypolars.frame.GBSelection.max",
"url":2,
"doc":"Reduce the groups to the maximal value.",
"func":1
},
{
"ref":"pypolars.frame.GBSelection.count",
"url":2,
"doc":"Count the number of values in each group.",
"func":1
},
{
"ref":"pypolars.frame.GBSelection.mean",
"url":2,
"doc":"Reduce the groups to the mean values.",
"func":1
},
{
"ref":"pypolars.frame.GBSelection.n_unique",
"url":2,
"doc":"Count the unique values per group.",
"func":1
},
{
"ref":"pypolars.frame.GBSelection.quantile",
"url":2,
"doc":"Compute the quantile per group",
"func":1
},
{
"ref":"pypolars.frame.GBSelection.median",
"url":2,
"doc":"Return the median per group.",
"func":1
},
{
"ref":"pypolars.frame.GBSelection.agg_list",
"url":2,
"doc":"Aggregate the groups into Series.",
"func":1
},
{
"ref":"pypolars.frame.GBSelection.apply",
"url":2,
"doc":"Apply a function over the groups",
"func":1
},
{
"ref":"pypolars.frame.StringCache",
"url":2,
"doc":"Context manager that allows to data sources to share the same categorical features. This will temporarily cache the string categories until the context manager is finished."
},
{
"ref":"pypolars.datatypes",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.Int8",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.Int16",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.Int32",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.Int64",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.UInt8",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.UInt16",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.UInt32",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.UInt64",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.Float32",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.Float64",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.Boolean",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.Utf8",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.List",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.Date32",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.Date64",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.Time32Millisecond",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.Time32Second",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.Time64Nanosecond",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.Time64Microsecond",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.DurationNanosecond",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.DurationMicrosecond",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.DurationMillisecond",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.DurationSecond",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.TimestampNanosecond",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.TimestampMicrosecond",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.TimestampMillisecond",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.TimestampSecond",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.Object",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.Categorical",
"url":3,
"doc":""
},
{
"ref":"pypolars.datatypes.dtype_to_primitive",
"url":3,
"doc":"",
"func":1
},
{
"ref":"pypolars.datatypes.pytype_to_polars_type",
"url":3,
"doc":"",
"func":1
},
{
"ref":"pypolars.functions",
"url":4,
"doc":""
},
{
"ref":"pypolars.functions.get_dummies",
"url":4,
"doc":"",
"func":1
},
{
"ref":"pypolars.functions.read_csv",
"url":4,
"doc":"Read into a DataFrame from a csv file. Parameters      file Path to a file or a file like object. infer_schema_length Maximum number of lines to read to infer schema. batch_size Number of lines to read into the buffer at once. Modify this to change performance. has_headers If the CSV file has headers or not. ignore_errors Try to keep reading lines if some lines yield errors. stop_after_n_rows After n rows are read from the CSV stop reading. During multi-threaded parsing, an upper bound of  n rows cannot be guaranteed. skip_rows Start reading after  skip_rows . projection Indexes of columns to select sep Delimiter/ value seperator columns Columns to project/ select rechunk Make sure that all columns are contiguous in memory by aggregating the chunks into a single array. encoding - \"utf8\" _ \"utf8-lossy\" n_threads Number of threads to use in csv parsing. Defaults to the number of physical cpu's of you system. dtype Overwrite the dtypes during inference use_pyarrow Use pyarrow's native CSV parser. Returns    - DataFrame",
"func":1
},
{
"ref":"pypolars.functions.scan_csv",
"url":4,
"doc":"Lazily read from a csv file. This allows the query optimizer to push down predicates and projections to the scan level, thereby potentially reducing memory overhead. Parameters      file Path to a file has_headers If the CSV file has headers or not. ignore_errors Try to keep reading lines if some lines yield errors. sep Delimiter/ value separator skip_rows Start reading after  skip_rows . stop_after_n_rows After n rows are read from the CSV stop reading. During multi-threaded parsing, an upper bound of  n rows cannot be guaranteed. cache Cache the result after reading dtype Overwrite the dtypes during inference",
"func":1
},
{
"ref":"pypolars.functions.scan_parquet",
"url":4,
"doc":"Lazily read from a parquet file. This allows the query optimizer to push down predicates and projections to the scan level, thereby potentially reducing memory overhead. Parameters      file Path to a file stop_after_n_rows After n rows are read from the parquet stops reading. cache Cache the result after reading",
"func":1
},
{
"ref":"pypolars.functions.read_ipc",
"url":4,
"doc":"Read into a DataFrame from Arrow IPC stream format. This is also called the feather format. Parameters      file Path to a file or a file like object. Returns    - DataFrame",
"func":1
},
{
"ref":"pypolars.functions.read_parquet",
"url":4,
"doc":"Read into a DataFrame from a parquet file. Parameters      file Path to a file or a file like object. stop_after_n_rows After n rows are read from the parquet stops reading. memory_map Memory map underlying file. This will likely increase performance. Returns    - DataFrame",
"func":1
},
{
"ref":"pypolars.functions.arg_where",
"url":4,
"doc":"Get index values where Boolean mask evaluate True. Parameters      mask Boolean Series Returns    - UInt32 Series",
"func":1
},
{
"ref":"pypolars.functions.from_arrow_table",
"url":4,
"doc":"Create a DataFrame from an arrow Table Parameters      table Arrow Table rechunk Make sure that all data is contiguous.",
"func":1
},
{
"ref":"pypolars.functions.from_pandas",
"url":4,
"doc":"Convert from a pandas DataFrame to a polars DataFrame Parameters      df DataFrame to convert rechunk Make sure that all data is contiguous. Returns    - A Polars DataFrame",
"func":1
},
{
"ref":"pypolars.functions.concat",
"url":4,
"doc":"Aggregate all the Dataframe in a List of DataFrames to a single DataFrame Parameters      dfs DataFrames to concatenate rechunk rechunk the final DataFrame",
"func":1
},
{
"ref":"pypolars.functions.range",
"url":4,
"doc":"Create a Series that ranges from lower bound to upper bound. Parameters      lower Lower bound value. upper Upper bound value. step Optional step size. If none given, the step size will be 1. name Name of the Series",
"func":1
},
{
"ref":"pypolars.functions.repeat",
"url":4,
"doc":"Repeat a single value n times and collect into a Series. Parameters      val Value to repeat. n Number of repeats. name Optional name of the Series.",
"func":1
},
{
"ref":"pypolars.html",
"url":5,
"doc":"Module for formatting output data in HTML."
},
{
"ref":"pypolars.html.Tag",
"url":5,
"doc":""
},
{
"ref":"pypolars.html.tag",
"url":5,
"doc":"",
"func":1
},
{
"ref":"pypolars.html.th",
"url":5,
"doc":"",
"func":1
},
{
"ref":"pypolars.html.tr",
"url":5,
"doc":"",
"func":1
},
{
"ref":"pypolars.html.thead",
"url":5,
"doc":"",
"func":1
},
{
"ref":"pypolars.html.tbody",
"url":5,
"doc":"",
"func":1
},
{
"ref":"pypolars.html.table",
"url":5,
"doc":"",
"func":1
},
{
"ref":"pypolars.html.div",
"url":5,
"doc":"",
"func":1
},
{
"ref":"pypolars.html.HTMLFormatter",
"url":5,
"doc":""
},
{
"ref":"pypolars.html.HTMLFormatter.write_header",
"url":5,
"doc":"Writes the header of an HTML table",
"func":1
},
{
"ref":"pypolars.html.HTMLFormatter.write_body",
"url":5,
"doc":"Writes the body of an HTML table",
"func":1
},
{
"ref":"pypolars.html.HTMLFormatter.write",
"url":5,
"doc":"",
"func":1
},
{
"ref":"pypolars.html.HTMLFormatter.render",
"url":5,
"doc":"",
"func":1
},
{
"ref":"pypolars.html.NotebookFormatter",
"url":5,
"doc":"Internal class for formatting output data in html for display in Jupyter Notebooks. This class is intended for functionality specific to DataFrame._repr_html_() and DataFrame.to_html(notebook=True)"
},
{
"ref":"pypolars.html.NotebookFormatter.write_style",
"url":5,
"doc":"",
"func":1
},
{
"ref":"pypolars.html.NotebookFormatter.render",
"url":5,
"doc":"Return the lines needed to render a HTML table.",
"func":1
},
{
"ref":"pypolars.html.NotebookFormatter.write_header",
"url":5,
"doc":"Writes the header of an HTML table",
"func":1
},
{
"ref":"pypolars.html.NotebookFormatter.write_body",
"url":5,
"doc":"Writes the body of an HTML table",
"func":1
},
{
"ref":"pypolars.lazy",
"url":6,
"doc":""
},
{
"ref":"pypolars.lazy.LazyGroupBy",
"url":6,
"doc":""
},
{
"ref":"pypolars.lazy.LazyGroupBy.agg",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.LazyGroupBy.apply",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame",
"url":6,
"doc":""
},
{
"ref":"pypolars.lazy.LazyFrame.from_pyldf",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.scan_csv",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.scan_parquet",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.pipe",
"url":6,
"doc":"Apply a function on Self Parameters      func Callable args Arguments kwargs Keyword arguments",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.describe_plan",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.show_graph",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.describe_optimized_plan",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.sort",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.collect",
"url":6,
"doc":"Collect into a DataFrame Parameters      type_coercion do type coercion optimization predicate_pushdown do predicate pushdown optimization projection_pushdown do projection pushdown optimization simplify_expression run simplify expressions optimization Returns    - DataFrame",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.fetch",
"url":6,
"doc":"Fetch is like a collect operation, but it overwrites the number of rows read by every scan operation. This is a utility that helps debug a query on a smaller number of rows. Note that the fetch does not guarantee the final number of rows in the DataFrame. Filter, join operations and a lower number of rows available in the scanned file influence the final number of rows. Parameters      n_rows Collect n_rows from the data sources. type_coercion run type coercion optimization predicate_pushdown run predicate pushdown optimization projection_pushdown run projection pushdown optimization simplify_expression run simplify expressions optimization Returns    - DataFrame",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.cache",
"url":6,
"doc":"Cache the result once Physical plan hits this node.",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.filter",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.select",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.groupby",
"url":6,
"doc":"Start a groupby operation Parameters      by Column(s) to group by.",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.join",
"url":6,
"doc":"Add a join operation to the Logical Plan. Parameters      ldf Lazy DataFrame to join with left_on Join column of the left DataFrame. right_on Join column of the right DataFrame. on Join column of both DataFrames. If set,  left_on and  right_on should be None. how one of: \"inner\" \"left\" \"outer\" allow_parallel Allow the physical plan to optionally evaluate the computation of both DataFrames up to the join in parallel. force_parallel Force the physical plan evaluate the computation of both DataFrames up to the join in parallel.",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.with_columns",
"url":6,
"doc":"Add or overwrite multiple columns in a DataFrame Parameters      exprs List of Expressions that evaluate to columns",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.with_column",
"url":6,
"doc":"Add or overwrite column in a DataFrame Parameters      expr Expression that evaluates to column",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.drop_columns",
"url":6,
"doc":"Remove multiple columns from a DataFrame Parameters      columns List of column names",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.drop_column",
"url":6,
"doc":"Remove a column from the DataFrame Parameters      column Name of the column that should be removed",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.with_column_renamed",
"url":6,
"doc":"Rename a column in the DataFrame",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.reverse",
"url":6,
"doc":"Reverse the DataFrame.",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.shift",
"url":6,
"doc":"Shift the values by a given period and fill the parts that will be empty due to this operation with  Nones . Parameters      periods Number of places to shift (may be negative).",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.slice",
"url":6,
"doc":"Slice the DataFrame Parameters      offset Start index length Length of the slice",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.limit",
"url":6,
"doc":"Limit the DataFrame to the first  n rows. Note if you don't want the rows to be scanned, use the  fetch operation. Parameters      n Number of rows.",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.fill_none",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.std",
"url":6,
"doc":"Aggregate the columns in the DataFrame to their standard deviation value",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.var",
"url":6,
"doc":"Aggregate the columns in the DataFrame to their variance value",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.max",
"url":6,
"doc":"Aggregate the columns in the DataFrame to their maximum value",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.min",
"url":6,
"doc":"Aggregate the columns in the DataFrame to their minimum value",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.sum",
"url":6,
"doc":"Aggregate the columns in the DataFrame to their sum value",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.mean",
"url":6,
"doc":"Aggregate the columns in the DataFrame to their mean value",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.median",
"url":6,
"doc":"Aggregate the columns in the DataFrame to their median value",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.quantile",
"url":6,
"doc":"Aggregate the columns in the DataFrame to their quantile value",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.explode",
"url":6,
"doc":"Explode lists to long format",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.drop_duplicates",
"url":6,
"doc":"Drop duplicate rows from this DataFrame. Note that this fails if there is a column of type  List in the DataFrame.",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.drop_nulls",
"url":6,
"doc":"Drop rows with null values from this DataFrame.",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.melt",
"url":6,
"doc":"Unpivot DataFrame to long format. Parameters      id_vars Columns to use as identifier variables value_vars Values to use as identifier variables",
"func":1
},
{
"ref":"pypolars.lazy.LazyFrame.map",
"url":6,
"doc":"Apply a custom UDF. It is important that the UDF returns a Polars DataFrame. Parameters      f lambda/ function to apply predicate_pushdown Allow predicate pushdown optimization to pass this node projection_pushdown Allow projection pushdown optimization to pass this node",
"func":1
},
{
"ref":"pypolars.lazy.Expr",
"url":6,
"doc":""
},
{
"ref":"pypolars.lazy.Expr.from_pyexpr",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.eq",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.neq",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.gt",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.gt_eq",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.lt_eq",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.lt",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.alias",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.is_not",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.is_null",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.is_not_null",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.is_finite",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.is_infinite",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.is_nan",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.is_not_nan",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.agg_groups",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.count",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.cum_sum",
"url":6,
"doc":"Get an array with the cumulative sum computed at every element Parameters      reverse reverse the operation",
"func":1
},
{
"ref":"pypolars.lazy.Expr.cum_min",
"url":6,
"doc":"Get an array with the cumulative min computed at every element Parameters      reverse reverse the operation",
"func":1
},
{
"ref":"pypolars.lazy.Expr.cum_max",
"url":6,
"doc":"Get an array with the cumulative max computed at every element Parameters      reverse reverse the operation",
"func":1
},
{
"ref":"pypolars.lazy.Expr.cast",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.sort",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.shift",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.fill_none",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.reverse",
"url":6,
"doc":"Reverse the selection",
"func":1
},
{
"ref":"pypolars.lazy.Expr.std",
"url":6,
"doc":"Get standard deviation",
"func":1
},
{
"ref":"pypolars.lazy.Expr.var",
"url":6,
"doc":"Get variance",
"func":1
},
{
"ref":"pypolars.lazy.Expr.max",
"url":6,
"doc":"Get maximum value",
"func":1
},
{
"ref":"pypolars.lazy.Expr.min",
"url":6,
"doc":"Get minimum value",
"func":1
},
{
"ref":"pypolars.lazy.Expr.sum",
"url":6,
"doc":"Get sum value",
"func":1
},
{
"ref":"pypolars.lazy.Expr.mean",
"url":6,
"doc":"Get mean value",
"func":1
},
{
"ref":"pypolars.lazy.Expr.median",
"url":6,
"doc":"Get median value",
"func":1
},
{
"ref":"pypolars.lazy.Expr.n_unique",
"url":6,
"doc":"Count unique values",
"func":1
},
{
"ref":"pypolars.lazy.Expr.first",
"url":6,
"doc":"Get first value",
"func":1
},
{
"ref":"pypolars.lazy.Expr.last",
"url":6,
"doc":"Get last value",
"func":1
},
{
"ref":"pypolars.lazy.Expr.list",
"url":6,
"doc":"Aggregate to list",
"func":1
},
{
"ref":"pypolars.lazy.Expr.over",
"url":6,
"doc":"Apply window function over a subgroup. This is similar to a groupby + aggregation + self join. Or similar to [window functions in Postgres](https: www.postgresql.org/docs/9.1/tutorial-window.html).Do Parameters      expr Expression that evaluates to a column of groups Examples      python df = DataFrame({ \"groups\": [1, 1, 2, 2, 1, 2, 3, 3, 1], \"values\": [1, 2, 3, 4, 5, 6, 7, 8, 8] }) print(df.lazy() .select([ col(\"groups\") sum(\"values\").over(\"groups\" ]).collect(   outputs:  text \u256d    \u252c    \u256e \u2502 groups \u2506 values \u2502 \u2502  - \u2506  - \u2502 \u2502 i32 \u2506 i32 \u2502 \u255e    \u256a    \u2561 \u2502 1 \u2506 16 \u2502 \u251c    \u253c    \u2524 \u2502 1 \u2506 16 \u2502 \u251c    \u253c    \u2524 \u2502 2 \u2506 13 \u2502 \u251c    \u253c    \u2524 \u2502 2 \u2506 13 \u2502 \u251c    \u253c    \u2524 \u2502  . \u2506  . \u2502 \u251c    \u253c    \u2524 \u2502 1 \u2506 16 \u2502 \u251c    \u253c    \u2524 \u2502 2 \u2506 13 \u2502 \u251c    \u253c    \u2524 \u2502 3 \u2506 15 \u2502 \u251c    \u253c    \u2524 \u2502 3 \u2506 15 \u2502 \u251c    \u253c    \u2524 \u2502 1 \u2506 16 \u2502 \u2570    \u2534    \u256f  ",
"func":1
},
{
"ref":"pypolars.lazy.Expr.is_unique",
"url":6,
"doc":"Get mask of unique values",
"func":1
},
{
"ref":"pypolars.lazy.Expr.is_duplicated",
"url":6,
"doc":"Get mask of duplicated values",
"func":1
},
{
"ref":"pypolars.lazy.Expr.quantile",
"url":6,
"doc":"Get quantile value",
"func":1
},
{
"ref":"pypolars.lazy.Expr.str_parse_date",
"url":6,
"doc":"Parse utf8 expression as a Date32/Date64 type. Parameters      datatype Date32 | Date64 fmt \"yyyy-mm-dd\"",
"func":1
},
{
"ref":"pypolars.lazy.Expr.str_lengths",
"url":6,
"doc":"Get the length of the Strings as UInt32",
"func":1
},
{
"ref":"pypolars.lazy.Expr.str_to_uppercase",
"url":6,
"doc":"Transform to uppercase variant",
"func":1
},
{
"ref":"pypolars.lazy.Expr.str_to_lowercase",
"url":6,
"doc":"Transform to lowercase variant",
"func":1
},
{
"ref":"pypolars.lazy.Expr.str_contains",
"url":6,
"doc":"Check if string contains regex. Parameters      pattern regex pattern",
"func":1
},
{
"ref":"pypolars.lazy.Expr.str_replace",
"url":6,
"doc":"Replace substring where regex pattern first matches. Parameters      pattern regex pattern value replacement string",
"func":1
},
{
"ref":"pypolars.lazy.Expr.str_replace_all",
"url":6,
"doc":"Replace substring on all regex pattern matches. Parameters      pattern regex pattern value replacement string",
"func":1
},
{
"ref":"pypolars.lazy.Expr.datetime_str_fmt",
"url":6,
"doc":"Format date32/date64 with a formatting rule: See [chrono strftime/strptime](https: docs.rs/chrono/0.4.19/chrono/format/strftime/index.html).",
"func":1
},
{
"ref":"pypolars.lazy.Expr.year",
"url":6,
"doc":"Extract year from underlying Date representation. Can be performed on Date32 and Date64 Returns the year number in the calendar date. Returns    - Year as Int32",
"func":1
},
{
"ref":"pypolars.lazy.Expr.month",
"url":6,
"doc":"Extract month from underlying Date representation. Can be performed on Date32 and Date64 Returns the month number starting from 1. The return value ranges from 1 to 12. Returns    - Month as UInt32",
"func":1
},
{
"ref":"pypolars.lazy.Expr.day",
"url":6,
"doc":"Extract day from underlying Date representation. Can be performed on Date32 and Date64 Returns the day of month starting from 1. The return value ranges from 1 to 31. (The last day of month differs by months.) Returns    - Day as UInt32",
"func":1
},
{
"ref":"pypolars.lazy.Expr.ordinal_day",
"url":6,
"doc":"Extract ordinal day from underlying Date representation. Can be performed on Date32 and Date64 Returns the day of year starting from 1. The return value ranges from 1 to 366. (The last day of year differs by years.) Returns    - Day as UInt32",
"func":1
},
{
"ref":"pypolars.lazy.Expr.hour",
"url":6,
"doc":"Extract day from underlying DateTime representation. Can be performed on Date64 Returns the hour number from 0 to 23. Returns    - Hour as UInt32",
"func":1
},
{
"ref":"pypolars.lazy.Expr.minute",
"url":6,
"doc":"Extract minutes from underlying DateTime representation. Can be performed on Date64 Returns the minute number from 0 to 59. Returns    - Minute as UInt32",
"func":1
},
{
"ref":"pypolars.lazy.Expr.second",
"url":6,
"doc":"Extract seconds from underlying DateTime representation. Can be performed on Date64 Returns the second number from 0 to 59. Returns    - Second as UInt32",
"func":1
},
{
"ref":"pypolars.lazy.Expr.nanosecond",
"url":6,
"doc":"Extract seconds from underlying DateTime representation. Can be performed on Date64 Returns the number of nanoseconds since the whole non-leap second. The range from 1,000,000,000 to 1,999,999,999 represents the leap second. Returns    - Nanosecond as UInt32",
"func":1
},
{
"ref":"pypolars.lazy.Expr.map",
"url":6,
"doc":"Apply a custom UDF. It is important that the UDF returns a Polars Series. [read more in the book](https: ritchie46.github.io/polars-book/how_can_i/use_custom_functions.html lazy) Parameters      f lambda/ function to apply dtype_out dtype of the output Series",
"func":1
},
{
"ref":"pypolars.lazy.Expr.apply",
"url":6,
"doc":"Apply a custom UDF in a GroupBy context. This is syntactic sugar for the  apply method which operates on all groups at once. The UDF passed to this expression will operate on a single group. Parameters      f lambda/ function to apply dtype_out dtype of the output Series  Example   df = pl.DataFrame({\"a\": [1, 2, 1, 1], \"b\": [\"a\", \"b\", \"c\", \"c\"]}) (df .lazy() .groupby(\"b\") .agg([col(\"a\").apply_groups(lambda x: x.sum( ]) .collect() )   > returns   shape: (3, 2) \u256d  \u2500\u252c  \u2500\u256e \u2502 b \u2506 a \u2502 \u2502  - \u2506  - \u2502 \u2502 str \u2506 i64 \u2502 \u255e  \u2550\u256a  \u2550\u2561 \u2502 a \u2506 1 \u2502 \u251c  \u254c\u253c  \u254c\u2524 \u2502 b \u2506 2 \u2502 \u251c  \u254c\u253c  \u254c\u2524 \u2502 c \u2506 2 \u2502 \u2570  \u2500\u2534  \u2500\u256f  ",
"func":1
},
{
"ref":"pypolars.lazy.Expr.explode",
"url":6,
"doc":"Explode a list or utf8 Series. This means that every item is expanded to a new row. Returns    - Exploded Series of same dtype",
"func":1
},
{
"ref":"pypolars.lazy.Expr.take_every",
"url":6,
"doc":"Take every nth value in the Series and return as a new Series",
"func":1
},
{
"ref":"pypolars.lazy.Expr.head",
"url":6,
"doc":"Take the first n values",
"func":1
},
{
"ref":"pypolars.lazy.Expr.tail",
"url":6,
"doc":"Take the last n values",
"func":1
},
{
"ref":"pypolars.lazy.Expr.pow",
"url":6,
"doc":"Raise expression to the power of exponent",
"func":1
},
{
"ref":"pypolars.lazy.Expr.is_between",
"url":6,
"doc":"Check if this expression is between start and end",
"func":1
},
{
"ref":"pypolars.lazy.expr_to_lit_or_expr",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.WhenThen",
"url":6,
"doc":""
},
{
"ref":"pypolars.lazy.WhenThen.otherwise",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.When",
"url":6,
"doc":""
},
{
"ref":"pypolars.lazy.When.then",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.when",
"url":6,
"doc":"Start a when, then, otherwise expression  Example Below we add a column with the value 1, where column \"foo\" > 2 and the value -1 where it isn't.   lf.with_column( when(col(\"foo\") > 2) .then(lit(1 .otherwise(lit(-1 )  ",
"func":1
},
{
"ref":"pypolars.lazy.col",
"url":6,
"doc":"A column in a DataFrame",
"func":1
},
{
"ref":"pypolars.lazy.except_",
"url":6,
"doc":"Exclude a column from a selection  Example   df = pl.DataFrame({ \"ham\": [1, 1, 2, 2, 3], \"foo\": [1, 1, 2, 2, 3], \"bar\": [1, 1, 2, 2, 3], }) df.lazy() .select([\" \", except_(\"foo\")]) .collect()   Ouputs:   \u256d  \u2500\u252c  \u2500\u256e \u2502 ham \u2506 bar \u2502 \u2502  - \u2506  - \u2502 \u2502 f64 \u2506 f64 \u2502 \u255e  \u2550\u256a  \u2550\u2561 \u2502 1 \u2506 1 \u2502 \u251c  \u254c\u253c  \u254c\u2524 \u2502 1 \u2506 1 \u2502 \u251c  \u254c\u253c  \u254c\u2524 \u2502 2 \u2506 2 \u2502 \u251c  \u254c\u253c  \u254c\u2524 \u2502 2 \u2506 2 \u2502 \u251c  \u254c\u253c  \u254c\u2524 \u2502 3 \u2506 3 \u2502 \u2570  \u2500\u2534  \u2500\u256f  ",
"func":1
},
{
"ref":"pypolars.lazy.count",
"url":6,
"doc":"Count the number of values in this column",
"func":1
},
{
"ref":"pypolars.lazy.to_list",
"url":6,
"doc":"Aggregate to list",
"func":1
},
{
"ref":"pypolars.lazy.std",
"url":6,
"doc":"Get standard deviation",
"func":1
},
{
"ref":"pypolars.lazy.var",
"url":6,
"doc":"Get variance",
"func":1
},
{
"ref":"pypolars.lazy.max",
"url":6,
"doc":"Get maximum value",
"func":1
},
{
"ref":"pypolars.lazy.min",
"url":6,
"doc":"Get minimum value",
"func":1
},
{
"ref":"pypolars.lazy.sum",
"url":6,
"doc":"Get sum value",
"func":1
},
{
"ref":"pypolars.lazy.mean",
"url":6,
"doc":"Get mean value",
"func":1
},
{
"ref":"pypolars.lazy.avg",
"url":6,
"doc":"Alias for mean",
"func":1
},
{
"ref":"pypolars.lazy.median",
"url":6,
"doc":"Get median value",
"func":1
},
{
"ref":"pypolars.lazy.n_unique",
"url":6,
"doc":"Count unique values",
"func":1
},
{
"ref":"pypolars.lazy.first",
"url":6,
"doc":"Get first value",
"func":1
},
{
"ref":"pypolars.lazy.last",
"url":6,
"doc":"Get last value",
"func":1
},
{
"ref":"pypolars.lazy.head",
"url":6,
"doc":"Get the first n rows of an Expression Parameters      name column name n number of rows to take",
"func":1
},
{
"ref":"pypolars.lazy.tail",
"url":6,
"doc":"Get the last n rows of an Expression Parameters      name column name n number of rows to take",
"func":1
},
{
"ref":"pypolars.lazy.lit_date",
"url":6,
"doc":"Converts a Python DateTime to a literal Expression. Parameters      dt datetime.datetime",
"func":1
},
{
"ref":"pypolars.lazy.lit",
"url":6,
"doc":"A literal value  Example    literal integer lit(1)  literal str. lit(\"foo\")  literal date64 lit(datetime(2021, 1, 20  ",
"func":1
},
{
"ref":"pypolars.lazy.pearson_corr",
"url":6,
"doc":"Compute the pearson's correlation between two columns Parameters      a Column name or Expression b Column name or Expression",
"func":1
},
{
"ref":"pypolars.lazy.cov",
"url":6,
"doc":"Compute the covariance between two columns/ expressions. Parameters      a Column name or Expression b Column name or Expression",
"func":1
},
{
"ref":"pypolars.lazy.map_binary",
"url":6,
"doc":"Map a custom function over two columns and produce a single Series result. Parameters      a Input Series a b Input Series b f Function to apply output_type Output type of the udf",
"func":1
},
{
"ref":"pypolars.lazy.fold",
"url":6,
"doc":"Accumulate over multiple columns horizontally / row wise with a left fold. Parameters      acc Accumulator Expression. This is the value that will be initialized when the fold starts. For a sum this could for instance be lit(0) f Function to apply over the accumulator and the value Fn(acc, value) -> new_value exprs Expressions to aggregate over",
"func":1
},
{
"ref":"pypolars.lazy.any",
"url":6,
"doc":"Evaluate columnwise or elementwise with a bitwise OR operation",
"func":1
},
{
"ref":"pypolars.lazy.all",
"url":6,
"doc":"Evaluate columnwise or elementwise with a bitwise OR operation",
"func":1
},
{
"ref":"pypolars.lazy.UDF",
"url":6,
"doc":""
},
{
"ref":"pypolars.lazy.udf",
"url":6,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.range",
"url":6,
"doc":"Create a range expression. This can be used in a  select ,  with_column etc. Be sure that the range size is equal to the DataFrame you are collecting.  Example   (df.lazy() .filter(pl.col(\"foo\") < pl.range(0, 100 .collect(   Parameters      low lower bound of range. high upper bound of range. dtype DataType of the range. Valid dtypes:  Int32  Int64  UInt32",
"func":1
},
{
"ref":"pypolars.lazy.agg",
"url":7,
"doc":""
},
{
"ref":"pypolars.lazy.agg.count",
"url":7,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.agg.sum",
"url":7,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.agg.min",
"url":7,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.agg.max",
"url":7,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.agg.first",
"url":7,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.agg.last",
"url":7,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.agg.list",
"url":7,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.agg.groups",
"url":7,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.agg.mean",
"url":7,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.agg.median",
"url":7,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.agg.n_unique",
"url":7,
"doc":"",
"func":1
},
{
"ref":"pypolars.lazy.agg.quantile",
"url":7,
"doc":"",
"func":1
}
]