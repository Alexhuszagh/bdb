//! UniProt integrations.

mod client;
mod csv;
mod error;
mod record;
mod record_list;
mod test;

pub use self::client::{by_id, by_id_list, by_mnemonic, by_mnemonic_list};
pub use self::error::{UniProtError, UniProtErrorKind};
pub use self::record::{ProteinEvidence, protein_evidence_verbose, Record};
pub use self::record_list::{RecordList};
