//! General purpose protein routines.
//!
//! Masses are valid for low-pH LC-MS.

use super::mass::SequenceMass;

/// Valid aminoacid 1-letter codes.
pub const MONOMERS: &'static str = "ABCDEFGHIJKLMNPQRSTVWXYZ";

/// Calculate protein mass using only high-resolution masses from monoisotopic elements.
pub struct MonoisotopicMass;

impl SequenceMass for MonoisotopicMass {
    #[inline(always)]
    fn termini_mass() -> f64 {
        18.0105646942
    }

    #[inline]
    fn residue_mass(residue: u8) -> f64 {
        match residue {
            // uppercase
            b'A' => 71.0371137957,
            b'C' => 103.0091844957,
            b'D' => 115.0269430557,
            b'E' => 129.0425931199,
            b'F' => 147.0684139241,
            b'G' => 57.0214637315,
            b'H' => 137.0589118703,
            b'I' => 113.0840639883,
            b'K' => 128.0949630256,
            b'L' => 113.0840639883,
            b'M' => 131.0404846241,
            b'N' => 114.042927463,
            b'P' => 97.0527638599,
            b'Q' => 128.0585775272,
            b'R' => 156.101111036,
            b'S' => 87.0320284257,
            b'T' => 101.0476784899,
            b'U' => 150.9536347957,
            b'V' => 99.0684139241,
            b'W' => 186.0793129614,
            b'Y' => 163.0633285541,
            // lowercase
            b'a' => 71.0371137957,
            b'c' => 103.0091844957,
            b'd' => 115.0269430557,
            b'e' => 129.0425931199,
            b'f' => 147.0684139241,
            b'g' => 57.0214637315,
            b'h' => 137.0589118703,
            b'i' => 113.0840639883,
            b'k' => 128.0949630256,
            b'l' => 113.0840639883,
            b'm' => 131.0404846241,
            b'n' => 114.042927463,
            b'p' => 97.0527638599,
            b'q' => 128.0585775272,
            b'r' => 156.101111036,
            b's' => 87.0320284257,
            b't' => 101.0476784899,
            b'u' => 150.9536347957,
            b'v' => 99.0684139241,
            b'w' => 186.0793129614,
            b'y' => 163.0633285541,
            // default
            _    => 0.0,
        }
    }
}


/// Calculate protein mass using only low-resolution masses from average isotopic compositions.
pub struct AverageMass;

impl SequenceMass for AverageMass {
    #[inline(always)]
    fn termini_mass() -> f64 {
        18.015
    }

    #[inline]
    fn residue_mass(residue: u8) -> f64 {
        match residue {
            // uppercase
            b'A' => 71.0779,
            b'C' => 103.1429,
            b'D' => 115.0874,
            b'E' => 129.114,
            b'F' => 147.1739,
            b'G' => 57.0513,
            b'H' => 137.1393,
            b'I' => 113.1576,
            b'K' => 128.1723,
            b'L' => 113.1576,
            b'M' => 131.1961,
            b'N' => 114.1026,
            b'P' => 97.1152,
            b'Q' => 128.1292,
            b'R' => 156.1857,
            b'S' => 87.0773,
            b'T' => 101.1039,
            b'U' => 150.0379,
            b'V' => 99.1311,
            b'W' => 186.2099,
            b'Y' => 163.1733,
            // lowercase
            b'a' => 71.0779,
            b'c' => 103.1429,
            b'd' => 115.0874,
            b'e' => 129.114,
            b'f' => 147.1739,
            b'g' => 57.0513,
            b'h' => 137.1393,
            b'i' => 113.1576,
            b'k' => 128.1723,
            b'l' => 113.1576,
            b'm' => 131.1961,
            b'n' => 114.1026,
            b'p' => 97.1152,
            b'q' => 128.1292,
            b'r' => 156.1857,
            b's' => 87.0773,
            b't' => 101.1039,
            b'u' => 150.0379,
            b'v' => 99.1311,
            b'w' => 186.2099,
            b'y' => 163.1733,
            // default
            _    => 0.0,
        }
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    // AMINOACID

    fn one_letter_mass<T: SequenceMass>() {
        // shorthand for `to_ascii_lowercase`
        let lower = | a: u8 | a.to_ascii_lowercase();

        // check all uppercase and lowercase items are identical
        for a in MONOMERS.bytes() {
            assert_eq!(T::residue_mass(a), T::residue_mass(lower(a)));
            assert_eq!(T::monomer_mass(a), T::monomer_mass(lower(a)));
        }
    }

    #[test]
    fn one_letter_mass_test() {
        pub type A = AverageMass;
        pub type M = MonoisotopicMass;

        // check approximate monoisotopic masses
        // average to monoisotopic should be within 0.2
        for a in MONOMERS.bytes() {
            assert_approx_eq!(A::residue_mass(a), M::residue_mass(a), 0.2);
        }

        one_letter_mass::<MonoisotopicMass>();
        one_letter_mass::<AverageMass>();
    }

    // SEQUENCE

    #[test]
    fn sequence_mass_average_test() {
        // use common sequences to check whether the aminoacid masses
        // are correct values
        pub type T = AverageMass;

        let peptide = b"SAMPLER";
        assert_approx_eq!(T::internal_sequence_mass(peptide), 784.9238,    0.001);
        assert_approx_eq!(T::total_sequence_mass(peptide),  802.9388,    0.001);

        let peptide = b"TGPNLHGLFGR";
        assert_approx_eq!(T::internal_sequence_mass(peptide), 1150.2897,   0.001);
        assert_approx_eq!(T::total_sequence_mass(peptide),  1168.3047,   0.001);

        let peptide = b"ACDEFGHIKLMNPQRSTUVWY";
        assert_approx_eq!(T::internal_sequence_mass(peptide), 2527.7364,   0.001);
        assert_approx_eq!(T::total_sequence_mass(peptide),  2545.7514,   0.001);
    }

    #[test]
    fn sequence_mass_monoisotopic_test() {
        // use common sequences to check whether the aminoacid masses
        // are correct values
        pub type T = MonoisotopicMass;

        let peptide = b"SAMPLER";
        assert_approx_eq!(T::internal_sequence_mass(peptide), 784.39016,    0.001);
        assert_approx_eq!(T::total_sequence_mass(peptide),  802.4007,     0.001);

        let peptide = b"TGPNLHGLFGR";
        assert_approx_eq!(T::internal_sequence_mass(peptide), 1149.60433,   0.001);
        assert_approx_eq!(T::total_sequence_mass(peptide),  1167.61489,   0.001);

        let peptide = b"ACDEFGHIKLMNPQRSTUVWY";
        assert_approx_eq!(T::internal_sequence_mass(peptide), 2527.067977,  0.001);
        assert_approx_eq!(T::total_sequence_mass(peptide),  2545.0785414, 0.001);
    }
}
