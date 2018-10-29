#[cfg(test)]
#[macro_use]
extern crate assert_approx_eq;

#[macro_use]
extern crate cfg_if;

#[macro_use]
extern crate lazy_static;

extern crate dtoa;
extern crate itoa;
extern crate ref_slice;
extern crate regex;

#[cfg(feature = "csv")]
extern crate csv;

#[cfg(feature = "xml")]
extern crate quick_xml;

#[cfg(feature = "http")]
extern crate reqwest;

#[cfg(feature = "http")]
extern crate url;

#[cfg(test)]
extern crate bencher;

// Macros and utilities (required by other modules).
#[macro_use]
pub(crate) mod util;

// Testing modules
#[cfg(test)]
#[macro_use]
pub(crate) mod test;

// Public modules
pub mod bio;
pub mod db;
pub mod io;
pub mod traits;

// Re-export utility traits that should be shared.
pub use util::{Error, ErrorKind, Result};
