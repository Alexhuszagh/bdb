//! Shared traits.

pub(crate) mod complete;
pub(crate) mod fmt;
pub(crate) mod num;
pub(crate) mod parse;
pub(crate) mod valid;

#[cfg(feature = "csv")]
pub(crate) mod csv;

#[cfg(feature = "fasta")]
pub(crate) mod fasta;

#[cfg(feature = "fastq")]
pub(crate) mod fastq;

#[cfg(feature = "mgf")]
pub(crate) mod mgf;

#[cfg(feature = "xml")]
pub(crate) mod xml;

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

#[cfg(feature = "mgf")]
pub use self::mgf::{Mgf, MgfCollection, MgfKind};

#[cfg(feature = "xml")]
pub use self::xml::{Xml, XmlCollection};

// Export for internal use only.
pub(crate) use self::fmt::Serializable;
pub(crate) use self::num::*;
pub(crate) use self::parse::Deserializable;
