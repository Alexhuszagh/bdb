//! Shared utilities.

// Don't export the modules publicly, these are implementation details
// We just need the high-level functionality made available.
#[macro_use]
pub(crate) mod macros;

#[macro_use]
pub(crate) mod iterator;

#[macro_use]
pub(crate) mod re;

pub(crate) mod alias;
pub(crate) mod error;
pub(crate) mod fmt;
pub(crate) mod parse;
pub(crate) mod search;
pub(crate) mod writer;

#[cfg(feature = "xml")]
pub(crate) mod xml;

// Export low-level converters internally.
pub(crate) use self::fmt::*;
pub(crate) use self::iterator::*;
pub(crate) use self::parse::*;
pub(crate) use self::re::*;
pub(crate) use self::writer::TextWriterState;

#[cfg(feature = "xml")]
pub(crate) use self::xml::{XmlReader, XmlWriter};

// Publicly expose high-level APIs.
pub use self::alias::{Bytes, Result};
pub use self::error::{Error, ErrorKind};
