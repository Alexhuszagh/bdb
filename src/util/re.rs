//! Shared regular expression utilities.

use regex::Captures;

// MACROS

/// Construct static-like regex lazily at runtime.
macro_rules! lazy_regex {
    ($re:tt, $str:expr) => (lazy_static! {
        static ref REGEX: $re = $re::new($str).unwrap();
    })
}

// TRAITS

/// Provides a regular expression to extract data from input.
///
/// This regular expression may be significantly more expensive
/// than `ValidationRegex`.
pub trait ExtractionRegex<Re> {
    /// Static regex to extract data from input.
    fn extract() -> &'static Re;
}

/// Provides a regular expression to validate input.
pub trait ValidationRegex<Re> {
    /// Static regex to validate data from input.
    fn validate() -> &'static Re;
}

// CAPTURES

/// Convert capture group to `&str`.
#[inline(always)]
pub fn capture_as_str<'t>(captures: &'t Captures, index: usize) -> &'t str {
    captures.get(index).unwrap().as_str()
}

/// Convert optional capture group to `&str`.
#[inline(always)]
pub fn optional_capture_as_str<'t>(captures: &'t Captures, index: usize) -> &'t str {
    match captures.get(index) {
        None    => "",
        Some(v) => v.as_str(),
    }
}

/// Convert capture group to `String`.
#[inline(always)]
pub fn capture_as_string(captures: &Captures, index: usize) -> String {
    String::from(capture_as_str(captures, index))
}

/// Convert optional capture group to `String`.
#[inline(always)]
pub fn optional_capture_as_string(captures: &Captures, index: usize) -> String {
    String::from(optional_capture_as_str(captures, index))
}
