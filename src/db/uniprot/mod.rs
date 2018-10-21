//! UniProt integrations.

// Expose the low-level API in a public submodule.
pub mod low_level;
// Expose the client API in a public submodule.
pub mod client;

mod complete;
mod csv;
mod error;
mod evidence;
mod fasta;
mod re;
mod record;
mod record_list;
mod test;
mod valid;

// Re-export the models into the parent module.
pub use self::error::{UniProtError, UniProtErrorKind};
pub use self::evidence::ProteinEvidence;
pub use self::record::{Record, RecordField};
pub use self::record_list::RecordList;
