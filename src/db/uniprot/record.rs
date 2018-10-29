//! Model for UniProt protein definitions.

use super::evidence::ProteinEvidence;

/// Enumerated values for Record fields.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RecordField {
    SequenceVersion,
    ProteinEvidence,
    Mass,
    Length,
    Gene,
    Id,
    Mnemonic,
    Name,
    Organism,
    Proteome,
    Sequence,
    Taxonomy,
    Reviewed
}

/// Model for a single record from a UniProt KB query.
///
/// Record including core query fields for a given UniProt identifier.
/// The query fields are defined [here](http://www.uniprot.org/help/query-fields).
///
/// # Advanced
///
/// The following is a mapping of the UniProt form-encoded keys, struct
/// field names, and UniProt displayed column names.
/// Despite the name correspondence, the information may not be a
/// identical in one format or another, for example,
/// [`protein_evidence`] is an enumeration, while in a displayed
/// column it's a string, and in FASTA it's a numerical identifier.
/// [`ProteinEvidence.ProteinLevel`] is the same as `"Evidence at protein
/// level"` which is the same as `1`.
///
/// | Field Name           | Form-Encoded Key     | Displayed Column       |
/// |----------------------|----------------------|------------------------|
/// | [`sequence_version`] | version(sequence)    | Sequence version       |
/// | [`protein_evidence`] | existence            | Protein existence      |
/// | [`mass`]             | mass                 | Mass                   |
/// | [`length`]           | length               | Length                 |
/// | [`gene`]             | genes(PREFERRED)     | Gene names  (primary ) |
/// | [`id`]               | id                   | Entry                  |
/// | [`mnemonic`]         | entry name           | Entry name             |
/// | [`name`]             | protein names        | Protein names          |
/// | [`organism`]         | organism             | Organism               |
/// | [`proteome`]         | proteome             | Proteomes              |
/// | [`sequence`]         | sequence             | Sequence               |
/// | [`taxonomy`]         | organism-id          | Organism ID            |
///
/// [`sequence_version`]: struct.Record.html#structfield.sequence_version
/// [`protein_evidence`]: struct.Record.html#structfield.protein_evidence
/// [`mass`]: struct.Record.html#structfield.mass
/// [`length`]: struct.Record.html#structfield.length
/// [`gene`]: struct.Record.html#structfield.gene
/// [`id`]: struct.Record.html#structfield.id
/// [`mnemonic`]: struct.Record.html#structfield.mnemonic
/// [`name`]: struct.Record.html#structfield.name
/// [`organism`]: struct.Record.html#structfield.organism
/// [`proteome`]: struct.Record.html#structfield.proteome
/// [`sequence`]: struct.Record.html#structfield.sequence
/// [`taxonomy`]: struct.Record.html#structfield.taxonomy
/// [`ProteinEvidence.ProteinLevel`]: enum.ProteinEvidence.html#variant.ProteinLevel

