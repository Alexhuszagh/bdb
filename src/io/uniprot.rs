//! Input and output helpers for UniProt models.

use std::convert::AsRef;
use std::path::Path;

use db::uniprot::RecordList;
use traits::*;
use util::ResultType;

/// Reader/writer for UniProt FASTA records.
pub struct UniProtFasta;

impl UniProtFasta {
    /// Save UniProt records to string.
    #[inline(always)]
    pub fn to_string(list: &RecordList) -> ResultType<String> {
        list.to_fasta_string()
    }

    /// Save UniProt records to file.
    #[inline(always)]
    pub fn to_file<P: AsRef<Path>>(list: &RecordList, path: P) -> ResultType<()> {
        list.to_fasta_file(path)
    }

    /// Load UniProt records from string.
    #[inline(always)]
    pub fn from_string(text: &str) -> ResultType<RecordList> {
        RecordList::from_fasta_string(text)
    }

    /// Load UniProt records from file.
    #[inline(always)]
    pub fn from_file<P: AsRef<Path>>(path: P) -> ResultType<RecordList> {
        RecordList::from_fasta_file(path)
    }
}

/// Reader/writer for UniProt CSV (as tab-delimited text) records.
pub struct UniProtCsv;

impl UniProtCsv {
    /// Save UniProt records to string.
    #[inline(always)]
    pub fn to_string(list: &RecordList) -> ResultType<String> {
        list.to_csv_string(b'\t')
    }

    /// Save UniProt records to file.
    #[inline(always)]
    pub fn to_file<P: AsRef<Path>>(list: &RecordList, path: P) -> ResultType<()> {
        list.to_csv_file(path, b'\t')
    }

    /// Load UniProt records from string.
    #[inline(always)]
    pub fn from_string(text: &str) -> ResultType<RecordList> {
        RecordList::from_csv_string(text, b'\t')
    }

    /// Load UniProt records from file.
    #[inline(always)]
    pub fn from_file<P: AsRef<Path>>(path: P) -> ResultType<RecordList> {
        RecordList::from_csv_file(path, b'\t')
    }
}

// TODO(ahuszagh)
//  Add other classes


#[cfg(test)]
mod tests {
    // TODO(ahuszagh)
    // Add tests
}
