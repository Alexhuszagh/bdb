//! UniProt integrations.

// Expose the low-level API in a public submodule.
pub mod low_level;

// Expose the client API in a public submodule.
// Requires the CSV feature to function.
#[cfg(all(feature = "csv", feature = "http"))]
pub mod client;

mod complete;
mod error;
mod evidence;
mod re;
mod record;
mod record_list;
mod test;
mod valid;

#[cfg(feature = "csv")]
mod csv;

#[cfg(feature = "fasta")]
mod fasta;

#[cfg(feature = "xml")]
mod xml;

// Re-export the models into the parent module.
pub use self::error::{UniProtError, UniProtErrorKind};
pub use self::evidence::ProteinEvidence;
pub use self::record::{Record, RecordField};
pub use self::record_list::RecordList;
