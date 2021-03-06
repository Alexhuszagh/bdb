//! Input and output helpers for UniProt models.

// RE-EXPORTS

// Use re-exports to avoid name collisions with traits.
#[cfg(feature = "csv")]
pub use self::private::UniProtCsv as Csv;

#[cfg(feature = "fasta")]
pub use self::private::UniProtFasta as Fasta;

#[cfg(feature = "xml")]
pub use self::private::UniProtXml as Xml;

// PRIVATE
// -------

mod private {

use std::convert::AsRef;
use std::io::{BufRead, Write};
use std::path::Path;

use db::uniprot::RecordList;
use traits::*;
use util::{Bytes, Result};

/// Reader/writer for UniProt FASTA records.
#[cfg(feature = "fasta")]
pub struct UniProtFasta;

#[cfg(feature = "fasta")]
impl UniProtFasta {
    /// Save UniProt records to stream.
    #[inline(always)]
    pub fn to_stream<T: Write>(list: &RecordList, writer: &mut T) -> Result<()> {
        list.to_fasta(writer)
    }

    /// Save UniProt records to bytes.
    #[inline(always)]
    pub fn to_bytes(list: &RecordList) -> Result<Bytes> {
        list.to_fasta_bytes()
    }

    /// Save UniProt records to string.
    #[inline(always)]
    pub fn to_string(list: &RecordList) -> Result<String> {
        list.to_fasta_string()
    }

    /// Save UniProt records to file.
    #[inline(always)]
    pub fn to_file<P: AsRef<Path>>(list: &RecordList, path: P) -> Result<()> {
        list.to_fasta_file(path)
    }

    /// Load UniProt records from stream.
    #[inline(always)]
    pub fn from_stream<T: BufRead>(reader: &mut T) -> Result<RecordList> {
        RecordList::from_fasta(reader)
    }

    /// Load UniProt records from bytes.
    #[inline(always)]
    pub fn from_bytes(bytes: &[u8]) -> Result<RecordList> {
        RecordList::from_fasta_bytes(bytes)
    }

    /// Load UniProt records from string.
    #[inline(always)]
    pub fn from_string(string: &str) -> Result<RecordList> {
        RecordList::from_fasta_string(string)
    }

    /// Load UniProt records from file.
    #[inline(always)]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<RecordList> {
        RecordList::from_fasta_file(path)
    }
}

/// Reader/writer for UniProt CSV (as tab-delimited text) records.
#[cfg(feature = "csv")]
pub struct UniProtCsv;

#[cfg(feature = "csv")]
impl UniProtCsv {
    /// Save UniProt records to stream.
    #[inline(always)]
    pub fn to_stream<T: Write>(list: &RecordList, writer: &mut T) -> Result<()> {
        list.to_csv(writer, b'\t')
    }

    /// Save UniProt records to bytes.
    #[inline(always)]
    pub fn to_bytes(list: &RecordList) -> Result<Bytes> {
        list.to_csv_bytes(b'\t')
    }

    /// Save UniProt records to string.
    #[inline(always)]
    pub fn to_string(list: &RecordList) -> Result<String> {
        list.to_csv_string(b'\t')
    }

    /// Save UniProt records to file.
    #[inline(always)]
    pub fn to_file<P: AsRef<Path>>(list: &RecordList, path: P) -> Result<()> {
        list.to_csv_file(path, b'\t')
    }

    /// Load UniProt records from stream.
    #[inline(always)]
    pub fn from_stream<T: BufRead>(reader: &mut T) -> Result<RecordList> {
        RecordList::from_csv(reader, b'\t')
    }

    /// Load UniProt records from bytes.
    #[inline(always)]
    pub fn from_bytes(bytes: &[u8]) -> Result<RecordList> {
        RecordList::from_csv_bytes(bytes, b'\t')
    }

    /// Load UniProt records from string.
    #[inline(always)]
    pub fn from_string(string: &str) -> Result<RecordList> {
        RecordList::from_csv_string(string, b'\t')
    }

    /// Load UniProt records from file.
    #[inline(always)]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<RecordList> {
        RecordList::from_csv_file(path, b'\t')
    }
}

/// Reader/writer for UniProt XML records.
#[cfg(feature = "xml")]
pub struct UniProtXml;

#[cfg(feature = "xml")]
impl UniProtXml {
    /// Save UniProt records to stream.
    #[inline(always)]
    pub fn to_stream<T: Write>(list: &RecordList, writer: &mut T) -> Result<()> {
        list.to_xml(writer)
    }

    /// Save UniProt records to bytes.
    #[inline(always)]
    pub fn to_bytes(list: &RecordList) -> Result<Bytes> {
        list.to_xml_bytes()
    }

    /// Save UniProt records to string.
    #[inline(always)]
    pub fn to_string(list: &RecordList) -> Result<String> {
        list.to_xml_string()
    }

    /// Save UniProt records to file.
    #[inline(always)]
    pub fn to_file<P: AsRef<Path>>(list: &RecordList, path: P) -> Result<()> {
        list.to_xml_file(path)
    }

    /// Load UniProt records from stream.
    #[inline(always)]
    pub fn from_stream<T: BufRead>(reader: &mut T) -> Result<RecordList> {
        RecordList::from_xml(reader)
    }

    /// Load UniProt records from bytes.
    #[inline(always)]
    pub fn from_bytes(bytes: &[u8]) -> Result<RecordList> {
        RecordList::from_xml_bytes(bytes)
    }

    /// Load UniProt records from string.
    #[inline(always)]
    pub fn from_string(string: &str) -> Result<RecordList> {
        RecordList::from_xml_string(string)
    }

    /// Load UniProt records from file.
    #[inline(always)]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<RecordList> {
        RecordList::from_xml_file(path)
    }
}

}   // private

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use std::path::PathBuf;
    use test::testdata_dir;
    use super::*;

    #[cfg(feature = "fasta")]
    fn fasta_dir() -> PathBuf {
        let mut dir = testdata_dir();
        dir.push("uniprot/fasta");
        dir
    }

    #[cfg(feature = "fasta")]
    #[test]
    #[ignore]
    fn fasta_test() {
        let mut path = fasta_dir();
        path.push("list.fasta");

        let expected = read_to_string(&path).unwrap();
        let actual = Fasta::to_string(&Fasta::from_file(&path).unwrap()).unwrap();

        // ignore the 1st and 4th element, the TrEMBL formatting differs.
        assert_eq!(expected.lines().nth(1), actual.lines().nth(1));
        assert_eq!(expected.lines().nth(2), actual.lines().nth(2));
    }

    #[cfg(feature = "csv")]
    fn csv_dir() -> PathBuf {
        let mut dir = testdata_dir();
        dir.push("uniprot/csv");
        dir
    }

    #[cfg(feature = "csv")]
    #[test]
    #[ignore]
    fn csv_test() {
        let mut path = csv_dir();
        path.push("list.csv");

        let expected = read_to_string(&path).unwrap();
        let actual = Csv::to_string(&Csv::from_file(&path).unwrap()).unwrap();
        assert_eq!(expected, actual.trim_right());
    }

    #[cfg(feature = "xml")]
    fn xml_dir() -> PathBuf {
        let mut dir = testdata_dir();
        dir.push("uniprot/xml");
        dir
    }

    #[cfg(feature = "xml")]
    #[test]
    #[ignore]
    fn xml_test() {
        let mut path = xml_dir();
        path.push("list.xml");

        let actual = Xml::to_string(&Xml::from_file(&path).unwrap());
        assert!(actual.is_ok());
    }
}
