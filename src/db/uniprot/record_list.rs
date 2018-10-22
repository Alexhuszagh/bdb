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
        assert_eq!(l, "[Record { sequence_version: 3, protein_evidence: ProteinLevel, mass: 35780, length: 333, gene: \"GAPDH\", id: \"P46406\", mnemonic: \"G3P_RABIT\", name: \"Glyceraldehyde-3-phosphate dehydrogenase\", organism: \"Oryctolagus cuniculus\", proteome: \"UP000001811\", sequence: \"MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\", taxonomy: \"9986\" }, Record { sequence_version: 4, protein_evidence: ProteinLevel, mass: 69293, length: 607, gene: \"ALB\", id: \"P02769\", mnemonic: \"ALBU_BOVIN\", name: \"Serum albumin\", organism: \"Bos taurus\", proteome: \"UP000009136\", sequence: \"MKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA\", taxonomy: \"9913\" }]");
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
