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
use std::path::Path;

use db::uniprot::RecordList;
use traits::*;
use util::ResultType;

/// Reader/writer for UniProt FASTA records.
#[cfg(feature = "fasta")]
pub struct UniProtFasta;

#[cfg(feature = "fasta")]
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
#[cfg(feature = "csv")]
pub struct UniProtCsv;

#[cfg(feature = "csv")]
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

/// Reader/writer for UniProt XML records.
#[cfg(feature = "xml")]
pub struct UniProtXml;

#[cfg(feature = "xml")]
impl UniProtXml {
    /// Save UniProt records to string.
    #[inline(always)]
    pub fn to_string(list: &RecordList) -> ResultType<String> {
        list.to_xml_string()
    }

    /// Save UniProt records to file.
    #[inline(always)]
    pub fn to_file<P: AsRef<Path>>(list: &RecordList, path: P) -> ResultType<()> {
        list.to_xml_file(path)
    }

    /// Load UniProt records from string.
    #[inline(always)]
    pub fn from_string(text: &str) -> ResultType<RecordList> {
        RecordList::from_xml_string(text)
    }

    /// Load UniProt records from file.
    #[inline(always)]
    pub fn from_file<P: AsRef<Path>>(path: P) -> ResultType<RecordList> {
        RecordList::from_xml_file(path)
    }
}

}   /* private */

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

    // TODO(ahuszagh)
    //  Implement the XML unittests.
}
