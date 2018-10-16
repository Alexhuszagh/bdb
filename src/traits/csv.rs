use util::ResultType;

/// Serialize to and from CSV.
pub trait Csv: Sized {
    /// Export model to CSV (with headers).
    fn to_csv(&self, delimiter: u8) -> ResultType<String>;
    //async fn to_csv_async(&self, delimiter: u8) -> ResultType<String>;

    /// Import model from CSV (with headers).
    ///
    /// Works identically to a collection importer, only fetches at max
    /// 1 record, since the headers are shared over all records.
    fn from_csv(text: &str, delimiter: u8) -> ResultType<Self>;
    //async fn from_csv_async(text: &str, delimiter: u8) -> ResultType<Self>;
}

/// Specialization of the `Csv` trait for collections.
pub trait CsvCollection: Csv {
    /// Export collection to CSV (with headers).
    ///
    /// Returns an error if any of the items within the collection
    /// are invalid.
    fn to_csv_strict(&self, delimiter: u8) -> ResultType<String>;
    //async fn to_csv_strict_async(&self, delimiter: u8) -> ResultType<String>;

    /// Export collection to CSV (with headers).
    ///
    /// Returns an error if none of the items are valid, otherwise,
    /// exports as many items as possible.
    fn to_csv_lenient(&self, delimiter: u8) -> ResultType<String>;
    //async fn to_csv_lenient_async(&self, delimiter: u8) -> ResultType<String>;

    /// Import collection from CSV (with headers).
    ///
    /// Returns an error if any of the rows within the CSV document
    /// are invalid.
    fn from_csv_strict(text: &str) -> ResultType<Self>;
    //async fn from_csv_strict_async(text: &str) -> ResultType<Self>;

    /// Import collection from CSV (with headers).
    ///
    /// Returns an error if none of the rows within the CSV document
    /// are valid, otherwise, imports as many rows as possible.
    fn from_csv_lenient(text: &str) -> ResultType<Self>;
    //async fn from_csv_lenient_async(text: &str) -> ResultType<Self>;
}
