//! UniProt integrations.

// TODO(ahuszagh)
//      Restore
//mod client;
mod complete;
mod csv;
mod error;
mod evidence;
mod fasta;
mod iterator;
mod re;
mod record;
mod record_list;
mod test;
mod valid;

//pub use self::client::{by_id, by_id_list, by_mnemonic, by_mnemonic_list};
pub use self::error::{UniProtError, UniProtErrorKind};
pub use self::evidence::ProteinEvidence;
pub use self::iterator::{RecordIterator, ReferenceRecordIterator, ValueRecordIterator};
pub use self::record::{Record, RecordField};
pub use self::record_list::RecordList;
