//! Model for UniProt protein evidence.

use traits::Ntoa;
use util::{ErrorKind, Result};
use std::mem;

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
/// More documentation can be found [`here`].
///
/// [`here`]: https://www.uniprot.org/help/protein_existence
///
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ProteinEvidence {
    /// Protein evidence at the protein level.
    ProteinLevel = 1,
    /// Protein evidence at the transcript (mRNA) level.
    TranscriptLevel = 2,
    /// Protein is inferred from homology.
    Inferred = 3,
    /// Protein is predicted bioinformatically.
    Predicted = 4,
    /// Internal implementation detail.
    #[doc(hidden)]
    Unknown = 5,
}

impl ProteinEvidence {
    /// Verbose constant messages.
    const PROTEIN_LEVEL_VERBOSE: &'static str = "Evidence at protein level";
    const TRANSCRIPT_LEVEL_VERBOSE: &'static str = "Evidence at transcript level";
    const INFERRED_LEVEL_VERBOSE: &'static str = "Inferred from homology";
    const PREDICTED_LEVEL_VERBOSE: &'static str = "Predicted";
    const UNKNOWN_LEVEL_VERBOSE: &'static str = "";

    /// Minimum and maximum bounds on the enumeration.
    const MIN: u8 = 1;
    const MAX: u8 = 5;

    /// Convert enumerated value for ProteinEvidence to verbose text.
    #[inline]
    pub fn verbose(&self) -> &'static str {
        match self {
            ProteinEvidence::ProteinLevel       => Self::PROTEIN_LEVEL_VERBOSE,
            ProteinEvidence::TranscriptLevel    => Self::TRANSCRIPT_LEVEL_VERBOSE,
            ProteinEvidence::Inferred           => Self::INFERRED_LEVEL_VERBOSE,
            ProteinEvidence::Predicted          => Self::PREDICTED_LEVEL_VERBOSE,
            ProteinEvidence::Unknown            => Self::UNKNOWN_LEVEL_VERBOSE,
        }
    }

    /// Create enumerated value from verbose text.
    #[inline]
    pub fn from_verbose(text: &str) -> Result<Self> {
        match text {
            Self::PROTEIN_LEVEL_VERBOSE      => Ok(ProteinEvidence::ProteinLevel),
            Self::TRANSCRIPT_LEVEL_VERBOSE   => Ok(ProteinEvidence::TranscriptLevel),
            Self::INFERRED_LEVEL_VERBOSE     => Ok(ProteinEvidence::Inferred),
            Self::PREDICTED_LEVEL_VERBOSE    => Ok(ProteinEvidence::Predicted),
            Self::UNKNOWN_LEVEL_VERBOSE      => Ok(ProteinEvidence::Unknown),
            _                                => Err(From::from(ErrorKind::InvalidEnumeration)),
        }
    }

    /// Create raw integer from enumerated value.
    #[inline]
    pub fn to_int(&self) -> u8 {
        *self as u8
    }

    /// Create enumerated value (like C) from raw integer.
    #[inline]
    pub fn from_int(int: u8) -> Result<Self> {
        if int >= Self::MIN && int <= Self::MAX {
            Ok(unsafe { mem::transmute(int) })
        } else {
            Err(From::from(ErrorKind::InvalidEnumeration))
        }
    }

    /// Create string from an enumerated value.
    #[inline(always)]
    pub fn to_string(&self) -> String {
        self.to_int().to_string()
    }

    /// Create enumerated value from str.
    #[inline(always)]
    pub fn from_str(s: &str) -> Result<Self> {
        ProteinEvidence::from_int(s.parse::<u8>()?)
    }
}

#[cfg(feature = "xml")]
impl ProteinEvidence {
    /// XML Verbose constant messages.
    const PROTEIN_LEVEL_XML_VERBOSE: &'static str = "evidence at protein level";
    const TRANSCRIPT_LEVEL_XML_VERBOSE: &'static str = "evidence at transcript level";
    const INFERRED_LEVEL_XML_VERBOSE: &'static str = "inferred from homology";
    const PREDICTED_LEVEL_XML_VERBOSE: &'static str = "predicted";
    const UNKNOWN_LEVEL_XML_VERBOSE: &'static str = "";

