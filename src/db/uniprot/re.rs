//! Regular expression utilities for UniProt services.
//!
//! Disable Unicode for all but the generic header formats, which may
//! accept arbitrary Unicode input. The rest should only be valid ASCII,
//! and therefore we should disable matching to Unicode characters
//! explicitly.

use regex::Regex;
use regex::bytes::Regex as BytesRegex;

// Re-export regular-expression traits.
pub use util::{ExtractionRegex, ValidationRegex};

// ACCESSION

/// Regular expression to validate accession numbers.
///
/// Derived from [here](https://www.uniprot.org/help/accession_numbers).
pub struct AccessionRegex;

impl ValidationRegex<Regex> for AccessionRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r"(?-u)(?x)
            \A
            (?:
                [OPQ][0-9][A-Z0-9]{3}[0-9]|
                [A-NR-Z][0-9](?:[A-Z][A-Z0-9]{2}[0-9]){1,2}
            )
            \z
        ");
        &REGEX
    }
}

impl ExtractionRegex<Regex> for AccessionRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r"(?-u)(?x)
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

impl ValidationRegex<Regex> for MnemonicRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r"(?-u)(?x)
            \A
            (?:
                (?:
                    (?:
                        [[:alnum:]]{1,5}
                    )
                    |
                    (?:
                        [OPQ][0-9][A-Z0-9]{3}[0-9]|[A-NR-Z][0-9](?:[A-Z][A-Z0-9]{2}[0-9]){1,2}
                    )
                )
                _
                (?:
                    [[:alnum:]]{1,5}
                )
            )
            \z
        ");
        &REGEX
    }
}

