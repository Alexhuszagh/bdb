//! Sequence Read Archive (SRA) integrations.

// Expose the low-level API in a public submodule.
pub mod low_level;

// Expose the client API in a public submodule.
// Requires the CSV feature to function.
#[cfg(all(feature = "csv", feature = "http"))]
pub mod client;

mod complete;
mod re;
mod record;
mod record_list;
mod valid;

#[cfg(test)]
mod test;

#[cfg(feature = "fastq")]
mod fastq;

// Re-export the models into the parent module.
pub use self::record::{Record, RecordField};
pub use self::record_list::RecordList;
