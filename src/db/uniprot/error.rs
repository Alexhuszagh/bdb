//! Error definitions for UniProt models and services.

use std::error::Error;
use std::io;
use std::fmt;
use std::num::ParseIntError;
use std::str::Utf8Error;

#[cfg(feature = "xml")]
use quick_xml::Error as XmlError;

use util::ErrorType;

// TYPE

/// Error type.
#[derive(Debug)]
pub enum UniProtErrorKind {
    // PROTEIN EVIDENCE

    /// Deserializer fails due to improper number for protein evidence.
    ProteinEvidenceInvalidNumber,

    // RECORD

    /// Serializer fails due to invalid record data.
    InvalidRecord,
    /// Deserializer fails due to invalid or empty input data.
    InvalidInput,

    // FASTA

    /// Deserializer fails because the FASTA type is not recognized.
    InvalidFastaType,

    // INHERITED
    Io(io::Error),
    Utf8(Utf8Error),
    ParseInt(ParseIntError),

    #[cfg(feature = "xml")]
    Xml(XmlError),
}

// CONVERSIONS

impl From<io::Error> for UniProtError {
    fn from(err: io::Error) -> Self {
        UniProtError(UniProtErrorKind::Io(err))
    }
}

impl From<Utf8Error> for UniProtError {
    fn from(err: Utf8Error) -> Self {
        UniProtError(UniProtErrorKind::Utf8(err))
    }
}

impl From<ParseIntError> for UniProtError {
    fn from(err: ParseIntError) -> Self {
        UniProtError(UniProtErrorKind::ParseInt(err))
    }
}

#[cfg(feature = "xml")]
impl From<XmlError> for UniProtError {
    fn from(err: XmlError) -> Self {
        UniProtError(UniProtErrorKind::Xml(err))
    }
}

impl From<UniProtErrorKind> for UniProtError {
    fn from(kind: UniProtErrorKind) -> Self {
        UniProtError(kind)
    }
}

impl From<UniProtErrorKind> for ErrorType {
    fn from(kind: UniProtErrorKind) -> Self {
        Box::new(UniProtError(kind))
    }
}

// ERROR

/// Custom error for UniProt-related tasks.
///
/// Errors may occur during serializing/deserializing data, as well
/// as over network and file I/O.
#[derive(Debug)]
pub struct UniProtError(UniProtErrorKind);

impl UniProtError {
    /// Get error type.
    pub fn kind(&self) -> &UniProtErrorKind {
        &self.0
    }
}

impl fmt::Display for UniProtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UniProt error: {}", self.description())
    }
}

impl Error for UniProtError {
    fn description(&self) -> &str {
        match self.kind() {
            // PROTEIN EVIDENCE

            UniProtErrorKind::ProteinEvidenceInvalidNumber => {
                "out-of-range number found, cannot create ProteinEvidence."
            }

            // RECORD

            UniProtErrorKind::InvalidRecord => {
                "invalid record found, cannot serialize data"
            },
            UniProtErrorKind::InvalidInput => {
                "invalid input data, cannot deserialize data"
            },

            // FASTA

            UniProtErrorKind::InvalidFastaType => {
                "invalid FASTA type, cannot deserialize data"
            },

            // INHERITED
            UniProtErrorKind::Io(ref err) => err.description(),
            UniProtErrorKind::Utf8(ref err) => err.description(),
            UniProtErrorKind::ParseInt(ref err) => err.description(),

            #[cfg(feature = "xml")]
            UniProtErrorKind::Xml(ref err) => match err {
                XmlError::Io(ref e) => e.description(),
                XmlError::Utf8(ref e) => e.description(),
                XmlError::UnexpectedEof(_) => "xml: unexpected EOF",
                XmlError::EndEventMismatch {expected: _, found: _} => "xml: end event mismatch",
                XmlError::UnexpectedToken(_) => "xml: unexpected token",
                XmlError::UnexpectedBang => "xml: unexpected '!'",
                XmlError::TextNotFound => "xml: expected Event::Text",
                XmlError::XmlDeclWithoutVersion(_) => "xml: missing version in declaration",
                XmlError::NameWithQuote(_) => "xml: key cannot contain quote",
                XmlError::NoEqAfterName(_) => "xml: no '=' or ' '  after key",
                XmlError::UnquotedValue(_) => "xml: value is not quoted",
                XmlError::DuplicatedAttribute(_, _) => "xml: duplicate attribute found",
                XmlError::EscapeError(_) => "xml: escape error",
            },
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self.kind() {
            UniProtErrorKind::Io(ref err) => Some(err),
            UniProtErrorKind::Utf8(ref err) => Some(err),
            UniProtErrorKind::ParseInt(ref err) => Some(err),

            #[cfg(feature = "xml")]
            UniProtErrorKind::Xml(ref err) => match err {
                XmlError::Io(ref e) => Some(e),
                XmlError::Utf8(ref e) => Some(e),
                _  => None,
            },

            _ => None
        }
    }
}