    /// Convert enumerated value for ProteinEvidence to XML verbose text.
    #[inline]
    pub fn xml_verbose(&self) -> &'static str {
        match self {
            ProteinEvidence::ProteinLevel       => Self::PROTEIN_LEVEL_XML_VERBOSE,
            ProteinEvidence::TranscriptLevel    => Self::TRANSCRIPT_LEVEL_XML_VERBOSE,
            ProteinEvidence::Inferred           => Self::INFERRED_LEVEL_XML_VERBOSE,
            ProteinEvidence::Predicted          => Self::PREDICTED_LEVEL_XML_VERBOSE,
            ProteinEvidence::Unknown            => Self::UNKNOWN_LEVEL_XML_VERBOSE,
        }
    }

    /// Create enumerated value from XML verbose text.
    #[inline]
    pub fn from_xml_verbose(text: &str) -> Result<Self> {
        match text {
            Self::PROTEIN_LEVEL_XML_VERBOSE      => Ok(ProteinEvidence::ProteinLevel),
            Self::TRANSCRIPT_LEVEL_XML_VERBOSE   => Ok(ProteinEvidence::TranscriptLevel),
            Self::INFERRED_LEVEL_XML_VERBOSE     => Ok(ProteinEvidence::Inferred),
            Self::PREDICTED_LEVEL_XML_VERBOSE    => Ok(ProteinEvidence::Predicted),
            Self::UNKNOWN_LEVEL_XML_VERBOSE      => Ok(ProteinEvidence::Unknown),
            _                                    => Err(From::from(ErrorKind::InvalidEnumeration)),
        }
    }
}

impl Ntoa for ProteinEvidence {
    #[inline(always)]
    fn ntoa(&self) -> Result<String> {
        self.to_int().ntoa()
    }

    #[inline(always)]
    fn ntoa_with_capacity(&self, capacity: usize) -> Result<String> {
        self.to_int().ntoa_with_capacity(capacity)
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    // PROTEIN EVIDENCE
    // Note: Do not test Unknown, it is an implementation detail.

    #[test]
    fn debug_protein_evidence_test() {
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
    fn protein_evidence_verbose_test() {
        // ProteinLevel
        let text = ProteinEvidence::ProteinLevel.verbose();
        assert_eq!(text, "Evidence at protein level");

        // TranscriptLevel
        let text = ProteinEvidence::TranscriptLevel.verbose();
        assert_eq!(text, "Evidence at transcript level");

        // Inferred
        let text = ProteinEvidence::Inferred.verbose();
        assert_eq!(text, "Inferred from homology");

        // Predicted
        let text = ProteinEvidence::Predicted.verbose();
        assert_eq!(text, "Predicted");
    }

    #[cfg(feature = "xml")]
    #[test]
    fn protein_evidence_xml_verbose_test() {
        // ProteinLevel
        let text = ProteinEvidence::ProteinLevel.xml_verbose();
        assert_eq!(text, "evidence at protein level");

        // TranscriptLevel
        let text = ProteinEvidence::TranscriptLevel.xml_verbose();
        assert_eq!(text, "evidence at transcript level");

        // Inferred
        let text = ProteinEvidence::Inferred.xml_verbose();
        assert_eq!(text, "inferred from homology");

        // Predicted
        let text = ProteinEvidence::Predicted.xml_verbose();
        assert_eq!(text, "predicted");
    }

    fn serialize_protein_evidence(evidence: ProteinEvidence, expected: &str) {
        let text = evidence.to_string();
        assert_eq!(text, expected);
        let result = ProteinEvidence::from_str(&text).unwrap();
        assert_eq!(result, evidence);

        let text = evidence.ntoa().unwrap();
        assert_eq!(text, expected);
    }

    #[test]
    fn serialize_protein_evidence_test() {
        serialize_protein_evidence(ProteinEvidence::ProteinLevel, "1");
        serialize_protein_evidence(ProteinEvidence::TranscriptLevel, "2");
        serialize_protein_evidence(ProteinEvidence::Inferred, "3");
        serialize_protein_evidence(ProteinEvidence::Predicted, "4");
    }
}
