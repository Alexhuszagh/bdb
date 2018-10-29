//! Biological molecule definitions.

mod mass;

// Expose biological molecules in public submodules.
pub mod dna;
pub mod proteins;
pub mod rna;

// Publicly re-export the SequenceMass.
pub use self::mass::SequenceMass;
