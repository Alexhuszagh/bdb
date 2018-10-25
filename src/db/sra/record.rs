//! Model for SRA (Sequence Read Archive) read definitions.

/// Enumerated values for Record fields.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[allow(dead_code)]     // TODO(ahuszagh)       Remove
pub enum RecordField {
    SeqId,
    Description,
    Length,
    Sequence,
    Quality,
}

/// Model for a single record from a sequence read.
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Record {
    /// Sequence identifier for the read.
    pub seq_id: String,
    /// Description for the sequence identifier.
    pub description: String,
    /// Read length.
    pub length: u32,
    /// Nucleotide sequence.
    pub sequence: Vec<u8>,
    /// Nucleotide sequence quality scores.
    pub quality: Vec<u8>,
}

impl Record {
    /// Create new, empty SRA record.
    #[inline]
    pub fn new() -> Self {
        Record {
            seq_id: String::new(),
            description: String::new(),
            length: 0,
            sequence: vec![],
            quality: vec![],
        }
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use traits::*;
    //use super::*;
    use super::super::test::*;

    #[test]
    fn debug_record() {
        let text = format!("{:?}", srr390728_2());
        assert_eq!(text, "Record { seq_id: \"SRR390728.2\", description: \"2\", length: 72, sequence: [65, 65, 71, 84, 65, 71, 71, 84, 67, 84, 67, 71, 84, 67, 84, 71, 84, 71, 84, 84, 84, 84, 67, 84, 65, 67, 71, 65, 71, 67, 84, 84, 71, 84, 71, 84, 84, 67, 67, 65, 71, 67, 84, 71, 65, 67, 67, 67, 65, 67, 84, 67, 67, 67, 84, 71, 71, 71, 84, 71, 71, 71, 71, 71, 71, 65, 67, 84, 71, 71, 71, 84], quality: [59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 52, 59, 59, 59, 59, 51, 59, 51, 57, 51, 46, 49, 43, 52, 38, 38, 53, 38, 38, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 60, 57, 59, 60, 59, 59, 59, 59, 59, 52, 54, 52, 50, 54, 50] }");

        let text = format!("{:?}", srr390728_3());
        assert_eq!(text, "Record { seq_id: \"SRR390728.3\", description: \"3\", length: 72, sequence: [67, 67, 65, 71, 67, 67, 84, 71, 71, 67, 67, 65, 65, 67, 65, 71, 65, 71, 84, 71, 84, 84, 65, 67, 67, 67, 67, 71, 84, 84, 84, 84, 84, 65, 67, 84, 84, 65, 84, 84, 84, 65, 84, 84, 65, 84, 84, 65, 84, 84, 65, 84, 84, 84, 84, 71, 65, 71, 65, 67, 65, 71, 65, 71, 67, 65, 84, 84, 71, 71, 84, 67], quality: [45, 59, 59, 59, 56, 59, 59, 59, 59, 59, 59, 59, 44, 42, 59, 59, 39, 59, 45, 52, 44, 52, 52, 59, 44, 58, 38, 44, 49, 44, 52, 39, 46, 47, 38, 49, 57, 59, 59, 59, 59, 59, 59, 54, 54, 57, 59, 59, 57, 57, 59, 59, 59, 59, 59, 45, 59, 51, 59, 50, 59, 48, 59, 43, 59, 55, 52, 52, 50, 38, 50, 47] }");
    }

    #[test]
    fn equality_record() {
        let x = srr390728_2();
        let y = srr390728_2();
        let z = srr390728_3();
        assert_eq!(x, y);
        assert_ne!(x, z);
        assert_ne!(y, z);
    }

    #[test]
    fn properties_record() {
        // test various permutations that can lead to
        // invalid or incomplete identifications
        let g1 = srr390728_2();
        let mut g2 = g1.clone();
        assert!(g2.is_valid());
        assert!(g2.is_complete());

        // check keeping the protein valid but make it incomplete
        g2.description = String::new();
        assert!(g2.is_valid());
        assert!(!g2.is_complete());
        g2.description = g1.description.clone();

        // check replacing items with invalid data
        g2.sequence = b"AAGUAGGUCUCGUCUGUGUUUUCUACGAGCUUGUGUUCCAGCUGACCCACUCCCUGGGUGGGGGGACUGGGU".to_vec();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.sequence = g1.sequence.clone();

        // calculate the shift amount
        g2.quality = b"AAGUAGGUCUCGUCUGUGUUUUCUACGAGCUUGUGUUCCAGCUGACCCACUCCCUGGGUGGGGGGACUGGGU".to_vec();
        let min: u8 = *g2.quality.iter().min().unwrap();
        g2.quality.iter_mut().for_each(|c| *c -= min - 1);
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.sequence = g1.sequence.clone();

    }

    // TODO(ahuszagh)
    //      implement...
}
