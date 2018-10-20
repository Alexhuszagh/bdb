//! Regular expression utilities for UniProt services.

use regex::{Captures, Regex};

/// Regular expressions for UniProt record fields.
pub trait FieldRegex {
    /// Validate a field.
    fn validate() -> &'static Regex;
    /// Extract a field from external data.
    fn extract() -> &'static Regex;
}

/// Construct new regex from pattern.
#[inline(always)]
fn new_regex(pattern: &'static str) -> Regex {
    Regex::new(pattern).unwrap()
}

/// Construct static-like regex lazily at runtime.
macro_rules! lazy_regex {
    ($str:expr) => (lazy_static! {
        static ref REGEX: Regex = new_regex($str);
    })
}

// ACCESSION

/// Regular expression to validate accession numbers.
///
/// Derived from [here](https://www.uniprot.org/help/accession_numbers).
pub struct AccessionRegex;

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
pub struct MnemonicRegex;

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

// GENE

/// Regular expression to validate gene names.
pub struct GeneRegex;

impl FieldRegex for GeneRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(r"(?x)
            \A
            (?:
                [[:alnum:]-_\x20/*.@:();'$+]+
            )
            \z
        ");
        &REGEX
    }

    fn extract() -> &'static Regex {
        lazy_regex!(r"(?x)
            \A
            # Group 1, Gene Name
            (
                [[:alnum:]-_\x20/*.@:();'$+]+
            )
            \z
        ");
        &REGEX
    }
}

// AMINOACID

/// Regular expression to validate aminoacid sequences.
pub struct AminoacidRegex;

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
pub struct ProteomeRegex;

impl FieldRegex for ProteomeRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(r"(?x)
            \A
            (?:
                UP[0-9]{9}
                (?:
                    :\s[[:upper:]][[:lower:]]+
                )?
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
pub struct TaxonomyRegex;

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

/// Regular expression to validate and extract FASTA headers.
pub struct FastaHeaderRegex;

impl FastaHeaderRegex {
    /// Hard-coded index fields for data extraction.
    pub const ACCESSION_INDEX: usize = 2;
    pub const MNEMONIC_INDEX: usize = 3;
    pub const NAME_INDEX: usize = 4;
    pub const ORGANISM_INDEX: usize = 5;
    pub const GENE_INDEX: usize = 6;
    pub const PE_INDEX: usize = 7;
    pub const SV_INDEX: usize = 8;
}

impl FieldRegex for FastaHeaderRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(r"(?x)(?m)
             \A
            (?:
                >sp\|
                (?:
                    (?:[OPQ][0-9][A-Z0-9]{3}[0-9]|[A-NR-Z][0-9](?:[A-Z][A-Z0-9]{2}[0-9]){1,2})?
                )
                \|
                (?:
                    (?:[a-zA-Z0-9]{1,5}_[a-zA-Z0-9]{1,5})?
                )
                \s
                (?:
                    .*
                )
                \sOS=
                (?:
                    .*
                )
                \sGN=
                (?:
                    [[:alnum:]-_\x20/*.@:();'$+]*
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
                    (?:[OPQ][0-9][A-Z0-9]{3}[0-9]|[A-NR-Z][0-9](?:[A-Z][A-Z0-9]{2}[0-9]){1,2})?
                )
                \|
                # Group 3, Mnemonic Identifier
                (
                    (?:[a-zA-Z0-9]{1,5}_[a-zA-Z0-9]{1,5})?
                )
                \s
                #Group 4, Protein Name
                (
                    .*
                )
                \sOS=
                # Group 5, Organism Name
                (
                    .*
                )
                \sGN=
                # Group 6, Gene Name
                (
                    [[:alnum:]-_\x20/*.@:();'$+]*
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

// CAPTURES

/// Convert capture group to `&str`.
#[inline(always)]
pub fn capture_as_str<'t>(captures: &'t Captures, index: usize) -> &'t str {
    captures.get(index).unwrap().as_str()
}

/// Convert capture group to `String`.
#[inline(always)]
pub fn capture_as_string(captures: &Captures, index: usize) -> String {
    String::from(capture_as_str(captures, index))
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;
    use test::testdata_dir;
    use super::*;

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
    fn gene_regex() {
        type T = GeneRegex;

        // empty
        check_regex::<T>("", false);

        // valid
        check_regex::<T>("ND3", true);
        check_regex::<T>("KIF5B-RET(NM_020975)_K15;R12", true);
        check_regex::<T>("TRA@", true);
        check_regex::<T>("HLA-DRB5", true);
        check_regex::<T>("NOD2/CARD15", true);
        check_regex::<T>("Hosa(Biaka)-T2R50", true);
        check_regex::<T>("cytb", true);
        check_regex::<T>("dopamine D4 receptor/ DRD4", true);

        // valid + 1 letter
        check_regex::<T>("ND3[", false);
        check_regex::<T>("ND3`", false);

        // extract
        extract_regex::<T>("ND3", 1, "ND3");
        extract_regex::<T>("KIF5B-RET(NM_020975)_K15;R12", 1, "KIF5B-RET(NM_020975)_K15;R12");
        extract_regex::<T>("TRA@", 1, "TRA@");
        extract_regex::<T>("Hosa(Biaka)-T2R50", 1, "Hosa(Biaka)-T2R50");
        extract_regex::<T>("dopamine D4 receptor/ DRD4", 1, "dopamine D4 receptor/ DRD4");
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
        validate_regex::<T>("UP000001811: Unplaced", true);
        validate_regex::<T>("UP000001114: Chromosome", true);

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
        check_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAP[DH PE=1 SV=3", false);
        check_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1X SV=3", false);
        check_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=X3", false);

        // extract
        extract_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", 1, ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3");
        extract_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", T::ACCESSION_INDEX, "P46406");
        extract_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", T::MNEMONIC_INDEX, "G3P_RABIT");
        extract_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", T::NAME_INDEX, "Glyceraldehyde-3-phosphate dehydrogenase");
        extract_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", T::ORGANISM_INDEX, "Oryctolagus cuniculus");
        extract_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", T::GENE_INDEX, "GAPDH");
        extract_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", T::PE_INDEX, "1");
        extract_regex::<T>(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", T::SV_INDEX, "3");
    }

    fn human_dir() -> PathBuf {
        let mut dir = testdata_dir();
        dir.push("uniprot/human");
        dir
    }

    #[test]
    #[ignore]
    fn human_gene_regex() {
        let mut path = human_dir();
        path.push("gene");
        let reader = BufReader::new(File::open(path).unwrap());

        for gene in reader.lines() {
            assert!(GeneRegex::validate().is_match(&gene.unwrap()));
        }
    }
}
