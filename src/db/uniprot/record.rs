//! Model for UniProt protein definitions.

use regex::Regex;
use std::fmt;
//use serde_json;

use traits;

/// Identifier for the evidence type for protein existence.
///
/// An identifier used by biological databases for the level of evidence
/// that supports a protein's existence. Strong evidence includes
/// evidence at the protein level, while weaker evidence is evidence
/// at the transcript (or mRNA) level. Weak evidence is inferred from
/// homology from similar species. Curated protein databases frequently
/// only include proteins identified at the protein level.
///
/// `Unknown` is a custom value for invalid entries, or those with yet-
/// to-be annotated protein evidence scores.
///
/// More documentation can be found at:
///     https://www.uniprot.org/help/protein_existence
///
enum_number!(ProteinEvidence {
    ProteinLevel = 1,
    TranscriptLevel = 2,
    Inferred = 3,
    Predicted = 4,
    Unknown = 5,
});


/// Convert enumerated value for ProteinEvidence to verbose text.
pub fn protein_evidence_verbose(evidence: ProteinEvidence) -> &'static str {
    match evidence {
        ProteinEvidence::ProteinLevel       => "Evidence at protein level",
        ProteinEvidence::TranscriptLevel    => "Evidence at transcript level",
        ProteinEvidence::Inferred           => "Inferred from homology",
        ProteinEvidence::Predicted          => "Predicted",
        ProteinEvidence::Unknown            => "Unknown evidence (BDB-only designation)",
    }
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
//          `protein_evidence_verbose`.
//
//      `mass`:
//          Simple integer in all variants.
//
//      `length`:
//          Simple integer in all variants.
//
//      `gene`:
//          TODO(ahuszagh) [I believe this frequently gives more than
//          one gene name, confirm with the unannotated human proteome.
//          If so, designate a regex for filtering from external queries.]
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
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
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
    pub sequence: String,
    /// Taxonomic identifier.
    pub taxonomy: String,
}


impl Record {
    /// Create new, empty UniProt record.
    pub fn new() -> Record {
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
            sequence: String::new(),
            taxonomy: String::new(),
        }
    }
}


impl traits::Valid for Record {
    fn is_valid(&self) -> bool {
        {
            self.sequence_version >= 1 &&
            self.protein_evidence < ProteinEvidence::Unknown &&
            self.mass > 0 &&
            self.length as usize == self.sequence.len() &&
            self.sequence.len() > 0 &&
            self.gene.len() > 0 &&
            self.name.len() > 0 &&
            AccessionRegex::validate().is_match(&self.id) &&
            MnemonicRegex::validate().is_match(&self.mnemonic) &&
            //ORGANISM_REGEX.is_match(&self.organism) &&
            AminoacidRegex::validate().is_match(&self.sequence)
        }
    }
}

// PRIVATE
// -------

// REGULAR EXPRESSIONS

/// Regular expressions for UniProt record fields.
trait FieldRegex {
    /// Validate a field.
    fn validate() -> &'static Regex;
    /// Extract a field from external data.
    fn extract() -> &'static Regex;
}

fn new_regex(pattern: &'static str) -> Regex {
    Regex::new(pattern).unwrap()
}

macro_rules! lazy_regex {
    ($str:expr) => (lazy_static! {
        static ref REGEX: Regex = new_regex($str);
    })
}

// ACCESSION

/// Regular expression to validate accession numbers.
///
/// Derived from [here](https://www.uniprot.org/help/accession_numbers).
struct AccessionRegex;

impl FieldRegex for AccessionRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(r"(?x)
            \A
            (?:
                [OPQ][0-9][A-Z0-9]{3}[0-9]|
                [A-NR-Z][0-9](?:[A-Z][A-Z0-9]{2}[0-9]){1,2}
            )
            \z
        ");
        &REGEX
    }

    fn extract() -> &'static Regex {
        lazy_regex!(r"(?x)
            \A
            # Group 1, Accession Number
            (
                [OPQ][0-9][A-Z0-9]{3}[0-9]|
                [A-NR-Z][0-9](?:[A-Z][A-Z0-9]{2}[0-9]){1,2}
            )
            \z
        ");
        &REGEX
    }
}

// MNEMONIC

/// Regular expression to validate mnemonic identifiers.
struct MnemonicRegex;

impl FieldRegex for MnemonicRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(r"(?x)
            \A
            (?:
                [a-zA-Z0-9]{1,5}_[a-zA-Z0-9]{1,5}
            )
            \z
        ");
        &REGEX
    }

    fn extract() -> &'static Regex {
        lazy_regex!(r"(?x)
            \A
            # Group 1, Mnemonic Identifier
            (
                [a-zA-Z0-9]{1,5}_[a-zA-Z0-9]{1,5}
            )
            \z
        ");
        &REGEX
    }
}

// ORGANISM

// TODO(ahuszagh)
//      Restore
//macro_rules! organism_pattern {
//    () => (r"(?x)
//        \A
//        (?P<genus>[A-Z][a-z]+)      # Genus (generic) name for species
//        \s                          # Word boundary
//        (?P<species>[A-Z][a-z]+)    # Specific name for species
//
//        # TODO: implement here...
//        #   Need the strain catcher
//    ")
//}
//
//struct OrganismRegex;
//
//impl FieldRegex for OrganismRegex {
//    fn validate() -> &'static Regex {
//        // TODO(ahuszagh)
//        //  Concat to a \z
//        lazy_regex!(organism_pattern!());
//        &REGEX
//    }
//
//    fn extract() -> &'static Regex {
//        lazy_regex!(organism_pattern!());
//        &REGEX
//    }
//}

