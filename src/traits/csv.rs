use std::convert::AsRef;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::Path;

use util::ResultType;

/// Serialize to and from CSV.
///
/// The underlying CSV readers and writers (`rust-csv`) are buffered,
/// so do not wrap out readers and writers in `BufReader` and `BufWriter`,
/// respectively.
pub trait Csv: Sized {
    /// Estimate the size of the resulting CSV output to avoid reallocations.
    #[inline(always)]
    fn estimate_csv_size(&self) -> usize {
        0
    }

    /// Export model to CSV (with headers).
    fn to_csv<T: Write>(&self, writer: &mut T, delimiter: u8) -> ResultType<()>;

    /// Export model to CSV string.
    fn to_csv_string(&self, delimiter: u8) -> ResultType<String> {
        let capacity = self.estimate_csv_size();
        let mut writer = Cursor::new(Vec::with_capacity(capacity));

        self.to_csv(&mut writer, delimiter)?;
        match String::from_utf8(writer.into_inner()) {
            Err(e)  => Err(Box::new(e)),
            Ok(v)   => Ok(v),
        }
    }

    /// Export model to CSV output file.
    #[inline]
    fn to_csv_file<P: AsRef<Path>>(&self, path: P, delimiter: u8) -> ResultType<()> {
        let mut file = File::create(path)?;
        self.to_csv(&mut file, delimiter)
    }

    /// Import model from CSV (with headers).
    ///
    /// Works identically to a collection importer, only fetches at max
    /// 1 record, since the headers are shared over all records.
    fn from_csv<T: Read>(reader: &mut T, delimiter: u8) -> ResultType<Self>;

    /// Import model from CSV string.
    #[inline]
    fn from_csv_string(text: &str, delimiter: u8) -> ResultType<Self> {
        // Rust uses the contents of the immutable &str as the buffer
        // Cursor is then immutable.
        let mut reader = Cursor::new(text);
        Self::from_csv(&mut reader, delimiter)
    }

    /// Import model from CSV file.
    #[inline]
    fn from_csv_file<P: AsRef<Path>>(path: P, delimiter: u8) -> ResultType<Self> {
        let mut reader = File::open(path)?;
        Self::from_csv(&mut reader, delimiter)
    }
}

/// Specialization of the `Csv` trait for collections.
pub trait CsvCollection: Csv {
    /// Export collection to CSV (with headers).
    ///
    /// Returns an error if any of the items within the collection error.
    fn to_csv_strict<T: Write>(&self, writer: &mut T, delimiter: u8) -> ResultType<()>;

//    /// Export collection to CSV (with headers).
//    ///
//    /// Returns an error if none of the items are valid, otherwise,
//    /// exports as many items as possible.
//    fn to_csv_lenient<T: Write>(&self, writer: &mut T, delimiter: u8) -> ResultType<()>;
//
//    /// Import collection from CSV (with headers).
//    ///
//    /// Returns an error if any of the rows within the CSV document
//    /// are invalid.
//    fn from_csv_strict<T: Read>(reader: &mut T, delimiter: u8) -> ResultType<Self>;
//
//    /// Import collection from CSV (with headers).
//    ///
//    /// Returns an error if none of the rows within the CSV document
//    /// are valid, otherwise, imports as many rows as possible.
//    fn from_csv_lenient<T: Read>(reader: &mut T, delimiter: u8) -> ResultType<Self>;
}
