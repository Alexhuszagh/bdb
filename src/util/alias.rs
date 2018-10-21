//! Shared aliases.

use std::error::Error as StdError;

/// General buffer type.
pub type BufferType = Vec<u8>;

/// General error type.
pub type ErrorType = Box<StdError>;

/// General result type.
pub type ResultType<T> = Result<T, ErrorType>;
