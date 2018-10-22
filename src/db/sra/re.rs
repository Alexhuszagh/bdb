//! Regular expression utilities for SRA services.
//!
//! Disable Unicode for all but the generic header formats, which may
//! accept arbitrary Unicode input. The rest should only be valid ASCII,
//! and therefore we should disable matching to Unicode characters
//! explicitly.

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
pub struct SequenceQualityRegex;

impl ValidationRegex<BytesRegex> for SequenceQualityRegex {
    fn validate() -> &'static BytesRegex {
        lazy_regex!(BytesRegex, r"(?-u)(?x)
            \A
            (?:
                # TODO(ahuszagh) Implement
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
                # TODO(ahuszagh) Implement
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
    // TODO(ahuszagh)
    //      Implement....
}