// AMINOACID

/// Regular expression to validate aminoacid sequences.
struct AminoacidRegex;

impl FieldRegex for AminoacidRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(r"(?x)
            \A
            (?:
                [ABCDEFGHIJKLMNPQRSTVWXYZabcdefghijklmnpqrstvwxyz]*
            )
            \z
        ");
        &REGEX
    }

    fn extract() -> &'static Regex {
        lazy_regex!(r"(?x)
            \A
            # Group 1, Aminoacid Sequence
            (
                [ABCDEFGHIJKLMNPQRSTVWXYZabcdefghijklmnpqrstvwxyz]*
            )
            \z
        ");
        &REGEX
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use serde_json;
    use super::*;

    // PROTEIN EVIDENCE
    // Note: Do not test Unknown, it is an implementation detail.

    #[test]
    fn debug_protein_evidence() {
        // ProteinLevel
        let text = format!("{:?}", ProteinEvidence::ProteinLevel);
        assert_eq!(text, "ProteinLevel");

        // TranscriptLevel
        let text = format!("{:?}", ProteinEvidence::TranscriptLevel);
        assert_eq!(text, "TranscriptLevel");

        // Inferred
        let text = format!("{:?}", ProteinEvidence::Inferred);
        assert_eq!(text, "Inferred");

        // Predicted
        let text = format!("{:?}", ProteinEvidence::Predicted);
        assert_eq!(text, "Predicted");
    }

    #[test]
    fn serialize_protein_evidence() {
        // ProteinLevel
        let text = serde_json::to_string(&ProteinEvidence::ProteinLevel).unwrap();
        assert_eq!(text, "1");
        let evidence: ProteinEvidence = serde_json::from_str(&text).unwrap();
        assert_eq!(evidence, ProteinEvidence::ProteinLevel);

        // TranscriptLevel
        let text = serde_json::to_string(&ProteinEvidence::TranscriptLevel).unwrap();
        assert_eq!(text, "2");
        let evidence: ProteinEvidence = serde_json::from_str(&text).unwrap();
        assert_eq!(evidence, ProteinEvidence::TranscriptLevel);

        // Inferred
        let text = serde_json::to_string(&ProteinEvidence::Inferred).unwrap();
        assert_eq!(text, "3");
        let evidence: ProteinEvidence = serde_json::from_str(&text).unwrap();
        assert_eq!(evidence, ProteinEvidence::Inferred);

        // Predicted
        let text = serde_json::to_string(&ProteinEvidence::Predicted).unwrap();
        assert_eq!(text, "4");
        let evidence: ProteinEvidence = serde_json::from_str(&text).unwrap();
        assert_eq!(evidence, ProteinEvidence::Predicted);
    }

    #[test]
    fn protein_evidence_verbose_test() {
        // ProteinLevel
        let text = protein_evidence_verbose(ProteinEvidence::ProteinLevel);
        assert_eq!(text, "Evidence at protein level");

        // TranscriptLevel
        let text = protein_evidence_verbose(ProteinEvidence::TranscriptLevel);
        assert_eq!(text, "Evidence at transcript level");

        // Inferred
        let text = protein_evidence_verbose(ProteinEvidence::Inferred);
        assert_eq!(text, "Inferred from homology");

        // Predicted
        let text = protein_evidence_verbose(ProteinEvidence::Predicted);
        assert_eq!(text, "Predicted");
    }

    // REGULAR EXPRESSIONS

    /// Check regex matches or does not match text.
    fn check_regex<T: FieldRegex>(text: &str, result: bool) {
        assert_eq!(T::validate().is_match(text), result);
        assert_eq!(T::extract().is_match(text), result);
    }

    /// Check regex extracts the desired subgroup.
    fn extract_regex<T: FieldRegex>(text: &str, index: usize, result: &str) {
        let re = T::extract();
        let caps = re.captures(text).unwrap();
        assert_eq!(caps.get(index).unwrap().as_str(), result);
    }

    #[test]
    fn accession_regex() {
        type T = AccessionRegex;

        // empty
        check_regex::<T>("", false);

        // valid
        check_regex::<T>("A2BC19", true);
        check_regex::<T>("P12345", true);
        check_regex::<T>("A0A022YWF9", true);

        // valid - 1 letter
        check_regex::<T>("2BC19", false);
        check_regex::<T>("A2BC1", false);
        check_regex::<T>("0A022YWF9", false);
        check_regex::<T>("A0A022YWF", false);

        // valid + 1 letter
        check_regex::<T>("XA2BC19", false);
        check_regex::<T>("A2BC19X", false);
        check_regex::<T>("XA0A022YWF9", false);
        check_regex::<T>("A0A022YWF9X", false);

        // valid + space
        check_regex::<T>(" A2BC19", false);
        check_regex::<T>("A2BC19 ", false);
        check_regex::<T>(" A0A022YWF9", false);
        check_regex::<T>("A0A022YWF9 ", false);

        // extract
        extract_regex::<T>("A2BC19", 1, "A2BC19");
        extract_regex::<T>("P12345", 1, "P12345");
        extract_regex::<T>("A0A022YWF9", 1, "A0A022YWF9");
    }

    #[test]
    fn mnemonic_regex() {
        // TODO(ahuszagh)
        // Implement...
    }

    #[test]
    fn aminoacid_regex() {
        // TODO(ahuszagh)
        // Implement...
    }

    // TODO(ahuszagh)
    //  Import tests from uniprot.rs
    //  Implement tests of the regular expressions
}
