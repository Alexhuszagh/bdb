/**
 *  :copyright: (c) 2018 Alex Huszagh.
 *  :license: MIT, see LICENSE.md for more details.
 */

// TODO(ahuszagh)
//  Add const_assert liberally.
//#[macro_use] extern crate static_assertions;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
#[cfg_attr(test, macro_use)] extern crate assert_approx_eq;

extern crate hyper;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate url;

#[macro_use] pub mod macros;

// Traits
pub mod complete;
pub mod fasta;
pub mod text;
pub mod valid;
pub mod xml;

// General
pub mod proteins;

// Models & Services
pub mod uniprot;
