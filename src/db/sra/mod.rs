//! Sequence Read Archive (SRA) integrations.

// Expose the low-level API in a public submodule.
pub mod low_level;

// Expose the client API in a public submodule.
// Requires the CSV feature to function.
#[cfg(all(feature = "csv", feature = "http"))]
pub mod client;

pub(crate) mod complete;
pub(crate) mod re;
pub(crate) mod record;
pub(crate) mod record_list;
pub(crate) mod valid;

#[cfg(test)]
pub(crate) mod test;

#[cfg(feature = "fastq")]
pub(crate) mod fastq;

// Re-export the models into the parent module.
pub use self::record::Record;
pub use self::record_list::RecordList;
