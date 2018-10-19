#[cfg_attr(test, macro_use)] extern crate assert_approx_eq;
extern crate csv;
extern crate digit_group;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate reqwest;
// TODO(ahuszagh)
//  Add const_assert liberally.
//#[macro_use] extern crate static_assertions;
extern crate url;

#[macro_use] pub mod util;
pub mod bio;
pub mod db;
pub mod io;
pub mod traits;
