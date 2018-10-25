//! Regular expression utilities for SRA services.
//!
//! Disable Unicode for all but the generic header formats, which may
//! accept arbitrary Unicode input. The rest should only be valid ASCII,
//! and therefore we should disable matching to Unicode characters
//! explicitly.

use regex::Regex;
use regex::bytes::Regex as BytesRegex;

// Re-export regular-expression traits.
pub use util::{ExtractionRegex, ValidationRegex};

// NUCLEOTIDE

/// Regular expression to validate aminoacid sequences.
pub struct NucleotideRegex;

impl ValidationRegex<BytesRegex> for NucleotideRegex {
    fn validate() -> &'static BytesRegex {
        lazy_regex!(BytesRegex, r"(?-u)(?x)
            \A
            (?:
                [ACGTacgt]+
            )
            \z
        ");
        &REGEX
    }
}

impl ExtractionRegex<BytesRegex> for NucleotideRegex {
    fn extract() -> &'static BytesRegex {
        lazy_regex!(BytesRegex, r"(?-u)(?x)
            \A
            # Group 1, Nucleotide Sequence
            (
                [ACGTacgt]+
            )
            \z
        ");
        &REGEX
    }
}

// SEQUENCE QUALITY

/// Regular expression to validate sequence quality scores.
///
/// The quality score can be any value from " " (32) to "~" (126) in
/// the ASCII vocabulary,
pub struct SequenceQualityRegex;

impl ValidationRegex<BytesRegex> for SequenceQualityRegex {
    fn validate() -> &'static BytesRegex {
        lazy_regex!(BytesRegex, r"(?-u)(?x)
            \A
            (?:
                [[:print:]]+
            )
            \z
        ");
        &REGEX
    }
}

impl ExtractionRegex<BytesRegex> for SequenceQualityRegex {
    fn extract() -> &'static BytesRegex {
        lazy_regex!(BytesRegex, r"(?-u)(?x)
            \A
            # Group 1, Sequence Quality Scores
            (
                [[:print:]]+
            )
            \z
        ");
        &REGEX
    }
}

/// Regular expression to parse the sequence ID and description from FASTQ.
pub struct FastqHeaderRegex;

impl FastqHeaderRegex {
    /// Hard-coded index fields for data extraction.
    pub const SEQID_INDEX: usize = 1;
    pub const DESCRIPTION_INDEX: usize = 2;
}

impl ValidationRegex<Regex> for FastqHeaderRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)(?m)
            \A
            [@+]
            (?:
                [^[:space:]]+
            )
            \s
            (?:
                .*?
            )
            (?:
                \slength=[[:digit:]]+
            )?
            \z
        ");
        &REGEX
    }
}

impl ExtractionRegex<Regex> for FastqHeaderRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)(?m)
            \A
            [@+]        # The symbol for a header line.
            # Group 1, Sequence ID.
            (
                [^[:space:]]+
            )
            \s
            # Group 2, Description.
            (
                .*?
            )
            # Optional length after description.
            (?:
                \slength=[[:digit:]]+
            )?
            \z
        ");
        &REGEX
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nucleotide_regex() {
        type T = NucleotideRegex;

        // empty
        check_regex!(T, b"", false);

        // valid
        check_regex!(T, b"AAGTAGGTCTCGTCTGTGTTTTCTACGAGCTTGTGTTCCAGCTGACCCACTCCCTGGGTGGGGGGACTGGGT", true);
        check_regex!(T, b"CCAGCCTGGCCAACAGAGTGTTACCCCGTTTTTACTTATTTATTATTATTATTTTGAGACAGAGCATTGGTC", true);
        check_regex!(T, b"ATAAAATCAGGGGTGTTGGAGATGGGATGCCTATTTCTGCACACCTTGGCCTCCCAAATTGCTGGGATTACA", true);
        check_regex!(T, b"TTAAGAAATTTTTGCTCAAACCATGCCCTAAAGGGTTCTGTAATAAATAGGGCTGGGAAAACTGGCAAGCCA", true);

        // rna
        check_regex!(T, b"AAGUAGGUCUCGUCUGUGUUUUCUACGAGCUUGUGUUCCAGCUGACCCACUCCCUGGGUGGGGGGACUGGGU", false);
        check_regex!(T, b"CCAGCCUGGCCAACAGAGUGUUACCCCGUUUUUACUUAUUUAUUAUUAUUAUUUUGAGACAGAGCAUUGGUC", false);
        check_regex!(T, b"AUAAAAUCAGGGGUGUUGGAGAUGGGAUGCCUAUUUCUGCACACCUUGGCCUCCCAAAUUGCUGGGAUUACA", false);
        check_regex!(T, b"UUAAGAAAUUUUUGCUCAAACCAUGCCCUAAAGGGUUCUGUAAUAAAUAGGGCUGGGAAAACUGGCAAGCCA", false);

        // protein
        check_regex!(T, b"SAMPLER", false);
        check_regex!(T, b"sampler", false);
        check_regex!(T, b"sAmpLer", false);
    }

    #[test]
    fn sequence_quality_regex() {
        type T = SequenceQualityRegex;

        // empty
        check_regex!(T, b"", false);

        // valid
        check_regex!(T, b";;;;;;;;;;;;;;;;;4;;;;3;393.1+4&&5&&;;;;;;;;;;;;;;;;;;;;;<9;<;;;;;464262", true);
        check_regex!(T, b"-;;;8;;;;;;;,*;;';-4,44;,:&,1,4'./&19;;;;;;669;;99;;;;;-;3;2;0;+;7442&2/", true);
        check_regex!(T, b"1;;;;;;,;;4;3;38;8%&,,;)*;1;;,)/%4+,;1;;);;;;;;;4;(;1;;;;24;;;;41-444//0", true);
        check_regex!(T, b";;;;;;;;;;;;;;;;;;;;;;;;;;;;;9445552;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;446662", true);

        // invalid (non-printables)
        check_regex!(T, b"\r\n", false);
    }

    #[test]
    fn fastq_header_regex() {
        type T = FastqHeaderRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        check_regex!(T, "@SRR390728.2 2 length=72", true);
        check_regex!(T, "+SRR390728.2 2 length=72", true);
        check_regex!(T, "@EAS139:136:FC706VJ:2:2104:15343:197393 1:N:18:1", true);
        check_regex!(T, "+EAS139:136:FC706VJ:2:2104:15343:197393 1:N:18:1", true);

        // We can't really invalid the regex, since any character is valid
        // in the second group.

        //extract
        extract_regex!(T, "@SRR390728.2 2 length=72", 1, "SRR390728.2", as_str);
        extract_regex!(T, "@SRR390728.2 2 length=72", 2, "2", as_str);
        extract_regex!(T, "@EAS139:136:FC706VJ:2:2104:15343:197393 1:N:18:1", 1, "EAS139:136:FC706VJ:2:2104:15343:197393", as_str);
        extract_regex!(T, "@EAS139:136:FC706VJ:2:2104:15343:197393 1:N:18:1", 2, "1:N:18:1", as_str);
    }
}
