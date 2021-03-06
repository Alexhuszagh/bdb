//! Error definitions for UniProt models and services.

use std::error::Error as StdError;
use std::io;
use std::fmt;
use std::num::ParseFloatError;
use std::num::ParseIntError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

#[cfg(feature = "csv")]
use csv::Error as CsvError;

#[cfg(feature = "http")]
use reqwest::Error as HttpError;

#[cfg(feature = "xml")]
use quick_xml::Error as XmlError;

// TYPE

/// Enumerated error type during BDB error handling.
#[derive(Debug)]
pub enum ErrorKind {
    // ENUMERATION

    /// Enumeration creation fails due to invalid value.
    InvalidEnumeration,

    // RECORD

    /// Serializer fails due to invalid record data.
    InvalidRecord,

    // DESERIALIZER

    /// Deserializer fails due to invalid or empty input data.
    InvalidInput,
    /// Deserializer fails because the FASTA type is not recognized.
    InvalidFastaFormat,
    /// Deserializer fails because of an unexpected EOF.
    UnexpectedEof,

    // INHERITED
    /// Inherited `io::Error`.
    Io(io::Error),
    /// Inherited `Utf8Error`.
    Utf8(Utf8Error),
    /// Inherited `FromUtf8Error`.
    FromUtf8(FromUtf8Error),
    /// Inherited `ParseIntError`.
    ParseInt(ParseIntError),
    /// Inherited `ParseFloatError`.
    ParseFloat(ParseFloatError),

    /// Inherited `csv::Error`.
    #[cfg(feature = "csv")]
    Csv(CsvError),

    /// Inherited `reqwest::Error`.
    #[cfg(feature = "http")]
    Http(HttpError),

    /// Inherited `quick_xml::Error`.
    #[cfg(feature = "xml")]
    Xml(XmlError),
}

// CONVERSIONS

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error(ErrorKind::Io(err))
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error(ErrorKind::Utf8(err))
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error(ErrorKind::FromUtf8(err))
    }
}

impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Self {
        Error(ErrorKind::ParseFloat(err))
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error(ErrorKind::ParseInt(err))
    }
}

#[cfg(feature = "csv")]
impl From<CsvError> for Error {
    fn from(err: CsvError) -> Self {
        Error(ErrorKind::Csv(err))
    }
}

#[cfg(feature = "http")]
impl From<HttpError> for Error {
    fn from(err: HttpError) -> Self {
        Error(ErrorKind::Http(err))
    }
}

#[cfg(feature = "xml")]
impl From<XmlError> for Error {
    fn from(err: XmlError) -> Self {
        Error(ErrorKind::Xml(err))
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error(kind)
    }
}

// ERROR

/// Custom error for BDB-related tasks.
///
/// Errors may occur during serializing/deserializing data, as well
/// as over network and file I/O.
#[derive(Debug)]
pub struct Error(ErrorKind);

impl Error {
    /// Get error type.
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UniProt error: {}", self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self.kind() {
            // PROTEIN EVIDENCE

            ErrorKind::InvalidEnumeration => {
                "out-of-range value found, cannot create enumeration"
            }

            // RECORD

            ErrorKind::InvalidRecord => {
                "invalid record found, cannot write data"
            },

            // DESERIALIZER

            ErrorKind::InvalidInput => {
                "invalid input data, cannot read data"
            },
            ErrorKind::InvalidFastaFormat => {
                "invalid FASTA type, cannot read data"
            },
            ErrorKind::UnexpectedEof => {
                "unexpected EOF, cannot read data"
            }

            // INHERITED
            ErrorKind::Io(ref err) => err.description(),
            ErrorKind::Utf8(ref err) => err.description(),
            ErrorKind::FromUtf8(ref err) => err.description(),
            ErrorKind::ParseFloat(ref err) => err.description(),
            ErrorKind::ParseInt(ref err) => err.description(),

            #[cfg(feature = "csv")]
            ErrorKind::Csv(ref err) => err.description(),

            #[cfg(feature = "http")]
            ErrorKind::Http(ref err) => err.description(),

            #[cfg(feature = "xml")]
            ErrorKind::Xml(ref err) => match err {
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

    fn cause(&self) -> Option<&StdError> {
        match self.kind() {
            ErrorKind::Io(ref err) => Some(err),
            ErrorKind::Utf8(ref err) => Some(err),
            ErrorKind::FromUtf8(ref err) => Some(err),
            ErrorKind::ParseFloat(ref err) => Some(err),
            ErrorKind::ParseInt(ref err) => Some(err),

            #[cfg(feature = "csv")]
            ErrorKind::Csv(ref err) => err.cause(),

            #[cfg(feature = "http")]
            ErrorKind::Http(ref err) => err.cause(),

            #[cfg(feature = "xml")]
            ErrorKind::Xml(ref err) => match err {
                XmlError::Io(ref e) => Some(e),
                XmlError::Utf8(ref e) => Some(e),
                _  => None,
            },

            _ => None
        }
    }
}
