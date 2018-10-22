//! Sequence Read Archive (SRA) integrations.

mod client;
mod complete;
mod re;
mod record;
mod record_list;
mod valid;

// Re-export the models into the parent module.
// TODO(ahuszagh)
//      Restore
pub use self::record::{Record, RecordField};
pub use self::record_list::RecordList;
