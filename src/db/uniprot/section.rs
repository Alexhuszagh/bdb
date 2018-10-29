//! Model for Uniprot protein section type.

use std::mem;

use traits::{Deserializable, Serializable, Zero};
use util::{Bytes, ErrorKind, Result};

/// Identifier for the section type of a UniProt record.
///
/// UniProt datasets are split into two sections, Swiss-Prot and TrEMBL.
/// Due to the copious time required to annotate protein sequences,
/// a high-quality, computationally-derived databases was added to UniProt
/// to predict proteins from genomic workflows.
///
/// More documentation can be found [`here`].
///
/// [`here`]: https://www.uniprot.org/help/uniprotkb_sections
///
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Section {
    /// High quality, computationally analyzed protein sequence database.
    TrEMBL = 0,
    /// Manually curated protein sequence database.
    SwissProt = 1,
    /// Internal implementation detail.
    #[doc(hidden)]
    Unknown = 2
}

impl Section {
    /// Minimum and maximum bounds on the enumeration.
    const MIN: u8 = 0;
    const MAX: u8 = 2;

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
}

impl Zero for Section {
    #[inline(always)]
    fn zero() -> Self {
        Section::TrEMBL
    }
}

impl Serializable for Section {
    #[inline(always)]
    fn export_bytes(&self) -> Result<Bytes> {
        self.to_int().export_bytes()
    }
}

impl Deserializable for Section {
    #[inline(always)]
    fn import_bytes(bytes: &[u8]) -> Result<Self> {
        Section::from_int(u8::import_bytes(bytes)?)
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;
    use util::{from_string, to_string};

    // SECTION
    // Note: Do not test Unknown, it is an implementation detail.

    #[test]
    fn debug_section_test() {
        // TrEMBL
        let text = format!("{:?}", Section::TrEMBL);
        assert_eq!(text, "TrEMBL");

        // SwissProt
        let text = format!("{:?}", Section::SwissProt);
        assert_eq!(text, "SwissProt");
    }

    fn serialize_section(section: Section, expected: &str) {
        let text = to_string(&section).unwrap();
        assert_eq!(text, expected);
        let result = from_string::<Section>(&text).unwrap();
        assert_eq!(result, section);

        let text = to_string(&section).unwrap();
        assert_eq!(text, expected);
    }

    #[test]
    fn serialize_section_test() {
        serialize_section(Section::TrEMBL, "0");
        serialize_section(Section::SwissProt, "1");
    }
}
