//! Shared macros.

/// Macro to serialize non-zero numbers to string.
///
/// Exports a number to string only if the number is non-zero.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate bdb;
/// # pub fn main() {
/// assert_eq!(nonzero_to_string!(0), "");
/// assert_eq!(nonzero_to_string!(1), "1");
/// # }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! nonzero_to_string {
    ($e:expr) => (
        match $e {
            0 => String::new(),
            _ => $e.to_string(),
        }
    );
}


/// Macro to call `s.push_str(x)` for all x.
///
/// Converts each successive expression in `x` to converted to
/// `s.push_str(x)`.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate bdb;
/// # pub fn main() {
/// let mut s = String::new();
/// push_strs!(s, "1", "2", "345");
/// assert_eq!(s, "12345");
/// # }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! push_strs {
    // Base case, call `push_str`
    ($s:ident, $x:expr) => ($s.push_str($x););
    // `$x` followed by at least one `$y,`
    ($s:ident, $x:expr, $($y:expr),+) => ({
        $s.push_str($x);
        push_strs!($s, $($y),+)
    });
}


/// Macro to call `s.write_all(x)` for all x.
///
/// Converts each successive expression in `x` to converted to
/// `s.write_all(x)`.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate bdb;
/// use std::io::{Cursor, Write};
/// # pub fn main() {
/// let mut writer = Cursor::new(Vec::new());
/// write_alls!(writer, b"1", b"2", b"345");
/// assert_eq!(String::from_utf8(writer.into_inner()).unwrap(), "12345");
/// # }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! write_alls {
    // Base case, call `write_all`
    ($s:ident, $x:expr) => ($s.write_all($x));
    // `$x` followed by at least one `$y,`
    ($s:ident, $x:expr, $($y:expr),+) => ({
        match $s.write_all($x) {
            Err(e) => Err(e),
            _      => write_alls!($s, $($y),+)
        }
    });
}