// Extra information hidden from the documentation, for developers.
//  Notes:
//      `sequence_version`:
//          Simple integer in all variants.
//
//      `protein_evidence
//          Enumerated value, which appears as a string or integer, with
//          the mapping defined in `ProteinEvidence` and
//          `ProteinEvidence::verbose()`.
//
//      `mass`:
//          Simple integer in all variants.
//
//      `length`:
//          Simple integer in all variants.
//
//      `gene`:
//          Identifier for the gene name. Although normally alpha-numeric,
//          the gene name may include rather esoteric elements. An analysis
//          of the whole human proteome also includes the following
//          identifiers, as a regex character group: "[-_ /*.@:();'$+]".
//          These identifiers are rather rare, from 4% of gene names to
//          being present in almost 1 in a million gene names.
//
//      `id`:
//          Accession number as a string.
//
//      `mnemonic`:
//          Mnemonic identifier as a string.
//
//      `name`:
//          Name for the protein (ex. Glyceraldehyde-3-phosphate
//          dehydrogenase). However, UniProt frequently spits out
//          more than one possible protein name, with each subsequent
//          name enclosed in parentheses (ex. "Glyceraldehyde-3-phosphate
//          dehydrogenase (GAPDH) (EC 1.2.1.12) (Peptidyl-cysteine
//          S-nitrosylase GAPDH) (EC 2.6.99.-)").
//
//      `organism`:
//          Species name (with an optional common name in parentheses).
//          BDB considers the common name superfluous, and therefore
//          removes it from all records fetched from external queries.
//          Strain information, which is also enclosed in parentheses,
//          however, should not be removed.
//
//      `proteome`:
//          Proteomes include a proteome identifier and an optional
//          proteome location, for example, "UP000001811: Unplaced",
//          "UP000001114: Chromosome", and "UP000001811" are all valid
//          values. We discard the location, and solely store the proteome
//          identifier.
//
//      `sequence`:
//          Aminoacid sequence of the protein, as a string.
//
//      `taxonomy`:
//          Numerical identifier for the species, described by "name".
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Record {
    /// Numerical identifier for protein version.
    ///
    /// Value starts from 1, and is incremented for each revision of the protein.
    pub sequence_version: u8,
    /// Enumeration for the strength of evidence for the protein existence.
    pub protein_evidence: ProteinEvidence,
    /// Mass of the protein.
    pub mass: u64,
    /// Protein sequence length.
    pub length: u32,
    /// HGNC Gene name.
    pub gene: String,
    /// Accession number (randomly assigned identifier).
    pub id: String,
    /// Entry name (readable identifier).
    pub mnemonic: String,
    /// Protein name.
    pub name: String,
    /// Readable organism name.
    pub organism: String,
    /// UniProt proteome identifier.
    pub proteome: String,
    /// Protein aminoacid sequence.
    pub sequence: Vec<u8>,
    /// Taxonomic identifier.
    pub taxonomy: String,
    /// Whether the protein has been manually reviewed.
    pub reviewed: bool,
}


