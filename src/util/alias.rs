//! Shared aliases.

use super::error::Error;
use std::result::Result as StdResult;

/// General buffer type.
pub type Buffer = Vec<u8>;

/// General result type.
pub type Result<T> = StdResult<T, Error>;
