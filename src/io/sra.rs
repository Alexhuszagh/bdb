//! Input and output helpers for SRA models.

// RE-EXPORTS

// Use re-exports to avoid name collisions with traits.
#[cfg(feature = "fastq")]
pub use self::private::SraFastq as Fastq;

// PRIVATE
// -------

mod private {

use std::convert::AsRef;
use std::io::{BufRead, Write};
use std::path::Path;

use db::sra::RecordList;
use traits::*;
use util::{Bytes, Result};

/// Reader/writer for SRA FASTQ records.
#[cfg(feature = "fastq")]
pub struct SraFastq;

#[cfg(feature = "fastq")]
impl SraFastq {
    /// Save Sra records to stream.
    #[inline(always)]
    pub fn to_stream<T: Write>(list: &RecordList, writer: &mut T) -> Result<()> {
        list.to_fastq(writer)
    }

    /// Save Sra records to bytes.
    #[inline(always)]
    pub fn to_bytes(list: &RecordList) -> Result<Bytes> {
        list.to_fastq_bytes()
    }

    /// Save Sra records to string.
    #[inline(always)]
    pub fn to_string(list: &RecordList) -> Result<String> {
        list.to_fastq_string()
    }

    /// Save Sra records to file.
    #[inline(always)]
    pub fn to_file<P: AsRef<Path>>(list: &RecordList, path: P) -> Result<()> {
        list.to_fastq_file(path)
    }

    /// Load Sra records from stream.
    #[inline(always)]
    pub fn from_stream<T: BufRead>(reader: &mut T) -> Result<RecordList> {
        RecordList::from_fastq(reader)
    }

    /// Load Sra records from bytes.
    #[inline(always)]
    pub fn from_bytes(bytes: &[u8]) -> Result<RecordList> {
        RecordList::from_fastq_bytes(bytes)
    }

    /// Load Sra records from string.
    #[inline(always)]
    pub fn from_string(string: &str) -> Result<RecordList> {
        RecordList::from_fastq_string(string)
    }

    /// Load Sra records from file.
    #[inline(always)]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<RecordList> {
        RecordList::from_fastq_file(path)
    }
}

}   // private

#[cfg(test)]
mod tests {
    // TODO(ahuszagh)   Implement...
}