impl ExtractionRegex<Regex> for MnemonicRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r"(?-u)(?x)
            \A
            # Group 1, Mnemonic Identifier
            (
                # Group 2, Protein Name
                # Can be either {1,5} alnum characters in SwissProt
                # or an accession number in TrEMBL.
                (
                    (?:
                        [[:alnum:]]{1,5}
                    )
                    |
                    (?:
                        [OPQ][0-9][A-Z0-9]{3}[0-9]|[A-NR-Z][0-9](?:[A-Z][A-Z0-9]{2}[0-9]){1,2}
                    )
                )
                _
                # Group 3, Species Name
                (
                    [[:alnum:]]{1,5}
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

impl ValidationRegex<Regex> for GeneRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r"(?-u)(?x)
            \A
            (?:
                [[:alnum:]-_\x20/*.@:();'$+]+
            )
            \z
        ");
        &REGEX
    }
}

impl ExtractionRegex<Regex> for GeneRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r"(?-u)(?x)
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

impl ValidationRegex<BytesRegex> for AminoacidRegex {
    fn validate() -> &'static BytesRegex {
        lazy_regex!(BytesRegex, r"(?-u)(?x)
            \A
            (?:
                [ABCDEFGHIJKLMNPQRSTUVWXYZabcdefghijklmnpqrstuvwxyz]+
            )
            \z
        ");
        &REGEX
    }
}

impl ExtractionRegex<BytesRegex> for AminoacidRegex {
    fn extract() -> &'static BytesRegex {
        lazy_regex!(BytesRegex, r"(?-u)(?x)
            \A
            # Group 1, Aminoacid Sequence
            (
                [ABCDEFGHIJKLMNPQRSTUVWXYZabcdefghijklmnpqrstuvwxyz]+
            )
            \z
        ");
        &REGEX
    }
}

// PROTEOME

/// Regular expression to validate proteome identifiers.
pub struct ProteomeRegex;

impl ValidationRegex<Regex> for ProteomeRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r"(?-u)(?x)
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
}

impl ExtractionRegex<Regex> for ProteomeRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r"(?-u)(?x)
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

impl ValidationRegex<Regex> for TaxonomyRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r"(?-u)(?x)
            \A
            (?:
                [[:digit:]]+
            )
            \z
        ");
        &REGEX
    }
}

impl ExtractionRegex<Regex> for TaxonomyRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r"(?-u)(?x)
            \A
            # Group 1, Taxonomy ID
            (
                [[:digit:]]+
            )
            \z
        ");
        &REGEX
    }
}

// FASTA HEADER

/// Regular expression to validate and extract SwissProt FASTA headers.
pub struct SwissProtHeaderRegex;

impl SwissProtHeaderRegex {
    /// Hard-coded index fields for data extraction.
    pub const ACCESSION_INDEX: usize = 2;
    pub const MNEMONIC_INDEX: usize = 3;
    pub const NAME_INDEX: usize = 4;
    pub const ORGANISM_INDEX: usize = 5;
    pub const TAXONOMY_INDEX: usize = 6;
    pub const GENE_INDEX: usize = 7;
    pub const PE_INDEX: usize = 8;
    pub const SV_INDEX: usize = 9;
}

impl ValidationRegex<Regex> for SwissProtHeaderRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)(?m)
             \A
            (?:
                >sp\|
                (?:
                    (?:[OPQ][0-9][A-Z0-9]{3}[0-9]|[A-NR-Z][0-9](?:[A-Z][A-Z0-9]{2}[0-9]){1,2})?
                )
                \|
                (?:
                    (?:[[:alnum:]]{1,5}_[[:alnum:]]{1,5})?
                )
                \s
                (?:
                    .*?
                )
                \sOS=
                (?:
                    .*?
                )
                (?:
                    \sOX=
                    (?:
                        [[:digit:]]*
                    )
                )?
                (?:
                    \sGN=
                    (?:
                        [[:alnum:]-_\x20/*.@:();'$+]*
                    )
                )?
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
}

impl ExtractionRegex<Regex> for SwissProtHeaderRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)(?m)
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
                # The first part must be {1,5} alnum characters in SwissProt
                (
                    (?:[[:alnum:]]{1,5}_[[:alnum:]]{1,5})?
                )
                \s
                #Group 4, Protein Name
                (
                    .*?
                )
                \sOS=
                # Group 5, Organism Name
                (
                    .*?
                )
                (?:
                    \sOX=
                    # Group 6, Taxonomy ID
                    (
                        [[:digit:]]*
                    )
                )?
                (?:
                    \sGN=
                    # Group 7, Gene Name
                    (
                        [[:alnum:]-_\x20/*.@:();'$+]*
                    )
                )?
                \sPE=
                # Group 8, Protein Evidence
                (
                    [[:digit:]]+
                )
                \sSV=
                # Group 9, Sequence Version
                (
                    [[:digit:]]+
                )
            )
        ");
        &REGEX
    }
}

/// Regular expression to validate and extract TrEMBL FASTA headers.
pub struct TrEMBLHeaderRegex;

impl TrEMBLHeaderRegex {
    /// Hard-coded index fields for data extraction.
    pub const ACCESSION_INDEX: usize = 2;
    pub const MNEMONIC_INDEX: usize = 3;
    pub const NAME_INDEX: usize = 4;
    pub const ORGANISM_INDEX: usize = 5;
    pub const TAXONOMY_INDEX: usize = 6;
    pub const GENE_INDEX: usize = 7;
    pub const PE_INDEX: usize = 8;
    pub const SV_INDEX: usize = 9;
}

impl ValidationRegex<Regex> for TrEMBLHeaderRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)(?m)
             \A
            (?:
                >tr\|
                (?:
                    (?:[OPQ][0-9][A-Z0-9]{3}[0-9]|[A-NR-Z][0-9](?:[A-Z][A-Z0-9]{2}[0-9]){1,2})?
                )
                \|
                (?:
                    (?:
                        (?:
                            (?:
                                [[:alnum:]]{1,5}
                            )
                            |
                            (?:
                                [OPQ][0-9][A-Z0-9]{3}[0-9]|[A-NR-Z][0-9](?:[A-Z][A-Z0-9]{2}[0-9]){1,2}
                            )
                        )
                        _
                        (?:
                            [[:alnum:]]{1,5}
                        )
                    )?
                )
                \s
                (?:
                    .*?
                )
                \sOS=
                (?:
                    .*?
                )
                (?:
                    \sOX=
                    (?:
                        [[:digit:]]*
                    )
                )?
                (?:
                    \sGN=
                    (?:
                        [[:alnum:]-_\x20/*.@:();'$+]*
                    )
                )?
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
}

impl ExtractionRegex<Regex> for TrEMBLHeaderRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)(?m)
            \A
            # Group 1, the entire header.
            (
                >tr\|
                # Group 2, Accession Number
                (
                    (?:[OPQ][0-9][A-Z0-9]{3}[0-9]|[A-NR-Z][0-9](?:[A-Z][A-Z0-9]{2}[0-9]){1,2})?
                )
                \|
                # Group 3, Mnemonic Identifier
                (
                    (?:
                        (?:
                            (?:
                                [[:alnum:]]{1,5}
                            )
                            |
                            (?:
                                [OPQ][0-9][A-Z0-9]{3}[0-9]|[A-NR-Z][0-9](?:[A-Z][A-Z0-9]{2}[0-9]){1,2}
                            )
                        )
                        _
                        (?:
                            [[:alnum:]]{1,5}
                        )
                    )?
                )
                \s
                #Group 4, Protein Name
                (
                    .*?
                )
                \sOS=
                # Group 5, Organism Name
                (
                    .*?
                )
                (?:
                    \sOX=
                    # Group 6, Taxonomy ID
                    (
                        [[:digit:]]*
                    )
                )?
                (?:
                    \sGN=
                    # Group 7, Gene Name
                    (
                        [[:alnum:]-_\x20/*.@:();'$+]*
                    )
                )?
                \sPE=
                # Group 8, Protein Evidence
                (
                    [[:digit:]]+
                )
                \sSV=
                # Group 9, Sequence Version
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
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;
    use test::testdata_dir;
    use super::*;

    #[test]
    fn accession_regex_test() {
        type T = AccessionRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        check_regex!(T, "A2BC19", true);
        check_regex!(T, "P12345", true);
        check_regex!(T, "A0A022YWF9", true);

        // valid - 1 letter
        check_regex!(T, "2BC19", false);
        check_regex!(T, "A2BC1", false);
        check_regex!(T, "0A022YWF9", false);
        check_regex!(T, "A0A022YWF", false);

        // valid + 1 letter
        check_regex!(T, "XA2BC19", false);
        check_regex!(T, "A2BC19X", false);
        check_regex!(T, "XA0A022YWF9", false);
        check_regex!(T, "A0A022YWF9X", false);

        // valid + space
        check_regex!(T, " A2BC19", false);
        check_regex!(T, "A2BC19 ", false);
        check_regex!(T, " A0A022YWF9", false);
        check_regex!(T, "A0A022YWF9 ", false);

        // extract
        extract_regex!(T, "A2BC19", 1, "A2BC19", as_str);
        extract_regex!(T, "P12345", 1, "P12345", as_str);
        extract_regex!(T, "A0A022YWF9", 1, "A0A022YWF9", as_str);
    }

    #[test]
    fn mnemonic_regex_test() {
        type T = MnemonicRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        check_regex!(T, "G3P_RABIT", true);
        check_regex!(T, "1433B_HUMAN", true);
        check_regex!(T, "ENO_ACTSZ", true);
        check_regex!(T, "A0A024R832_HUMAN", true);

        // valid + 1 letter
        check_regex!(T, "G3P_RABITX", false);
        check_regex!(T, "1433B_HUMANX", false);
        check_regex!(T, "A0A024R832_HUMANX", false);

        // valid - group
        check_regex!(T, "_RABIT", false);
        check_regex!(T, "G3P_", false);
        check_regex!(T, "_HUMAN", false);
        check_regex!(T, "1433B_", false);
        check_regex!(T, "A0A024R832_", false);

        check_regex!(T, " G3P_RABIT", false);
        check_regex!(T, "G3P_RABIT ", false);
        check_regex!(T, " ENO_ACTSZ", false);
        check_regex!(T, "ENO_ACTSZ ", false);

        // extract
        extract_regex!(T, "G3P_RABIT", 1, "G3P_RABIT", as_str);
        extract_regex!(T, "G3P_RABIT", 2, "G3P", as_str);
        extract_regex!(T, "G3P_RABIT", 3, "RABIT", as_str);
    }

    #[test]
    fn gene_regex_test() {
        type T = GeneRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        check_regex!(T, "ND3", true);
        check_regex!(T, "KIF5B-RET(NM_020975)_K15;R12", true);
        check_regex!(T, "TRA@", true);
        check_regex!(T, "HLA-DRB5", true);
        check_regex!(T, "NOD2/CARD15", true);
        check_regex!(T, "Hosa(Biaka)-T2R50", true);
        check_regex!(T, "cytb", true);
        check_regex!(T, "dopamine D4 receptor/ DRD4", true);

        // valid + 1 letter
        check_regex!(T, "ND3[", false);
        check_regex!(T, "ND3`", false);

        // extract
        extract_regex!(T, "ND3", 1, "ND3", as_str);
        extract_regex!(T, "KIF5B-RET(NM_020975)_K15;R12", 1, "KIF5B-RET(NM_020975)_K15;R12", as_str);
        extract_regex!(T, "TRA@", 1, "TRA@", as_str);
        extract_regex!(T, "Hosa(Biaka)-T2R50", 1, "Hosa(Biaka)-T2R50", as_str);
        extract_regex!(T, "dopamine D4 receptor/ DRD4", 1, "dopamine D4 receptor/ DRD4", as_str);
    }

    #[test]
    fn aminoacid_regex_test() {
        type T = AminoacidRegex;

        // empty
        check_regex!(T, b"", false);

        // valid
        check_regex!(T, b"SAMPLER", true);
        check_regex!(T, b"sampler", true);
        check_regex!(T, b"sAmpLer", true);

        // Add "U", which is a non-standard aminoacid (selenocysteine)
        check_regex!(T, b"USAMPLER", true);

        // invalid aminoacid
        check_regex!(T, b"ORANGE", false);
        check_regex!(T, b"oRANGE", false);

        // extract
        extract_regex!(T, b"SAMPLER", 1, b"SAMPLER", as_bytes);
    }

    #[test]
    fn proteome_regex_test() {
        type T = ProteomeRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        check_regex!(T, "UP000001811", true);
        check_regex!(T, "UP000001114", true);

        // mutated valid
        check_regex!(T, "UX000001811", false);
        check_regex!(T, "UPX00001114", false);

        // valid + 1 number
        validate_regex!(T, "UP0000018113", false);
        validate_regex!(T, "UP0000011144", false);

        // valid + trailing
        validate_regex!(T, "UP000001811: Unplaced", true);
        validate_regex!(T, "UP000001114: Chromosome", true);

        // extract
        extract_regex!(T, "UP000001811: Unplaced", 1, "UP000001811", as_str);
        extract_regex!(T, "UP000001114: Chromosome", 1, "UP000001114", as_str);
    }

    #[test]
    fn taxonomy_regex_test() {
        type T = TaxonomyRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        check_regex!(T, "9606", true);
        check_regex!(T, "731", true);

        // invalid
        check_regex!(T, "965X", false);
        check_regex!(T, "965 ", false);
        check_regex!(T, " 965", false);
        check_regex!(T, "X965", false);

        // extract
       extract_regex!(T, "9606", 1, "9606", as_str);
    }

    #[test]
    fn swissprot_header_regex_test() {
        type T = SwissProtHeaderRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        check_regex!(T, ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", true);
        check_regex!(T, ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\n", true);
        check_regex!(T, ">sp|P02769|ALBU_BOVIN Serum albumin OS=Bos taurus GN=ALB PE=1 SV=4", true);
        check_regex!(T, ">sp|P02769|ALBU_BOVIN Serum albumin OS=Bos taurus GN=ALB PE=1 SV=4\n", true);
        check_regex!(T, ">sp|Q9N2K0|ENH1_HUMAN HERV-H_2q24.3 provirus ancestral Env polyprotein OS=Homo sapiens OX=9606 PE=2 SV=1", true);
        check_regex!(T, ">sp|Q6ZN92|DUTL_HUMAN Putative inactive deoxyuridine 5\'-triphosphate nucleotidohydrolase-like protein FLJ16323 OS=Homo sapiens OX=9606 PE=5 SV=1", true);

        // invalid
        check_regex!(T, ">up|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", false);
        check_regex!(T, ">sp|PX6406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", false);
        check_regex!(T, ">sp|P46406|G3P_RABITS Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3", false);
        check_regex!(T, ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1X SV=3", false);
        check_regex!(T, ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=X3", false);

        // extract
        static GAPDH: &'static str = ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3";
        extract_regex!(T, GAPDH, 1, GAPDH, as_str);
        extract_regex!(T, GAPDH, T::ACCESSION_INDEX, "P46406", as_str);
        extract_regex!(T, GAPDH, T::MNEMONIC_INDEX, "G3P_RABIT", as_str);
        extract_regex!(T, GAPDH, T::NAME_INDEX, "Glyceraldehyde-3-phosphate dehydrogenase", as_str);
        extract_regex!(T, GAPDH, T::ORGANISM_INDEX, "Oryctolagus cuniculus", as_str);
        extract_regex!(T, GAPDH, T::GENE_INDEX, "GAPDH", as_str);
        extract_regex!(T, GAPDH, T::PE_INDEX, "1", as_str);
        extract_regex!(T, GAPDH, T::SV_INDEX, "3", as_str);

        // extract (no gene name)
        static ENH1: &'static str = ">sp|Q9N2K0|ENH1_HUMAN HERV-H_2q24.3 provirus ancestral Env polyprotein OS=Homo sapiens OX=9606 PE=2 SV=1";
        extract_regex!(T, ENH1, 1, ENH1, as_str);
        extract_regex!(T, ENH1, T::ACCESSION_INDEX, "Q9N2K0", as_str);
        extract_regex!(T, ENH1, T::MNEMONIC_INDEX, "ENH1_HUMAN", as_str);
        extract_regex!(T, ENH1, T::NAME_INDEX, "HERV-H_2q24.3 provirus ancestral Env polyprotein", as_str);
        extract_regex!(T, ENH1, T::ORGANISM_INDEX, "Homo sapiens", as_str);
        extract_regex!(T, ENH1, T::TAXONOMY_INDEX, "9606", as_str);
        extract_regex!(T, ENH1, T::PE_INDEX, "2", as_str);
        extract_regex!(T, ENH1, T::SV_INDEX, "1", as_str);
    }

    #[test]
    fn trembl_header_regex_test() {
        type T = TrEMBLHeaderRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        check_regex!(T, ">tr|A0A2U8RNL1|A0A2U8RNL1_HUMAN MHC class II antigen (Fragment) OS=Homo sapiens OX=9606 GN=DPB1 PE=4 SV=1", true);
        check_regex!(T, ">tr|O14861|O14861_HUMAN Zinc finger protein (Fragment) OS=Homo sapiens OX=9606 PE=2 SV=1", true);
        check_regex!(T, ">tr|Q53FP0|Q53FP0_HUMAN Pyridoxine 5\'-phosphate oxidase variant (Fragment) OS=Homo sapiens OX=9606 PE=2 SV=1", true);
        check_regex!(T, ">tr|B7ZKX2|B7ZKX2_HUMAN Uncharacterized protein OS=Homo sapiens OX=9606 PE=2 SV=1", true);
        check_regex!(T, ">tr|Q59FB0|Q59FB0_HUMAN PREDICTED: KRAB domain only 2 variant (Fragment) OS=Homo sapiens OX=9606 PE=2 SV=1", true);

        // invalid
        check_regex!(T, ">ur|A0A2U8RNL1|A0A2U8RNL1_HUMAN MHC class II antigen (Fragment) OS=Homo sapiens OX=9606 GN=DPB1 PE=4 SV=1", false);
        check_regex!(T, ">tr|AXA2U8RNL1|A0A2U8RNL1_HUMAN MHC class II antigen (Fragment) OS=Homo sapiens OX=9606 GN=DPB1 PE=4 SV=1", false);
        check_regex!(T, ">tr|A0A2U8RNL1|A0A2U8RNL1_HUMANS MHC class II antigen (Fragment) OS=Homo sapiens OX=9606 GN=DPB1 PE=4 SV=1", false);
        check_regex!(T, ">tr|A0A2U8RNL1|A0A2U8RNL1_HUMAN MHC class II antigen (Fragment) OS=Homo sapiens OX=9606 GN=DPB1 PE=4X SV=1", false);
        check_regex!(T, ">tr|A0A2U8RNL1|A0A2U8RNL1_HUMAN MHC class II antigen (Fragment) OS=Homo sapiens OX=9606 GN=DPB1 PE=4 SV=X1", false);

        // extract
        static O14861: &'static str = ">tr|O14861|O14861_HUMAN Zinc finger protein (Fragment) OS=Homo sapiens OX=9606 PE=2 SV=1";
        extract_regex!(T, O14861, 1, O14861, as_str);
        extract_regex!(T, O14861, T::ACCESSION_INDEX, "O14861", as_str);
        extract_regex!(T, O14861, T::MNEMONIC_INDEX, "O14861_HUMAN", as_str);
        extract_regex!(T, O14861, T::NAME_INDEX, "Zinc finger protein (Fragment)", as_str);
        extract_regex!(T, O14861, T::ORGANISM_INDEX, "Homo sapiens", as_str);
        extract_regex!(T, O14861, T::TAXONOMY_INDEX, "9606", as_str);
        extract_regex!(T, O14861, T::PE_INDEX, "2", as_str);
        extract_regex!(T, O14861, T::SV_INDEX, "1", as_str);
    }

    fn all_dir() -> PathBuf {
        let mut dir = testdata_dir();
        dir.push("uniprot/all");
        dir
    }

    fn human_dir() -> PathBuf {
        let mut dir = testdata_dir();
        dir.push("uniprot/human");
        dir
    }

    #[test]
    #[ignore]
    fn human_accession_regex_test() {
        let mut path = human_dir();
        path.push("accession");
        let reader = BufReader::new(File::open(path).unwrap());

        for id in reader.lines() {
            assert!(AccessionRegex::validate().is_match(&id.unwrap()));
        }
    }

    #[test]
    #[ignore]
    fn human_mnemonic_regex_test() {
        let mut path = human_dir();
        path.push("mnemonic");
        let reader = BufReader::new(File::open(path).unwrap());

        for mnemonic in reader.lines() {
            assert!(MnemonicRegex::validate().is_match(&mnemonic.unwrap()));
        }
    }

    #[test]
    #[ignore]
    fn human_gene_regex_test() {
        let mut path = human_dir();
        path.push("gene");
        let reader = BufReader::new(File::open(path).unwrap());

        for gene in reader.lines() {
            assert!(GeneRegex::validate().is_match(&gene.unwrap()));
        }
    }

    #[test]
    #[ignore]
    fn human_aminoacid_regex_test() {
        let mut path = human_dir();
        path.push("aminoacid");
        let reader = BufReader::new(File::open(path).unwrap());

        for sequence in reader.lines() {
            assert!(AminoacidRegex::validate().is_match(sequence.unwrap().as_bytes()));
        }
    }

    #[test]
    #[ignore]
    fn all_proteome_regex_test() {
        let mut path = all_dir();
        path.push("proteome");
        let reader = BufReader::new(File::open(path).unwrap());

        for proteome in reader.lines() {
            assert!(ProteomeRegex::validate().is_match(&proteome.unwrap()));
        }
    }

    #[test]
    #[ignore]
    fn all_taxonomy_regex_test() {
        let mut path = all_dir();
        path.push("taxonomy");
        let reader = BufReader::new(File::open(path).unwrap());

        for ahuszagh in reader.lines() {
            assert!(TaxonomyRegex::validate().is_match(&ahuszagh.unwrap()));
        }
    }

    #[test]
    #[ignore]
    fn human_fasta_header_regex_test() {
        let mut path = human_dir();
        path.push("header");
        let reader = BufReader::new(File::open(path).unwrap());

        for header in reader.lines() {
            let header = header.unwrap();
            if header.starts_with(">sp") {
                assert!(SwissProtHeaderRegex::validate().is_match(&header));
            } else if header.starts_with(">tr") {
                assert!(TrEMBLHeaderRegex::validate().is_match(&header));
            } else {
                panic!("Unknown FASTA format.");
            }
        }
    }
}
