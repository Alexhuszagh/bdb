//! Model for Uniprot protein section type.

use traits::Ntoa;
use util::{ErrorKind, Result};
use std::mem;

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

    /// Create string from an enumerated value.
    #[inline(always)]
    pub fn to_string(&self) -> String {
        self.to_int().to_string()
    }

    /// Create enumerated value from str.
    #[inline(always)]
    pub fn from_str(s: &str) -> Result<Self> {
        Section::from_int(s.parse::<u8>()?)
    }
}

impl Ntoa for Section {
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
        let text = section.to_string();
        assert_eq!(text, expected);
        let result = Section::from_str(&text).unwrap();
        assert_eq!(result, section);

        let text = section.ntoa().unwrap();
        assert_eq!(text, expected);
    }

    #[test]
    fn serialize_section_test() {
        serialize_section(Section::TrEMBL, "0");
        serialize_section(Section::SwissProt, "1");
    }
}
