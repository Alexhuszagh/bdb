//! Shared utilities.

// Don't export the modules publicly, these are implementation details
// We just need the high-level functionality made available.
#[macro_use]
mod macros;

#[macro_use]
mod re;

mod alias;
mod error;
mod iterator;
mod writer;

#[cfg(feature = "xml")]
mod xml;

pub use self::alias::{BufferType, ErrorType, ResultType};
pub use self::error::{Error, ErrorKind};
pub use self::iterator::{LenientIter, StrictIter};
pub use self::re::{ExtractionRegex, ValidationRegex};
pub use self::writer::TextWriterState;

#[cfg(feature = "xml")]
pub use self::xml::{XmlReader, XmlWriter};
