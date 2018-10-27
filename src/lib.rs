#[cfg_attr(test, macro_use)] extern crate assert_approx_eq;
extern crate bencher;
#[macro_use] extern crate cfg_if;
extern crate digit_group;
#[macro_use] extern crate lazy_static;
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
