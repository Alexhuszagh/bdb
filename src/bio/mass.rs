//! General purpose mass routines.

/// Calculate the mass of a biological sequence.
///
/// Different biological application depend on different assumptions for
/// mass calculations, some assuming average isotope composition and
/// some assuming the sole presence of monoisotopic species, and also
/// different biological molecules.
///
/// Calculating monoisotopic species uses a high-accuracy mass of the
/// most prevalent (and lowest mass) isotope of a given element.
/// The average mass calculates the mass of an element by summing the mass
/// of each isotope multiplied each isotope's abundance.
pub trait SequenceMass {
    /// Calculate the mass at the termini.
    fn termini_mass() -> f64;

    /// Calculate the mass of an internal residue.
    fn residue_mass(residue: u8) -> f64;

    /// Calculate the mass of an monomer with N- and C-termini.
    #[inline(always)]
    fn monomer_mass(residue: u8) -> f64 {
        Self::residue_mass(residue) + Self::termini_mass()
    }

    /// Calculate the mass of a protein sequence.
    #[inline]
    fn internal_sequence_mass(sequence: &[u8]) -> f64 {
        sequence.iter().fold(0.0, |sum, x| sum + Self::residue_mass(*x))
    }

    /// Calculate the mass of a protein sequence with N- or C-termini.
    #[inline(always)]
    fn total_sequence_mass(sequence: &[u8]) -> f64 {
        Self::internal_sequence_mass(sequence) + Self::termini_mass()
    }
}
