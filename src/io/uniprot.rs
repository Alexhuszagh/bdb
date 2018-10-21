//! Input and output helpers for UniProt models.

// RE-EXPORTS

// Use re-exports to avoid name collisions with traits.
pub use self::private::UniProtCsv as Csv;
pub use self::private::UniProtFasta as Fasta;

// PRIVATE
// -------

mod private {

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

}   /* private */

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use std::path::PathBuf;
    use test::testdata_dir;
    use super::*;

    fn fasta_dir() -> PathBuf {
        let mut dir = testdata_dir();
        dir.push("uniprot/fasta");
        dir
    }

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
}
