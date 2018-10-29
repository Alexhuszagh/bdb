//! Shared macros.

// RECURSIVE APPLICATION

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
/// assert_eq!(writer.into_inner(), b"12345".to_vec());
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

// ERROR

/// Macro to convert a `None` `Option` to an error.
#[doc(hidden)]
#[macro_export]
macro_rules! none_to_error {
    ($e:expr, $t:ident) => (
        match $e {
            None    => return Err(From::from(ErrorKind::$t)),
            Some(v) => v,
        };
    )
}

/// Macro to convert `false` to an error.
#[doc(hidden)]
#[macro_export]
macro_rules! bool_to_error {
    ($e:expr, $t:ident) => (
        if !$e {
            return Err(From::from(ErrorKind::$t));
        };
    )
}


/// Map an iterator to take items by value.
#[cfg(test)]
macro_rules! iterator_by_value {
    ($x:expr) => ($x.map(|x| { Ok(x.clone()) }))
}
