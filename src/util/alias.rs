//! Shared aliases.

use std::error::Error;

/// General error type.
pub type ErrorType = Box<Error>;

/// General result type.
pub type ResultType<T> = Result<T, ErrorType>;
