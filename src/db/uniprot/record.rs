//! Model for UniProt protein definitions.

use regex::{Captures, Regex};
use serde_json;
use std::io::{BufReader, BufWriter, Read, Write};
use std::fmt;

use bio::proteins::{AverageMass, ProteinMass};
use traits::*;
use util::ResultType;
use super::error::{new_boxed_error, UniProtErrorKind};

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
    #[inline]
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


impl Valid for Record {
    fn is_valid(&self) -> bool {
        {
            // Do not try to validate the Organism
            // With virus names being non-standard, it is impossible
            // with an NFA, and extremely time complex otherwise.
            self.sequence_version > 0 &&
            self.protein_evidence < ProteinEvidence::Unknown &&
            self.mass > 0 &&
            self.length as usize == self.sequence.len() &&
            !self.sequence.is_empty() &&
            !self.gene.is_empty() &&
            !self.name.is_empty() &&
            !self.organism.is_empty() &&
            AccessionRegex::validate().is_match(&self.id) &&
            MnemonicRegex::validate().is_match(&self.mnemonic) &&
            AminoacidRegex::validate().is_match(&self.sequence) &&
            (
                self.proteome.is_empty() ||
                ProteomeRegex::validate().is_match(&self.proteome)
            ) &&
            (
                self.taxonomy.is_empty() ||
                TaxonomyRegex::validate().is_match(&self.taxonomy)
            )
        }
    }
}

impl Complete for Record {
    fn is_complete(&self) -> bool {
        {
            self.is_valid() &&
            !self.proteome.is_empty() &&
            !self.taxonomy.is_empty()
        }
    }
}

/// Estimate the size of the resulting FASTA record.
///
/// Use to minimize reallocations while serializing to FASTA.
fn estimate_fasta_size(record: &Record) -> usize {
    /// The approximate sum of all the control vocabulary in the FASTA record.
    const FASTA_VOCABULARY_SIZE: usize = 20;
    FASTA_VOCABULARY_SIZE +
        record.gene.len() +
        record.id.len() +
        record.name.len() +
        record.organism.len() +
        record.sequence.len()
}

/// Convert capture group to `&str`.
#[inline(always)]
fn capture_as_str<'t>(captures: &'t Captures, index: usize) -> &'t str {
    captures.get(index).unwrap().as_str()
}

/// Convert capture group to `String`.
#[inline(always)]
fn capture_as_string(captures: &Captures, index: usize) -> String {
    String::from(capture_as_str(captures, index))
}

// TODO(ahuszagh)
//      Restore
impl Fasta for Record {
    fn to_fasta<T: Write>(&self, writer: &mut BufWriter<T>) -> ResultType<()> {
//        // initialize string and avoid reallocations.
//        let size = estimate_fasta_size(self);
//        let mut fasta = String::with_capacity(size);
//
//        // write SwissProt header
//        let evidence = self.protein_evidence as u32;
//        push_strs!(
//            fasta,
//            ">sp|",     &self.id,
//            "|",        &self.mnemonic,
//            " ",        &self.name,
//            " OS=",     &self.organism,
//            " GN=",     &self.gene,
//            " PE=",     &evidence.to_string(),
//            " SV=",     &self.sequence_version.to_string()
//        );
//
//        // write SwissProt sequence, formatted at 60 characters
//        // Start from 1, so we go 1..60, rather than 0..59
//        const SEQUENCE_LINE_LENGTH: usize = 60;
//        let mut i: usize = 1;
//        for c in self.sequence.chars() {
//            match i {
//                1                    => { fasta.push('\n'); i += 1; },
//                SEQUENCE_LINE_LENGTH => { i = 1; },
//                _                    => { i += 1; },
//            }
//            fasta.push(c);
//        }
//
//        Ok(fasta)
        Ok(())
    }

//    fn from_fasta(fasta: &str) -> ResultType<Record> {
//        // split along lines
//        // first line is the header, rest are the sequences
//        // short-circuit if the header is None.
//        let mut lines = fasta.lines();
//        let header = lines.next();
//        if header.is_none() {
//            return Err(new_boxed_error(UniProtErrorKind::InvalidInputData));
//        }
//        let header = header.unwrap();
//
//        // process the header and match it to the FASTA record
//        let captures = FastaHeaderRegex::extract().captures(&header);
//        if captures.is_none() {
//            return Err(new_boxed_error(UniProtErrorKind::InvalidInputData));
//        }
//        let captures = captures.unwrap();
//
//        // initialize the record with header data
//        let pe = capture_as_str(&captures, PE_INDEX);
//        let sv = capture_as_str(&captures, SV_INDEX);
//        let mut record = Record {
//            sequence_version: sv.parse().unwrap(),
//            protein_evidence: serde_json::from_str(pe).unwrap(),
//            mass: 0,
//            length: 0,
//            gene: capture_as_string(&captures, GENE_INDEX),
//            id: capture_as_string(&captures, ACCESSION_INDEX),
//            mnemonic: capture_as_string(&captures, MNEMONIC_INDEX),
//            name: capture_as_string(&captures, NAME_INDEX),
//            organism: capture_as_string(&captures, ORGANISM_INDEX),
//
//            // unused fields in header
//            proteome: String::new(),
//            sequence: String::new(),
//            taxonomy: String::new(),
//        };
//
//        // add sequence data to the FASTA sequence
//        for line in lines {
//            record.sequence.push_str(&line);
//        }
//
//        // calculate the protein length and mass
//        record.length = record.sequence.len() as u32;
//        let mass = AverageMass::protein_sequence_mass(record.sequence.as_bytes());
//        record.mass = mass.round() as u64;
//
//        Ok(record)
//    }
}

