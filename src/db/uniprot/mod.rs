//! UniProt integrations.

mod error;
mod record;
mod util;
// TODO(ahuszagh)
//      mod record_list;
//      etc...

pub use self::error::{UniProtError, UniProtErrorKind};
pub use self::record::{ProteinEvidence, protein_evidence_verbose, Record};
