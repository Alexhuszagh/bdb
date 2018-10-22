//! Shared traits.

mod complete;
mod valid;

#[cfg(feature = "csv")]
mod csv;

#[cfg(feature = "fasta")]
mod fasta;

#[cfg(feature = "fastq")]
mod fastq;

#[cfg(feature = "xml")]
mod xml;

// Record validation traits
pub use self::complete::{Complete};
pub use self::valid::{Valid};

// Serialization Traits
#[cfg(feature = "csv")]
pub use self::csv::{Csv, CsvCollection};

#[cfg(feature = "fasta")]
pub use self::fasta::{Fasta, FastaCollection};

#[cfg(feature = "fastq")]
pub use self::fastq::{Fastq, FastqCollection};

#[cfg(feature = "xml")]
pub use self::xml::{Xml, XmlCollection};
