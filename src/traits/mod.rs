//! Shared traits.

mod complete;
mod csv;
mod fasta;
mod text;
mod valid;
mod xml;

// Serialization Traits
pub use self::csv::{Csv, CsvCollection};
pub use self::fasta::{Fasta, FastaCollection};
pub use self::text::{Text, TextCollection};
pub use self::xml::{Xml};

// Record validation traits
pub use self::complete::{Complete};
pub use self::valid::{Valid};
