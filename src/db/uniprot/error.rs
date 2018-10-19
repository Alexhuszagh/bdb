//! Error definitions for UniProt models and services.

use std::error::Error;
use std::io;
use std::fmt;
use std::num::ParseIntError;
use std::str::Utf8Error;

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
    InvalidInputData,

    // INHERITED
    Io(io::Error),
    Utf8(Utf8Error),
    ParseInt(ParseIntError),
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
            UniProtErrorKind::InvalidInputData => {
                "invalid input data, cannot deserialize data"
            },

            // INHERITED
            UniProtErrorKind::Io(ref err) => err.description(),
            UniProtErrorKind::Utf8(ref err) => err.description(),
            UniProtErrorKind::ParseInt(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self.kind() {
            UniProtErrorKind::Io(ref err) => Some(err),
            UniProtErrorKind::Utf8(ref err) => Some(err),
            UniProtErrorKind::ParseInt(ref err) => Some(err),
            _ => None
        }
    }
}
