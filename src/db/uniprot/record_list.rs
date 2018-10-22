//! Model for UniProt protein collections.

use super::record::Record;

/// UniProt record collection type.
pub type RecordList = Vec<Record>;

// TESTS
// -----

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, Cursor};
    use std::path::PathBuf;
    use test::testdata_dir;
    use traits::*;
    use util::BufferType;
    use super::*;
    use super::super::test::*;

    // LIST

    #[test]
    fn debug_list() {
        let l = format!("{:?}", vec![gapdh(), bsa()]);
        assert_eq!(l, "[Record { sequence_version: 3, protein_evidence: ProteinLevel, mass: 35780, length: 333, gene: \"GAPDH\", id: \"P46406\", mnemonic: \"G3P_RABIT\", name: \"Glyceraldehyde-3-phosphate dehydrogenase\", organism: \"Oryctolagus cuniculus\", proteome: \"UP000001811\", sequence: [77, 86, 75, 86, 71, 86, 78, 71, 70, 71, 82, 73, 71, 82, 76, 86, 84, 82, 65, 65, 70, 78, 83, 71, 75, 86, 68, 86, 86, 65, 73, 78, 68, 80, 70, 73, 68, 76, 72, 89, 77, 86, 89, 77, 70, 81, 89, 68, 83, 84, 72, 71, 75, 70, 72, 71, 84, 86, 75, 65, 69, 78, 71, 75, 76, 86, 73, 78, 71, 75, 65, 73, 84, 73, 70, 81, 69, 82, 68, 80, 65, 78, 73, 75, 87, 71, 68, 65, 71, 65, 69, 89, 86, 86, 69, 83, 84, 71, 86, 70, 84, 84, 77, 69, 75, 65, 71, 65, 72, 76, 75, 71, 71, 65, 75, 82, 86, 73, 73, 83, 65, 80, 83, 65, 68, 65, 80, 77, 70, 86, 77, 71, 86, 78, 72, 69, 75, 89, 68, 78, 83, 76, 75, 73, 86, 83, 78, 65, 83, 67, 84, 84, 78, 67, 76, 65, 80, 76, 65, 75, 86, 73, 72, 68, 72, 70, 71, 73, 86, 69, 71, 76, 77, 84, 84, 86, 72, 65, 73, 84, 65, 84, 81, 75, 84, 86, 68, 71, 80, 83, 71, 75, 76, 87, 82, 68, 71, 82, 71, 65, 65, 81, 78, 73, 73, 80, 65, 83, 84, 71, 65, 65, 75, 65, 86, 71, 75, 86, 73, 80, 69, 76, 78, 71, 75, 76, 84, 71, 77, 65, 70, 82, 86, 80, 84, 80, 78, 86, 83, 86, 86, 68, 76, 84, 67, 82, 76, 69, 75, 65, 65, 75, 89, 68, 68, 73, 75, 75, 86, 86, 75, 81, 65, 83, 69, 71, 80, 76, 75, 71, 73, 76, 71, 89, 84, 69, 68, 81, 86, 86, 83, 67, 68, 70, 78, 83, 65, 84, 72, 83, 83, 84, 70, 68, 65, 71, 65, 71, 73, 65, 76, 78, 68, 72, 70, 86, 75, 76, 73, 83, 87, 89, 68, 78, 69, 70, 71, 89, 83, 78, 82, 86, 86, 68, 76, 77, 86, 72, 77, 65, 83, 75, 69], taxonomy: \"9986\" }, Record { sequence_version: 4, protein_evidence: ProteinLevel, mass: 69293, length: 607, gene: \"ALB\", id: \"P02769\", mnemonic: \"ALBU_BOVIN\", name: \"Serum albumin\", organism: \"Bos taurus\", proteome: \"UP000009136\", sequence: [77, 75, 87, 86, 84, 70, 73, 83, 76, 76, 76, 76, 70, 83, 83, 65, 89, 83, 82, 71, 86, 70, 82, 82, 68, 84, 72, 75, 83, 69, 73, 65, 72, 82, 70, 75, 68, 76, 71, 69, 69, 72, 70, 75, 71, 76, 86, 76, 73, 65, 70, 83, 81, 89, 76, 81, 81, 67, 80, 70, 68, 69, 72, 86, 75, 76, 86, 78, 69, 76, 84, 69, 70, 65, 75, 84, 67, 86, 65, 68, 69, 83, 72, 65, 71, 67, 69, 75, 83, 76, 72, 84, 76, 70, 71, 68, 69, 76, 67, 75, 86, 65, 83, 76, 82, 69, 84, 89, 71, 68, 77, 65, 68, 67, 67, 69, 75, 81, 69, 80, 69, 82, 78, 69, 67, 70, 76, 83, 72, 75, 68, 68, 83, 80, 68, 76, 80, 75, 76, 75, 80, 68, 80, 78, 84, 76, 67, 68, 69, 70, 75, 65, 68, 69, 75, 75, 70, 87, 71, 75, 89, 76, 89, 69, 73, 65, 82, 82, 72, 80, 89, 70, 89, 65, 80, 69, 76, 76, 89, 89, 65, 78, 75, 89, 78, 71, 86, 70, 81, 69, 67, 67, 81, 65, 69, 68, 75, 71, 65, 67, 76, 76, 80, 75, 73, 69, 84, 77, 82, 69, 75, 86, 76, 65, 83, 83, 65, 82, 81, 82, 76, 82, 67, 65, 83, 73, 81, 75, 70, 71, 69, 82, 65, 76, 75, 65, 87, 83, 86, 65, 82, 76, 83, 81, 75, 70, 80, 75, 65, 69, 70, 86, 69, 86, 84, 75, 76, 86, 84, 68, 76, 84, 75, 86, 72, 75, 69, 67, 67, 72, 71, 68, 76, 76, 69, 67, 65, 68, 68, 82, 65, 68, 76, 65, 75, 89, 73, 67, 68, 78, 81, 68, 84, 73, 83, 83, 75, 76, 75, 69, 67, 67, 68, 75, 80, 76, 76, 69, 75, 83, 72, 67, 73, 65, 69, 86, 69, 75, 68, 65, 73, 80, 69, 78, 76, 80, 80, 76, 84, 65, 68, 70, 65, 69, 68, 75, 68, 86, 67, 75, 78, 89, 81, 69, 65, 75, 68, 65, 70, 76, 71, 83, 70, 76, 89, 69, 89, 83, 82, 82, 72, 80, 69, 89, 65, 86, 83, 86, 76, 76, 82, 76, 65, 75, 69, 89, 69, 65, 84, 76, 69, 69, 67, 67, 65, 75, 68, 68, 80, 72, 65, 67, 89, 83, 84, 86, 70, 68, 75, 76, 75, 72, 76, 86, 68, 69, 80, 81, 78, 76, 73, 75, 81, 78, 67, 68, 81, 70, 69, 75, 76, 71, 69, 89, 71, 70, 81, 78, 65, 76, 73, 86, 82, 89, 84, 82, 75, 86, 80, 81, 86, 83, 84, 80, 84, 76, 86, 69, 86, 83, 82, 83, 76, 71, 75, 86, 71, 84, 82, 67, 67, 84, 75, 80, 69, 83, 69, 82, 77, 80, 67, 84, 69, 68, 89, 76, 83, 76, 73, 76, 78, 82, 76, 67, 86, 76, 72, 69, 75, 84, 80, 86, 83, 69, 75, 86, 84, 75, 67, 67, 84, 69, 83, 76, 86, 78, 82, 82, 80, 67, 70, 83, 65, 76, 84, 80, 68, 69, 84, 89, 86, 80, 75, 65, 70, 68, 69, 75, 76, 70, 84, 70, 72, 65, 68, 73, 67, 84, 76, 80, 68, 84, 69, 75, 81, 73, 75, 75, 81, 84, 65, 76, 86, 69, 76, 76, 75, 72, 75, 80, 75, 65, 84, 69, 69, 81, 76, 75, 84, 86, 77, 69, 78, 70, 86, 65, 70, 86, 68, 75, 67, 67, 65, 65, 68, 68, 75, 69, 65, 67, 70, 65, 86, 69, 71, 80, 75, 76, 86, 86, 83, 84, 81, 84, 65, 76, 65], taxonomy: \"9913\" }]");
    }

    #[test]
    fn equality_list() {
        let x = vec![gapdh(), bsa()];
        let y = vec![gapdh(), bsa()];
        let z = vec![gapdh(), gapdh()];
        assert_eq!(x, y);
        assert_ne!(x, z);
        assert_ne!(y, z);
    }

    #[test]
    fn properties_list() {
        // initial check
        let x = vec![gapdh(), Record::new()];
        let mut y = vec![gapdh(), bsa()];
        assert!(!x.is_valid());
        assert!(!x.is_complete());
        assert!(y.is_valid());
        assert!(y.is_complete());
        assert_eq!(x.estimate_fasta_size(), 494);
        assert_eq!(y.estimate_fasta_size(), 1143);

        // remove a necessary qualifier for complete
        y[1].proteome = String::new();
        assert!(y.is_valid());
        assert!(!y.is_complete());
        assert_eq!(y.estimate_fasta_size(), 1143);

        // remove a necessary qualifier for valid
        y[1].sequence_version = 0;
        assert!(!y.is_valid());
        assert!(!y.is_complete());
        assert_eq!(y.estimate_fasta_size(), 1143);
    }

    #[cfg(feature = "fasta")]
    #[test]
    fn fasta_list() {
        let v: RecordList = vec![gapdh(), bsa()];

        // to_fasta (valid, 2 items)
        let x = v.to_fasta_string().unwrap();
        assert_eq!(x, GAPDH_BSA_FASTA);

        let mut buf: BufferType = vec![];
        v.to_fasta_strict(&mut Cursor::new(&mut buf)).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), GAPDH_BSA_FASTA);

        let mut buf: BufferType = vec![];
        v.to_fasta_lenient(&mut Cursor::new(&mut buf)).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), GAPDH_BSA_FASTA);

        // from_fasta (valid, 2 items)
        let y = RecordList::from_fasta_string(&x).unwrap();
        assert_eq!(y, RecordList::from_fasta_strict(&mut Cursor::new(&x)).unwrap());
        assert_eq!(y, RecordList::from_fasta_lenient(&mut Cursor::new(&x)).unwrap());

        // completeness check
        incomplete_list_eq(&v, &y);

        // to_fasta (empty)
        let v: RecordList = vec![];
        let x = v.to_fasta_string().unwrap();
        assert_eq!(x, "");

        let mut buf: BufferType = vec![];
        v.to_fasta_strict(&mut Cursor::new(&mut buf)).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "");

        let mut buf: BufferType = vec![];
        v.to_fasta_lenient(&mut Cursor::new(&mut buf)).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "");

        // from_fasta (empty)
        let y = RecordList::from_fasta_string(&x).unwrap();
        assert_eq!(y, RecordList::from_fasta_strict(&mut Cursor::new(&x)).unwrap());
        assert_eq!(y, RecordList::from_fasta_lenient(&mut Cursor::new(&x)).unwrap());
        assert_eq!(y.len(), 0);

        // to_fasta (1 empty)
        let v: RecordList = vec![Record::new()];
        let x = v.to_fasta_string().unwrap();
        assert_eq!(x, EMPTY_FASTA);

        let mut buf: BufferType = vec![];
        assert!(v.to_fasta_strict(&mut Cursor::new(&mut buf)).is_err());
        assert!(v.to_fasta_lenient(&mut Cursor::new(&mut buf)).is_ok());
        assert_eq!(String::from_utf8(buf).unwrap(), "");

        // from_fasta (1 empty)
        let y = RecordList::from_fasta_string(&x).unwrap();
        assert!(RecordList::from_fasta_strict(&mut Cursor::new(&x)).is_err());
        assert!(RecordList::from_fasta_lenient(&mut Cursor::new(&x)).is_ok());
        assert_eq!(v, y);

        // to_fasta (1 valid, 1 empty)
        let v: RecordList = vec![gapdh(), Record::new()];
        let x = v.to_fasta_string().unwrap();
        assert_eq!(x, GAPDH_EMPTY_FASTA);

        let mut buf: BufferType = vec![];
        assert!(v.to_fasta_strict(&mut Cursor::new(&mut buf)).is_err());
        v.to_fasta_lenient(&mut Cursor::new(&mut buf)).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), GAPDH_FASTA);

        // from_fasta (1 valid, 1 empty)
        let y = RecordList::from_fasta_string(&x).unwrap();
        assert!(RecordList::from_fasta_strict(&mut Cursor::new(&x)).is_err());
        let z = RecordList::from_fasta_lenient(&mut Cursor::new(&x)).unwrap();
        incomplete_eq(&v[0], &y[0]);
        incomplete_eq(&v[0], &z[0]);
        assert_eq!(v[1], y[1]);
        assert_eq!(z.len(), 1);
    }

    #[cfg(feature = "csv")]
    #[test]
    fn csv_list() {
        let v: RecordList = vec![gapdh(), bsa()];

        // to_csv (valid, 2 items)
        let x = v.to_csv_string(b'\t').unwrap();
        assert_eq!(x, GAPDH_BSA_CSV_TAB);

        let mut buf: BufferType = vec![];
        v.to_csv_strict(&mut Cursor::new(&mut buf), b'\t').unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), GAPDH_BSA_CSV_TAB);

        let mut buf: BufferType = vec![];
        v.to_csv_lenient(&mut Cursor::new(&mut buf), b'\t').unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), GAPDH_BSA_CSV_TAB);

        // from_csv (valid, 2 items)
        let y = RecordList::from_csv_string(&x, b'\t').unwrap();
        assert_eq!(y, RecordList::from_csv_strict(&mut Cursor::new(&x), b'\t').unwrap());
        assert_eq!(y, RecordList::from_csv_lenient(&mut Cursor::new(&x), b'\t').unwrap());

        // completeness check
        assert_eq!(v, y);

        // to_csv (empty)
        let v: RecordList = vec![];
        let x = v.to_csv_string(b'\t').unwrap();
        assert_eq!(x, HEADER_CSV_TAB);

        let mut buf: BufferType = vec![];
        v.to_csv_strict(&mut Cursor::new(&mut buf), b'\t').unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), HEADER_CSV_TAB);

        let mut buf: BufferType = vec![];
        v.to_csv_lenient(&mut Cursor::new(&mut buf), b'\t').unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), HEADER_CSV_TAB);

        // from_csv (empty)
        let y = RecordList::from_csv_string(&x, b'\t').unwrap();
        assert_eq!(y, RecordList::from_csv_strict(&mut Cursor::new(&x), b'\t').unwrap());
        assert_eq!(y, RecordList::from_csv_lenient(&mut Cursor::new(&x), b'\t').unwrap());
        assert_eq!(y.len(), 0);

        // to_csv (1 empty)
        let v: RecordList = vec![Record::new()];
        let x = v.to_csv_string(b'\t').unwrap();
        assert_eq!(x, EMPTY_CSV_TAB);

        let mut buf: BufferType = vec![];
        assert!(v.to_csv_strict(&mut Cursor::new(&mut buf), b'\t').is_err());
        buf.clear();
        assert!(v.to_csv_lenient(&mut Cursor::new(&mut buf), b'\t').is_ok());
        assert_eq!(String::from_utf8(buf).unwrap(), HEADER_CSV_TAB);

        // from_csv (1 empty)
        let y = RecordList::from_csv_string(&x, b'\t').unwrap();
        assert!(RecordList::from_csv_strict(&mut Cursor::new(&x), b'\t').is_err());
        assert!(RecordList::from_csv_lenient(&mut Cursor::new(&x), b'\t').is_ok());
        assert_eq!(v, y);

        // to_csv (1 valid, 1 empty)
        let v: RecordList = vec![gapdh(), Record::new()];
        let x = v.to_csv_string(b'\t').unwrap();
        assert_eq!(x, GAPDH_EMPTY_CSV_TAB);

        let mut buf: BufferType = vec![];
        assert!(v.to_csv_strict(&mut Cursor::new(&mut buf), b'\t').is_err());
        v.to_csv_lenient(&mut Cursor::new(&mut buf), b'\t').unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), GAPDH_CSV_TAB);

        // from_csv (1 valid, 1 empty)
        let y = RecordList::from_csv_string(&x, b'\t').unwrap();
        assert!(RecordList::from_csv_strict(&mut Cursor::new(&x), b'\t').is_err());
        let z = RecordList::from_csv_lenient(&mut Cursor::new(&x), b'\t').unwrap();
        assert_eq!(&v[0], &y[0]);
        assert_eq!(&v[0], &z[0]);
        assert_eq!(v[1], y[1]);
        assert_eq!(z.len(), 1);
    }

    // TODO(ahuszagh)
    //  Add XML

    #[cfg(feature = "fasta")]
    fn fasta_dir() -> PathBuf {
        let mut dir = testdata_dir();
        dir.push("uniprot/fasta");
        dir
    }

    #[cfg(feature = "fasta")]
    #[test]
    #[ignore]
    fn list_fasta_test() {
        let mut path = fasta_dir();
        path.push("list.fasta");
        let mut reader = BufReader::new(File::open(path).unwrap());

        let expected = vec!["A0A2U8RNL1", "P02769", "P46406", "Q53FP0"];
        let v = RecordList::from_fasta(&mut reader).unwrap();
        let actual: Vec<String> = v.iter().map(|r| r.id.clone()).collect();
        assert_eq!(expected, actual);
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
    fn list_csv_test() {
        let mut path = csv_dir();
        path.push("list.csv");
        let mut reader = File::open(path).unwrap();

        let expected = vec!["A0A2U8RNL1", "P02769", "P46406", "Q53FP0"];
        let v = RecordList::from_csv(&mut reader, b'\t').unwrap();
        let actual: Vec<String> = v.iter().map(|r| r.id.clone()).collect();
        assert_eq!(expected, actual);
    }

    // TODO(ahuszagh)
    //  Implement the XML unittests.
}