// PRIVATE
// -------

// REGULAR EXPRESSIONS

/// Regular expressions for UniProt record fields.
pub trait FieldRegex {
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
                # Group 2, Protein Name
                (
                    [a-zA-Z0-9]{1,5}
                )
                _
                # Group 3, Species Name
                (
                    [a-zA-Z0-9]{1,5}
                )
            )
            \z
        ");
        &REGEX
    }
}

// AMINOACID

/// Regular expression to validate aminoacid sequences.
struct AminoacidRegex;

impl FieldRegex for AminoacidRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(r"(?x)
            \A
            (?:
                [ABCDEFGHIJKLMNPQRSTVWXYZabcdefghijklmnpqrstvwxyz]+
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
                [ABCDEFGHIJKLMNPQRSTVWXYZabcdefghijklmnpqrstvwxyz]+
            )
            \z
        ");
        &REGEX
    }
}

// PROTEOME

/// Regular expression to validate proteome identifiers.
struct ProteomeRegex;

impl FieldRegex for ProteomeRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(r"(?x)
            \A
            (?:
                UP[0-9]{9}
            )
            \z
        ");
        &REGEX
    }

    fn extract() -> &'static Regex {
        lazy_regex!(r"(?x)
            \A
            # Group 1, Proteome ID
            (
                UP[0-9]{9}
            )
        ");
        &REGEX
    }
}

// TAXONOMY

/// Regular expression to validate taxonomic identifiers.
struct TaxonomyRegex;

impl FieldRegex for TaxonomyRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(r"(?x)
            \A
            (?:
                \d+
            )
            \z
        ");
        &REGEX
    }

    fn extract() -> &'static Regex {
        lazy_regex!(r"(?x)
            \A
            # Group 1, Taxonomy ID
            (
                \d+
            )
            \z
        ");
        &REGEX
    }
}

// FASTA HEADER

/// Hard-coded index fields for data extraction.
const ACCESSION_INDEX: usize = 2;
const MNEMONIC_INDEX: usize = 3;
const NAME_INDEX: usize = 4;
const ORGANISM_INDEX: usize = 5;
const GENE_INDEX: usize = 6;
const PE_INDEX: usize = 7;
const SV_INDEX: usize = 8;

/// Regular expression to validate and extract FASTA headers.
struct FastaHeaderRegex;