impl Record {
    /// Create new, empty UniProt record.
    #[inline]
    pub fn new() -> Self {
        Record {
            sequence_version: 0,
            protein_evidence: ProteinEvidence::Unknown,
            mass: 0,
            length: 0,
            gene: String::new(),
            id: String::new(),
            mnemonic: String::new(),
            name: String::new(),
            organism: String::new(),
            proteome: String::new(),
            sequence: vec![],
            taxonomy: String::new(),
            reviewed: false,
        }
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use traits::*;
    use super::*;
    use super::super::test::*;

    #[test]
    fn debug_record_test() {
        let text = format!("{:?}", gapdh());
        assert_eq!(text, "Record { sequence_version: 3, protein_evidence: ProteinLevel, mass: 35780, length: 333, gene: \"GAPDH\", id: \"P46406\", mnemonic: \"G3P_RABIT\", name: \"Glyceraldehyde-3-phosphate dehydrogenase\", organism: \"Oryctolagus cuniculus\", proteome: \"UP000001811\", sequence: [77, 86, 75, 86, 71, 86, 78, 71, 70, 71, 82, 73, 71, 82, 76, 86, 84, 82, 65, 65, 70, 78, 83, 71, 75, 86, 68, 86, 86, 65, 73, 78, 68, 80, 70, 73, 68, 76, 72, 89, 77, 86, 89, 77, 70, 81, 89, 68, 83, 84, 72, 71, 75, 70, 72, 71, 84, 86, 75, 65, 69, 78, 71, 75, 76, 86, 73, 78, 71, 75, 65, 73, 84, 73, 70, 81, 69, 82, 68, 80, 65, 78, 73, 75, 87, 71, 68, 65, 71, 65, 69, 89, 86, 86, 69, 83, 84, 71, 86, 70, 84, 84, 77, 69, 75, 65, 71, 65, 72, 76, 75, 71, 71, 65, 75, 82, 86, 73, 73, 83, 65, 80, 83, 65, 68, 65, 80, 77, 70, 86, 77, 71, 86, 78, 72, 69, 75, 89, 68, 78, 83, 76, 75, 73, 86, 83, 78, 65, 83, 67, 84, 84, 78, 67, 76, 65, 80, 76, 65, 75, 86, 73, 72, 68, 72, 70, 71, 73, 86, 69, 71, 76, 77, 84, 84, 86, 72, 65, 73, 84, 65, 84, 81, 75, 84, 86, 68, 71, 80, 83, 71, 75, 76, 87, 82, 68, 71, 82, 71, 65, 65, 81, 78, 73, 73, 80, 65, 83, 84, 71, 65, 65, 75, 65, 86, 71, 75, 86, 73, 80, 69, 76, 78, 71, 75, 76, 84, 71, 77, 65, 70, 82, 86, 80, 84, 80, 78, 86, 83, 86, 86, 68, 76, 84, 67, 82, 76, 69, 75, 65, 65, 75, 89, 68, 68, 73, 75, 75, 86, 86, 75, 81, 65, 83, 69, 71, 80, 76, 75, 71, 73, 76, 71, 89, 84, 69, 68, 81, 86, 86, 83, 67, 68, 70, 78, 83, 65, 84, 72, 83, 83, 84, 70, 68, 65, 71, 65, 71, 73, 65, 76, 78, 68, 72, 70, 86, 75, 76, 73, 83, 87, 89, 68, 78, 69, 70, 71, 89, 83, 78, 82, 86, 86, 68, 76, 77, 86, 72, 77, 65, 83, 75, 69], taxonomy: \"9986\", reviewed: true }");

        let text = format!("{:?}", bsa());
        assert_eq!(text, "Record { sequence_version: 4, protein_evidence: ProteinLevel, mass: 69293, length: 607, gene: \"ALB\", id: \"P02769\", mnemonic: \"ALBU_BOVIN\", name: \"Serum albumin\", organism: \"Bos taurus\", proteome: \"UP000009136\", sequence: [77, 75, 87, 86, 84, 70, 73, 83, 76, 76, 76, 76, 70, 83, 83, 65, 89, 83, 82, 71, 86, 70, 82, 82, 68, 84, 72, 75, 83, 69, 73, 65, 72, 82, 70, 75, 68, 76, 71, 69, 69, 72, 70, 75, 71, 76, 86, 76, 73, 65, 70, 83, 81, 89, 76, 81, 81, 67, 80, 70, 68, 69, 72, 86, 75, 76, 86, 78, 69, 76, 84, 69, 70, 65, 75, 84, 67, 86, 65, 68, 69, 83, 72, 65, 71, 67, 69, 75, 83, 76, 72, 84, 76, 70, 71, 68, 69, 76, 67, 75, 86, 65, 83, 76, 82, 69, 84, 89, 71, 68, 77, 65, 68, 67, 67, 69, 75, 81, 69, 80, 69, 82, 78, 69, 67, 70, 76, 83, 72, 75, 68, 68, 83, 80, 68, 76, 80, 75, 76, 75, 80, 68, 80, 78, 84, 76, 67, 68, 69, 70, 75, 65, 68, 69, 75, 75, 70, 87, 71, 75, 89, 76, 89, 69, 73, 65, 82, 82, 72, 80, 89, 70, 89, 65, 80, 69, 76, 76, 89, 89, 65, 78, 75, 89, 78, 71, 86, 70, 81, 69, 67, 67, 81, 65, 69, 68, 75, 71, 65, 67, 76, 76, 80, 75, 73, 69, 84, 77, 82, 69, 75, 86, 76, 65, 83, 83, 65, 82, 81, 82, 76, 82, 67, 65, 83, 73, 81, 75, 70, 71, 69, 82, 65, 76, 75, 65, 87, 83, 86, 65, 82, 76, 83, 81, 75, 70, 80, 75, 65, 69, 70, 86, 69, 86, 84, 75, 76, 86, 84, 68, 76, 84, 75, 86, 72, 75, 69, 67, 67, 72, 71, 68, 76, 76, 69, 67, 65, 68, 68, 82, 65, 68, 76, 65, 75, 89, 73, 67, 68, 78, 81, 68, 84, 73, 83, 83, 75, 76, 75, 69, 67, 67, 68, 75, 80, 76, 76, 69, 75, 83, 72, 67, 73, 65, 69, 86, 69, 75, 68, 65, 73, 80, 69, 78, 76, 80, 80, 76, 84, 65, 68, 70, 65, 69, 68, 75, 68, 86, 67, 75, 78, 89, 81, 69, 65, 75, 68, 65, 70, 76, 71, 83, 70, 76, 89, 69, 89, 83, 82, 82, 72, 80, 69, 89, 65, 86, 83, 86, 76, 76, 82, 76, 65, 75, 69, 89, 69, 65, 84, 76, 69, 69, 67, 67, 65, 75, 68, 68, 80, 72, 65, 67, 89, 83, 84, 86, 70, 68, 75, 76, 75, 72, 76, 86, 68, 69, 80, 81, 78, 76, 73, 75, 81, 78, 67, 68, 81, 70, 69, 75, 76, 71, 69, 89, 71, 70, 81, 78, 65, 76, 73, 86, 82, 89, 84, 82, 75, 86, 80, 81, 86, 83, 84, 80, 84, 76, 86, 69, 86, 83, 82, 83, 76, 71, 75, 86, 71, 84, 82, 67, 67, 84, 75, 80, 69, 83, 69, 82, 77, 80, 67, 84, 69, 68, 89, 76, 83, 76, 73, 76, 78, 82, 76, 67, 86, 76, 72, 69, 75, 84, 80, 86, 83, 69, 75, 86, 84, 75, 67, 67, 84, 69, 83, 76, 86, 78, 82, 82, 80, 67, 70, 83, 65, 76, 84, 80, 68, 69, 84, 89, 86, 80, 75, 65, 70, 68, 69, 75, 76, 70, 84, 70, 72, 65, 68, 73, 67, 84, 76, 80, 68, 84, 69, 75, 81, 73, 75, 75, 81, 84, 65, 76, 86, 69, 76, 76, 75, 72, 75, 80, 75, 65, 84, 69, 69, 81, 76, 75, 84, 86, 77, 69, 78, 70, 86, 65, 70, 86, 68, 75, 67, 67, 65, 65, 68, 68, 75, 69, 65, 67, 70, 65, 86, 69, 71, 80, 75, 76, 86, 86, 83, 84, 81, 84, 65, 76, 65], taxonomy: \"9913\", reviewed: true }");
    }

    #[test]
    fn equality_record_test() {
        let x = gapdh();
        let y = gapdh();
        let z = bsa();
        assert_eq!(x, y);
        assert_ne!(x, z);
        assert_ne!(y, z);
    }

    #[test]
    fn properties_record_test() {
        // test various permutations that can lead to
        // invalid or incomplete identifications
        let g1 = gapdh();
        let mut g2 = g1.clone();
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g1.estimate_fasta_size(), 458);
        assert_eq!(g2.estimate_fasta_size(), 458);

        // check keeping the protein valid but make it incomplete
        g2.proteome = String::new();
        assert!(g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 458);
        g2.proteome = g1.proteome.clone();

        g2.taxonomy = String::new();
        assert!(g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 454);
        g2.taxonomy = g1.taxonomy.clone();

        // check replacing items with valid, but different data
        g2.sequence_version = 1;
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 458);
        g2.sequence_version = g1.sequence_version;

