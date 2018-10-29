//! UniProt integrations.

// Expose the low-level API in a public submodule.
pub mod low_level;

// Expose the client API in a public submodule.
// Requires the CSV feature to function.
#[cfg(all(feature = "csv", feature = "http"))]
pub mod client;

pub(crate) mod complete;
pub(crate) mod evidence;
pub(crate) mod re;
pub(crate) mod record;
pub(crate) mod record_list;
pub(crate) mod section;
pub(crate) mod valid;

#[cfg(feature = "csv")]
pub(crate) mod csv;

#[cfg(feature = "fasta")]
pub(crate) mod fasta;

#[cfg(feature = "xml")]
pub(crate) mod xml;

#[cfg(test)]
pub(crate) mod test;

// Re-export the models into the parent module.
pub use self::evidence::ProteinEvidence;
pub use self::record::{Record, RecordField};
pub use self::record_list::RecordList;
pub use self::section::Section;
