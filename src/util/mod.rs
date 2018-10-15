//! Shared utilities.

// Don't export the modules publicly, these are implementation details
// We just need the high-level functionality made available.
mod alias;
#[macro_use] mod macros;

pub use self::alias::ResultType;