        g2.protein_evidence = ProteinEvidence::Inferred;
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 458);
        g2.protein_evidence = g1.protein_evidence;

        g2.mass = 64234;
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 458);
        g2.mass = g1.mass;

        g2.sequence = g2.sequence[0..200].to_vec();
        g2.length = 200;
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 325);
        g2.sequence = g1.sequence.clone();
        g2.length = g1.length;

        g2.gene = String::from("HIST1H1A");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 461);
        g2.gene = g1.gene.clone();

        g2.id = String::from("A0A022YWF9");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 462);
        g2.id = g1.id.clone();

        g2.id = String::from("A2BC19");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 458);
        g2.id = g1.id.clone();

        g2.mnemonic = String::from("H11_HUMAN");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 458);
        g2.mnemonic = g1.mnemonic.clone();

        g2.name = String::from("Histone H1.1");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 430);
        g2.name = g1.name.clone();

        g2.organism = String::from("Homo sapiens");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 449);
        g2.organism = g1.organism.clone();

        g2.proteome = String::from("UP000005640");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 458);
        g2.proteome = g1.proteome.clone();

        g2.taxonomy = String::from("9606");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 458);
        g2.taxonomy = g1.taxonomy.clone();

        // check replacing items with invalid data
        g2.sequence_version = 0;
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 458);
        g2.sequence_version = g1.sequence_version;

        g2.protein_evidence = ProteinEvidence::Unknown;
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 458);
        g2.protein_evidence = g1.protein_evidence;

        g2.mass = 0;
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 458);
        g2.mass = g1.mass;

        g2.length = 334;
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 458);
        g2.length = g1.length;

        g2.gene = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 453);
        g2.gene = g1.gene.clone();

        g2.id = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 452);
        g2.id = g1.id.clone();

        g2.mnemonic = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 449);
        g2.mnemonic = g1.mnemonic.clone();

        g2.name = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 418);
        g2.name = g1.name.clone();

        g2.organism = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 437);
        g2.organism = g1.organism.clone();

        g2.sequence = vec![];
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 125);
        g2.sequence = g1.sequence.clone();
    }

    #[cfg(feature = "fasta")]
    #[test]
    fn fasta_record_test() {
        // gapdh
        let p = gapdh();
        let x = p.to_fasta_string().unwrap();
        assert_eq!(x, GAPDH_FASTA);
        let y = Record::from_fasta_string(&x).unwrap();
        incomplete_eq(&p, &y);

        // bsa
        let p = bsa();
        let x = p.to_fasta_string().unwrap();
        assert_eq!(x, BSA_FASTA);
        let y = Record::from_fasta_string(&x).unwrap();
        incomplete_eq(&p, &y);

        // empty
        let p = Record::new();
        let x = p.to_fasta_string().unwrap();
        assert_eq!(x, EMPTY_FASTA);
        let y = Record::from_fasta_string(&x).unwrap();
        assert_eq!(p, y);
    }

    #[cfg(feature = "csv")]
    #[test]
    fn csv_record_test() {
        // gapdh
        let p = gapdh();
        let x = p.to_csv_string(b'\t').unwrap();
        assert_eq!(x, GAPDH_CSV_TAB);
        let x = p.to_csv_string(b',').unwrap();
        assert_eq!(x, GAPDH_CSV_COMMA);
        let y = Record::from_csv_string(&x, b',').unwrap();
        assert_eq!(p, y);

        // bsa
        let p = bsa();
        let x = p.to_csv_string(b'\t').unwrap();
        assert_eq!(x, BSA_CSV_TAB);
        let x = p.to_csv_string(b',').unwrap();
        assert_eq!(x, BSA_CSV_COMMA);
        let y = Record::from_csv_string(&x, b',').unwrap();
        assert_eq!(p, y);

        // empty
        let p = Record::new();
        let x = p.to_csv_string(b'\t').unwrap();
        assert_eq!(x, EMPTY_CSV_TAB);
        let x = p.to_csv_string(b',').unwrap();
        assert_eq!(x, EMPTY_CSV_COMMA);
        let y = Record::from_csv_string(&x, b',').unwrap();
        assert_eq!(p, y);
    }

    #[cfg(feature = "xml")]
    #[test]
    fn xml_record_test() {
        // gapdh
        let p = gapdh();
        let x = p.to_xml_string().unwrap();
        let y = Record::from_xml_string(&x).unwrap();
        assert_eq!(p, y);

        // bsa
        let p = bsa();
        let x = p.to_xml_string().unwrap();
        let y = Record::from_xml_string(&x).unwrap();
        assert_eq!(p, y);

        // empty
        let p = Record::new();
        let x = p.to_xml_string().unwrap();
        let y = Record::from_xml_string(&x).unwrap();
        assert_eq!(p, y);
    }
}
