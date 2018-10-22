//! Shared utilities.

// Don't export the modules publicly, these are implementation details
// We just need the high-level functionality made available.
#[macro_use]
mod macros;

mod alias;
mod error;

#[cfg(feature = "xml")]
mod xml;

pub use self::alias::{BufferType, ErrorType, ResultType};
pub use self::error::{Error, ErrorKind};

#[cfg(feature = "xml")]
pub use self::xml::{XmlReader, XmlWriter};
