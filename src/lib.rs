#[cfg_attr(test, macro_use)] extern crate assert_approx_eq;
extern crate csv;
extern crate digit_group;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate reqwest;
extern crate url;

// Macros and utilities (required by other modules).
#[macro_use] pub mod util;

// Public modules
pub mod bio;
pub mod db;
pub mod io;
pub mod traits;

// Testing modules
#[cfg(test)] pub mod test;