impl FieldRegex for FastaHeaderRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(r"(?x)(?m)
             \A
            (?:
                >sp\|
                (?:
                    [OPQ][0-9][A-Z0-9]{3}[0-9]|[A-NR-Z][0-9](?:[A-Z][A-Z0-9]{2}[0-9]){1,2}
                )
                \|
                (?:
                    [a-zA-Z0-9]{1,5}_[a-zA-Z0-9]{1,5}
                )
                \s
                (?:
                    .+
                )
                \sOS=
                (?:
                    .+
                )
                \sGN=
                (?:
                    [[:alnum:]]+
                )
                \sPE=
                (?:
                    [[:digit:]]+
                )
                \sSV=
                (?:
                    [[:digit:]]+
                )
            )
            $
        ");
        &REGEX
    }

    fn extract() -> &'static Regex {
        lazy_regex!(r"(?x)(?m)
            \A
            # Group 1, the entire header.
            (
                >sp\|
                # Group 2, Accession Number
                (
                    [OPQ][0-9][A-Z0-9]{3}[0-9]|[A-NR-Z][0-9](?:[A-Z][A-Z0-9]{2}[0-9]){1,2}
                )
                \|
                # Group 3, Mnemonic Identifier
                (
                    [a-zA-Z0-9]{1,5}_[a-zA-Z0-9]{1,5}
                )
                \s
                #Group 4, Protein Name
                (
                    .+
                )
                \sOS=
                # Group 5, Organism Name
                (
                    .+
                )
                \sGN=
                # Group 6, Gene Name
                (
                    [[:alnum:]]+
                )
                \sPE=
                # Group 7, Protein Evidence
                (
                    [[:digit:]]+
                )
                \sSV=
                # Group 8, Sequence Version
                (
                    [[:digit:]]+
                )
            )
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

    /// Check regex validates or does not validate text.
    fn validate_regex<T: FieldRegex>(text: &str, result: bool) {
        assert_eq!(T::validate().is_match(text), result);
    }

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
        type T = MnemonicRegex;

        // empty
        check_regex::<T>("", false);

        // valid
        check_regex::<T>("G3P_RABIT", true);
        check_regex::<T>("1433B_HUMAN", true);
        check_regex::<T>("ENO_ACTSZ", true);

        // valid + 1 letter
        check_regex::<T>("G3P_RABITX", false);
        check_regex::<T>("1433B_HUMANX", false);

        // valid - group
        check_regex::<T>("_RABIT", false);
        check_regex::<T>("G3P_", false);
        check_regex::<T>("_HUMAN", false);
        check_regex::<T>("1433B_", false);

        check_regex::<T>(" G3P_RABIT", false);
        check_regex::<T>("G3P_RABIT ", false);
        check_regex::<T>(" ENO_ACTSZ", false);
        check_regex::<T>("ENO_ACTSZ ", false);

        // extract
        extract_regex::<T>("G3P_RABIT", 1, "G3P_RABIT");
        extract_regex::<T>("G3P_RABIT", 2, "G3P");
        extract_regex::<T>("G3P_RABIT", 3, "RABIT");
    }

    #[test]
    fn aminoacid_regex() {
        type T = AminoacidRegex;

        // empty
        check_regex::<T>("", false);

        // valid
        check_regex::<T>("SAMPLER", true);
        check_regex::<T>("sampler", true);
        check_regex::<T>("sAmpLer", true);

        // invalid aminoacid
        check_regex::<T>("ORANGE", false);
        check_regex::<T>("oRANGE", false);

        // extract
        extract_regex::<T>("SAMPLER", 1, "SAMPLER");
    }

    #[test]
    fn proteome_regex() {
        type T = ProteomeRegex;

        // empty
        check_regex::<T>("", false);

        // valid
        check_regex::<T>("UP000001811", true);
        check_regex::<T>("UP000001114", true);

        // mutated valid
        check_regex::<T>("UX000001811", false);
        check_regex::<T>("UPX00001114", false);

        // valid + 1 number
        validate_regex::<T>("UP0000018113", false);
        validate_regex::<T>("UP0000011144", false);

        // valid + trailing
        validate_regex::<T>("UP000001811: Unplaced", false);
        validate_regex::<T>("UP000001114: Chromosome", false);

        // extract
        extract_regex::<T>("UP000001811: Unplaced", 1, "UP000001811");
        extract_regex::<T>("UP000001114: Chromosome", 1, "UP000001114");
    }

    #[test]
    fn taxonomy_regex() {
        type T = TaxonomyRegex;

        // empty
        check_regex::<T>("", false);

        // valid
        check_regex::<T>("9606", true);
        check_regex::<T>("731", true);

        // invalid
        check_regex::<T>("965X", false);
        check_regex::<T>("965 ", false);
        check_regex::<T>(" 965", false);
        check_regex::<T>("X965", false);

        // extract
       extract_regex::<T>("9606", 1, "9606");
    }

    #[test]
    fn fasta_header_regex() {
        type T = FastaHeaderRegex;

        // empty
        check_regex::<T>("", false);

        // valid
        check_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", true);
        check_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\n", true);
        check_regex::<T>(">sp|P02769|ALBU_BOVIN Serum albumin OS=Bos taurus GN=ALB PE=1 SV=4", true);
        check_regex::<T>(">sp|P02769|ALBU_BOVIN Serum albumin OS=Bos taurus GN=ALB PE=1 SV=4\n", true);

        // invalid
        check_regex::<T>(">up|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", false);
        check_regex::<T>(">sp|PX6406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", false);
        check_regex::<T>(">sp|P46406|G3P_RABITS Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", false);
        check_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAP-DH PE=1 SV=3", false);
        check_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1X SV=3", false);
        check_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=X3", false);

        // extract
        extract_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", 1, ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3");
        extract_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", ACCESSION_INDEX, "P46406");
        extract_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", MNEMONIC_INDEX, "G3P_RABIT");
        extract_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", NAME_INDEX, "Glyceraldehyde-3-phosphate dehydrogenase");
        extract_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", ORGANISM_INDEX, "Oryctolagus cuniculus");
        extract_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", GENE_INDEX, "GAPDH");
        extract_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", PE_INDEX, "1");
        extract_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", SV_INDEX, "3");
    }

    // RECORDS

    /// Create a record for the standard protein GAPDH.
    fn gapdh() -> Record {
        Record {
            sequence_version: 3,
            protein_evidence: ProteinEvidence::ProteinLevel,
            mass: 35780,
            length: 333,
            gene: String::from("GAPDH"),
            id: String::from("P46406"),
            mnemonic: String::from("G3P_RABIT"),
            name: String::from("Glyceraldehyde-3-phosphate dehydrogenase"),
            organism: String::from("Oryctolagus cuniculus"),
            proteome: String::from("UP000001811"),
            sequence: String::from("MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE"),
            taxonomy: String::from("9986")
        }
    }

    /// Create a record for the standard protein BSA.
    fn bsa() -> Record {
        Record {
            sequence_version: 4,
            protein_evidence: ProteinEvidence::ProteinLevel,
            mass: 69293,
            length: 607,
            gene: String::from("ALB"),
            id: String::from("P02769"),
            mnemonic: String::from("ALBU_BOVIN"),
            name: String::from("Serum albumin"),
            organism: String::from("Bos taurus"),
            proteome: String::from("UP000009136"),
            sequence: String::from("MKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA"),
            taxonomy: String::from("9913")
        }
    }

    /// Check a record from FASTA with incomplete data is equal to the original.
    fn incomplete_eq(x: &Record, y: &Record) {
        assert_eq!(y.sequence_version, x.sequence_version);
        assert_eq!(y.protein_evidence, x.protein_evidence);
        assert_eq!(y.mass, x.mass);
        assert_eq!(y.length, x.length);
        assert_eq!(y.gene, x.gene);
        assert_eq!(y.id, x.id);
        assert_eq!(y.mnemonic, x.mnemonic);
        assert_eq!(y.name, x.name);
        assert_eq!(y.organism, x.organism);
        assert_eq!(y.proteome, "");
        assert_eq!(y.sequence, x.sequence);
        assert_eq!(y.taxonomy, "");

        assert!(x.is_valid());
        assert!(x.is_complete());

        assert!(y.is_valid());
        assert!(!y.is_complete());
    }

    #[test]
    fn debug_record() {
        let text = format!("{:?}", gapdh());
        assert_eq!(text, "Record { sequence_version: 3, protein_evidence: ProteinLevel, mass: 35780, length: 333, gene: \"GAPDH\", id: \"P46406\", mnemonic: \"G3P_RABIT\", name: \"Glyceraldehyde-3-phosphate dehydrogenase\", organism: \"Oryctolagus cuniculus\", proteome: \"UP000001811\", sequence: \"MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\", taxonomy: \"9986\" }");

        let text = format!("{:?}", bsa());
        assert_eq!(text, "Record { sequence_version: 4, protein_evidence: ProteinLevel, mass: 69293, length: 607, gene: \"ALB\", id: \"P02769\", mnemonic: \"ALBU_BOVIN\", name: \"Serum albumin\", organism: \"Bos taurus\", proteome: \"UP000009136\", sequence: \"MKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA\", taxonomy: \"9913\" }");
    }

    #[test]
    fn equality_record() {
        let x = gapdh();
        let y = gapdh();
        let z = bsa();
        assert_eq!(x, y);
        assert_ne!(x, z);
        assert_ne!(y, z);
    }

    #[test]
    fn properties_record() {
        // test various permutations that can lead to
        // invalid or incomplete identifications
        let g1 = gapdh();
        let mut g2 = g1.clone();
        assert!(g2.is_valid());
        assert!(g2.is_complete());

        // check keeping the protein valid but make it incomplete
        g2.proteome = String::new();
        assert!(g2.is_valid());
        assert!(!g2.is_complete());
        g2.proteome = g1.proteome.clone();

        g2.taxonomy = String::new();
        assert!(g2.is_valid());
        assert!(!g2.is_complete());
        g2.taxonomy = g1.taxonomy.clone();

        // check replacing items with valid, but different data
        g2.sequence_version = 1;
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.sequence_version = g1.sequence_version;

        g2.protein_evidence = ProteinEvidence::Inferred;
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.protein_evidence = g1.protein_evidence;

        g2.mass = 64234;
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.mass = g1.mass;

        g2.sequence = String::from(&g2.sequence[0..200]);
        g2.length = 200;
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.sequence = g1.sequence.clone();
        g2.length = g1.length;

        g2.gene = String::from("HIST1H1A");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.gene = g1.gene.clone();

        g2.id = String::from("A0A022YWF9");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.id = g1.id.clone();

        g2.id = String::from("A2BC19");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.id = g1.id.clone();

        g2.mnemonic = String::from("H11_HUMAN");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.mnemonic = g1.mnemonic.clone();

        g2.name = String::from("Histone H1.1");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.name = g1.name.clone();

        g2.organism = String::from("Homo sapiens");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.organism = g1.organism.clone();

        g2.proteome = String::from("UP000005640");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.proteome = g1.proteome.clone();

        g2.taxonomy = String::from("9606");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.taxonomy = g1.taxonomy.clone();

        // check replacing items with invalid data
        g2.sequence_version = 0;
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.sequence_version = g1.sequence_version;

        g2.protein_evidence = ProteinEvidence::Unknown;
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.protein_evidence = g1.protein_evidence;

        g2.mass = 0;
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.mass = g1.mass;

        g2.length = 334;
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.length = g1.length;

        g2.gene = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.gene = g1.gene.clone();

        g2.id = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.id = g1.id.clone();

        g2.mnemonic = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.mnemonic = g1.mnemonic.clone();

        g2.name = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.name = g1.name.clone();

        g2.organism = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.organism = g1.organism.clone();

        g2.sequence = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.sequence = g1.sequence.clone();
    }

    #[test]
    fn serde_record() {
        let x = serde_json::to_string(&gapdh()).unwrap();
        assert_eq!(x, "{\"sequence_version\":3,\"protein_evidence\":1,\"mass\":35780,\"length\":333,\"gene\":\"GAPDH\",\"id\":\"P46406\",\"mnemonic\":\"G3P_RABIT\",\"name\":\"Glyceraldehyde-3-phosphate dehydrogenase\",\"organism\":\"Oryctolagus cuniculus\",\"proteome\":\"UP000001811\",\"sequence\":\"MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\",\"taxonomy\":\"9986\"}");

        let x = serde_json::to_string(&bsa()).unwrap();
        assert_eq!(x, "{\"sequence_version\":4,\"protein_evidence\":1,\"mass\":69293,\"length\":607,\"gene\":\"ALB\",\"id\":\"P02769\",\"mnemonic\":\"ALBU_BOVIN\",\"name\":\"Serum albumin\",\"organism\":\"Bos taurus\",\"proteome\":\"UP000009136\",\"sequence\":\"MKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA\",\"taxonomy\":\"9913\"}");

        let x = serde_json::to_string(&Record::new()).unwrap();
        assert_eq!(x, "{\"sequence_version\":0,\"protein_evidence\":5,\"mass\":0,\"length\":0,\"gene\":\"\",\"id\":\"\",\"mnemonic\":\"\",\"name\":\"\",\"organism\":\"\",\"proteome\":\"\",\"sequence\":\"\",\"taxonomy\":\"\"}");
    }

