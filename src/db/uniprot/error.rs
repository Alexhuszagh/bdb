//! Error definitions for UniProt models and services.

use std::error::Error;
use std::fmt;

/// Error type.
#[derive(Debug, Clone)]
pub enum UniProtErrorKind {
    /// Serializer fails due to invalid record data.
    InvalidRecord,
    /// Deserializer fails due to invalid or empty input data.
    InvalidInputData,
}

// Private constructor for `Box<UniProtError>`.
pub fn new_boxed_error(kind: UniProtErrorKind) -> Box<UniProtError> {
    Box::new(UniProtError(kind))
}


/// Custom error for UniProt-related tasks.
///
/// Errors may occur during serializing/deserializing data, as well
/// as over network and file I/O.
#[derive(Debug, Clone)]
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
        match &self.0 {
            UniProtErrorKind::InvalidRecord     => {
                "invalid record found, cannot serialize data"
            },
            UniProtErrorKind::InvalidInputData  => {
                "invalid input data, cannot deserialize data"
            },
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
