//! Shared macros.

// GENERAL

/// Conditionally execute code based on a binary choice.
///
/// This is an internal helper function to simplify the logic for
/// numerous other macros.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate bdb;
/// # pub fn main() {
/// assert_eq!(binary_choice!(true, 0, 2), 0);
/// assert_eq!(binary_choice!(false, 0, 2), 2);
/// # }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! binary_choice {
    ($condition:expr, $yes:expr, $no:expr) => (
        if $condition {
            $yes
        } else {
            $no
        }
    );
}

// TO/FROM STRING

/// Macro to serialize non-zero numbers to string.
///
/// Exports a number to string only if the number is non-zero.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate bdb;
/// # use bdb::traits::Ntoa;
/// # pub fn main() {
/// assert_eq!(nonzero_to_string!(0 as i32), "");
/// assert_eq!(nonzero_to_string!(1 as i32), "1");
/// # }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! nonzero_to_string {
    ($e:expr) => ({
        // Prevent side effects from expression evaluation.
        let memo = $e;
        binary_choice!(memo == 0, String::new(), memo.ntoa().unwrap())
    });
}


/// Macro to serialize non-zero floating numbers to string.
///
/// Exports a number to string only if the float is non-zero.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate bdb;
/// # use bdb::traits::Ntoa;
/// # pub fn main() {
/// assert_eq!(nonzero_float_to_string!(0.0 as f32), "");
/// assert_eq!(nonzero_float_to_string!(1.2 as f32), "1.2");
/// # }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! nonzero_float_to_string {
    ($e:expr) => ({
        // Prevent side effects from expression evaluation.
        let memo = $e;
        binary_choice!(memo == 0.0, String::new(), memo.ntoa().unwrap())
    });
}


/// Macro to parse non-zero numbers from string.
///
/// Parses an empty string as zero, otherwise, parses the number.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate bdb;
/// # pub fn main() {
/// assert_eq!(nonzero_from_string!("").unwrap(), 0);
/// assert_eq!(nonzero_from_string!("1").unwrap(), 1);
/// assert_eq!(nonzero_from_string!("1", u8).unwrap(), 1);
/// # }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! nonzero_from_string {
    ($e:expr) => ({
        // Prevent side effects from expression evaluation.
        let memo = $e;
        binary_choice!(memo == "", Ok(0), memo.parse())
    });
    ($e:expr, $t:ty) => ({
        // Prevent side effects from expression evaluation.
        let memo = $e;
        binary_choice!(memo == "", Ok(0), memo.parse::<$t>())
    });
}


/// Macro to parse non-zero floating numbers from string.
///
/// Parses an empty string as zero, otherwise, parses the float.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate bdb;
/// # pub fn main() {
/// assert_eq!(nonzero_float_from_string!("").unwrap(), 0.0);
/// assert_eq!(nonzero_float_from_string!("1.2").unwrap(), 1.2);
/// assert_eq!(nonzero_float_from_string!("1.2", f32).unwrap(), 1.2);
/// # }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! nonzero_float_from_string {
    ($e:expr) => ({
        // Prevent side effects from expression evaluation.
        let memo = $e;
        binary_choice!(memo == "", Ok(0.), memo.parse())
    });
    ($e:expr, $t:ty) => ({
        // Prevent side effects from expression evaluation.
        let memo = $e;
        binary_choice!(memo == "", Ok(0.), memo.parse::<$t>())
    });
}


/// Macro to convert a number to a comma-separated string.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate bdb;
/// # extern crate digit_group;
/// # use digit_group::FormatGroup;
/// # pub fn main() {
/// assert_eq!(to_commas!(0), "0");
/// assert_eq!(to_commas!(1000), "1,000");
/// # }
#[doc(hidden)]
#[macro_export]
macro_rules! to_commas {
    ($e:expr) => ($e.format_commas())
}


/// Macro to strip a string of commas.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate bdb;
/// # pub fn main() {
/// assert_eq!(strip_commas!("500"), "500");
/// assert_eq!(strip_commas!("1,000"), "1000");
/// # }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! strip_commas {
    ($e:expr) => ({
        let memo = $e;
        let mut string = String::with_capacity(memo.len());
        for c in memo.chars() {
            match c {
                ',' => continue,
                _   => string.push(c),
            }
        }
        string
    })
}


/// Macro to convert a comma-separated string to a number.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate bdb;
/// # pub fn main() {
/// assert_eq!(from_commas!("500", u32).unwrap(), 500);
/// assert_eq!(from_commas!("1,000", u32).unwrap(), 1000);
/// # }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! from_commas {
    ($e:expr, $t:ty) => ({
        strip_commas!($e).parse::<$t>()
    });
}


/// Macro to serialize non-zero numbers to string.
///
/// Exports a number to string only if the number is non-zero.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate bdb;
/// # extern crate digit_group;
/// # use digit_group::FormatGroup;
/// # pub fn main() {
/// assert_eq!(nonzero_to_commas!(0), "");
/// assert_eq!(nonzero_to_commas!(1), "1");
/// assert_eq!(nonzero_to_commas!(1000), "1,000");
/// # }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! nonzero_to_commas {
    ($e:expr) => ({
        let memo = $e;
        binary_choice!(memo == 0, String::new(), to_commas!(memo))
    });
}


/// Macro to parse non-zero numbers from string.
///
/// Parses an empty string as zero, otherwise, parses the number.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate bdb;
/// # pub fn main() {
/// assert_eq!(nonzero_from_commas!("", u32).unwrap(), 0);
/// assert_eq!(nonzero_from_commas!("1", u32).unwrap(), 1);
/// assert_eq!(nonzero_from_commas!("1,000", u32).unwrap(), 1000);
/// # }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! nonzero_from_commas {
    ($e:expr, $t:ty) => ({
        let memo = $e;
        binary_choice!(memo == "", Ok(0), from_commas!(memo, $t))
    });
}

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