// TODO(ahuszagh)
//      Restore
//    #[test]
//    fn fasta_record() {
//        // gapdh
//        let p = gapdh();
//        let x = p.to_fasta().unwrap();
//        assert_eq!(x, ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\nMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA\nENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS\nAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT\nATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV\nVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA\nLNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE");
//        //assert_eq!(x, p.to_fasta_string().unwrap());
//        let y = Record::from_fasta(&x).unwrap();
//        incomplete_eq(&p, &y);
//
//        // bsa
//        let p = bsa();
//        let x = p.to_fasta().unwrap();
//        assert_eq!(x, ">sp|P02769|ALBU_BOVIN Serum albumin OS=Bos taurus GN=ALB PE=1 SV=4\nMKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPF\nDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEP\nERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYY\nANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVA\nRLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKE\nCCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRR\nHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEK\nLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLIL\nNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLP\nDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVV\nSTQTALA");
//        let y = Record::from_fasta(&x).unwrap();
//        incomplete_eq(&p, &y);
//
//        // empty
//        let p = Record::new();
//        let x = p.to_fasta().unwrap();
//        assert_eq!(x, ">sp||  OS= GN= PE=5 SV=0");
//    }

    // TODO(ahuszagh)
    //  CSV

    // TODO(ahuszagh)
    //  Import tests from uniprot.rs
    //  Implement tests of the regular expressions
}
