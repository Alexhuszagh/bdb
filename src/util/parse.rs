//! String parsing utilities.

// API

use traits::parse::Deserializable;
use super::alias::{Bytes, Result};

// NONZERO

/// Invoke method if value is non-zero, otherwise,get empty value.
macro_rules! nonzero {
    ($bytes:ident, $t:tt, $func:ident) => (match $bytes {
        b"" => Ok($t::zero()),
        _   => $func::<$t>($bytes),
    })
}

// COMMAS

/// Remove all commas from a buffer.
fn strip_commas(bytes: &[u8]) -> Bytes {
    let mut dst = Bytes::with_capacity(bytes.len());
    for c in bytes {
        match c {
            b',' => continue,
            _    => dst.push(*c),
        }
    }
    dst
}

// API

/// High-efficiency parser of a number from bytes.
#[inline(always)]
#[allow(dead_code)]
pub(crate) fn from_bytes<Number: Deserializable>(bytes: &[u8]) -> Result<Number> {
    Number::import_bytes(bytes)
}

/// Import number from bytes, returning zero if the buffer is empty.
#[inline(always)]
#[allow(dead_code)]
pub(crate) fn nonzero_from_bytes<Number: Deserializable>(bytes: &[u8]) -> Result<Number> {
    nonzero!(bytes, Number, from_bytes)
}

/// High-efficiency parser of a number from thousands-separated bytes.
#[inline(always)]
#[allow(dead_code)]
pub(crate) fn from_comma_bytes<Number: Deserializable>(bytes: &[u8]) -> Result<Number> {
    from_bytes::<Number>(&strip_commas(bytes))
}

/// Import non-zero number from thousands-separated bytes.
#[inline(always)]
#[allow(dead_code)]
pub(crate) fn nonzero_from_comma_bytes<Number: Deserializable>(bytes: &[u8]) -> Result<Number> {
    nonzero!(bytes, Number, from_comma_bytes)
}

/// High-efficiency parser of a number from string.
#[inline(always)]
#[allow(dead_code)]
pub(crate) fn from_string<Number: Deserializable>(string: &str) -> Result<Number> {
    from_bytes::<Number>(string.as_bytes())
}

/// Import number from string, returning zero if the buffer is empty.
#[inline(always)]
#[allow(dead_code)]
pub(crate) fn nonzero_from_string<Number: Deserializable>(string: &str) -> Result<Number> {
    nonzero_from_bytes::<Number>(string.as_bytes())
}

/// High-efficiency parser of a number from thousands-separated string.
#[inline(always)]
#[allow(dead_code)]
pub(crate) fn from_comma_string<Number: Deserializable>(string: &str) -> Result<Number> {
    from_comma_bytes::<Number>(string.as_bytes())
}

/// Import non-zero number from thousands-separated string.
#[inline(always)]
#[allow(dead_code)]
pub(crate) fn nonzero_from_comma_string<Number: Deserializable>(string: &str) -> Result<Number> {
    nonzero_from_comma_bytes::<Number>(string.as_bytes())
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! repeat_comma {
        ($func:ident, $value:expr, $int:expr, $float:expr) => (
            assert_eq!($func::<u16>($value).unwrap(), $int);
            assert_eq!($func::<i16>($value).unwrap(), $int);
            assert_eq!($func::<u32>($value).unwrap(), $int);
            assert_eq!($func::<i32>($value).unwrap(), $int);
            assert_eq!($func::<u64>($value).unwrap(), $int);
            assert_eq!($func::<i64>($value).unwrap(), $int);

            assert_eq!($func::<f32>($value).unwrap(), $float);
            assert_eq!($func::<f64>($value).unwrap(), $float);
        );
    }

    macro_rules! repeat {
        ($func:ident, $value:expr, $int:expr, $float:expr) => (
            assert_eq!($func::<u8>($value).unwrap(), $int);
            assert_eq!($func::<i8>($value).unwrap(), $int);
            repeat_comma!($func, $value, $int, $float);
        );
    }

    #[test]
    fn from_bytes_test() {
        repeat!(from_bytes, b"0", 0, 0.0);
        repeat!(from_bytes, b"1", 1, 1.0);
    }

    #[test]
    fn from_string_test() {
        repeat!(from_string, "0", 0, 0.0);
        repeat!(from_string, "1", 1, 1.0);
    }

    #[test]
    fn nonzero_from_bytes_test() {
        repeat!(nonzero_from_bytes, b"", 0, 0.0);
        repeat!(nonzero_from_bytes, b"1", 1, 1.0);
    }

    #[test]
    fn nonzero_from_string_test() {
        repeat!(nonzero_from_string, "", 0, 0.0);
        repeat!(nonzero_from_string, "1", 1, 1.0);
    }

    #[test]
    fn from_comma_bytes_test() {
        repeat_comma!(from_comma_bytes, b"0", 0, 0.0);
        repeat_comma!(from_comma_bytes, b"1", 1, 1.0);
        repeat_comma!(from_comma_bytes, b"1,000", 1000, 1000.0);
    }

    #[test]
    fn from_comma_string_test() {
        repeat_comma!(from_comma_string, "0", 0, 0.0);
        repeat_comma!(from_comma_string, "1", 1, 1.0);
        repeat_comma!(from_comma_string, "1,000", 1000, 1000.0);
    }

    #[test]
    fn nonzero_from_comma_bytes_test() {
        repeat_comma!(nonzero_from_comma_bytes, b"", 0, 0.0);
        repeat_comma!(nonzero_from_comma_bytes, b"1", 1, 1.0);
        repeat_comma!(nonzero_from_comma_bytes, b"1,000", 1000, 1000.0);
    }

    #[test]
    fn nonzero_from_comma_string_test() {
        repeat_comma!(nonzero_from_comma_string, "", 0, 0.0);
        repeat_comma!(nonzero_from_comma_string, "1", 1, 1.0);
        repeat_comma!(nonzero_from_comma_string, "1,000", 1000, 1000.0);
    }
}
