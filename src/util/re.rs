//! Shared regular expression utilities.

/// Construct static-like regex lazily at runtime.
macro_rules! lazy_regex {
    ($re:tt, $str:expr) => (lazy_static! {
        static ref REGEX: $re = $re::new($str).unwrap();
    })
}

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
