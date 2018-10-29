//! Formatting utilities.

use traits::fmt::Serializable;
use traits::num::{Float, Integer};
use util::search;
use super::alias::{Bytes, Result};

// NONZERO

/// Invoke method if value is non-zero, otherwise,get empty value.
macro_rules! nonzero {
    ($n:ident, $zero:expr, $func:ident) => (if *$n == $zero {
        Ok(vec![])
    } else {
        $func($n)
    })
}

// COMMAS

pub(crate) trait Comma: Serializable {
    /// Export number with thousands separators to bytes.
    fn export_comma_bytes(&self) -> Result<Bytes>;
}

/// Convert integer to comma-separated bytes.
macro_rules! comma_int_impl {
    ($($t:ty)*) => ($(
        impl Comma for $t {
            #[inline]
            fn export_comma_bytes(&self) -> Result<Bytes> {
                format_int_comma_impl(self)
            }
        }
    )*)
}

comma_int_impl! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }

/// Convert float to comma-separated bytes.
macro_rules! comma_float_impl {
    ($($t:ty)*) => ($(
        impl Comma for $t {
            #[inline]
            fn export_comma_bytes(&self) -> Result<Bytes> {
                format_float_comma_impl(self)
            }
        }
    )*)
}

comma_float_impl! { f32 f64 }

/// Internal method to add thousand separators an integer byte array.
fn add_commas_impl(bytes: &[u8], dst: &mut Bytes) -> Result<()> {
    if bytes.len() <= 3 {
        dst.extend(bytes);
        return Ok(());
    }

    // The first comma comes at the modulus  or 3.
    let index = bytes.len() % 3;
    let index = if index == 0 { 3 } else { index };
    let (start, remainder) = bytes.split_at(index);
    dst.extend(start);
    for chunk in remainder.chunks(3) {
        dst.push(b',');
        dst.extend(chunk);
    }

    Ok(())
}

/// Internal method to convert an integer to bytes.
#[allow(dead_code)]
fn format_int_comma_impl<Int: Comma + Integer>(int: &Int) -> Result<Bytes> {
    let bytes = to_bytes(int)?;
    let mut dst = Bytes::with_capacity(2 * bytes.len());
    add_commas_impl(&bytes, &mut dst)?;

    Ok(dst)
}

/// Internal method to convert an float to bytes.
#[allow(dead_code)]
fn format_float_comma_impl<Flt: Comma + Float>(float: &Flt) -> Result<Bytes> {
    let bytes = to_bytes(float)?;
    let mut dst = Bytes::with_capacity(2 * bytes.len());

    // add our commas to the integer part of the float
    let index = search::linear(&bytes, &b'.').expect("'.' in float");
    add_commas_impl(&bytes[..index], &mut dst)?;
    dst.extend(&bytes[index..]);

    Ok(dst)
}

// API

/// High-efficiency exporter of a number to bytes.
#[inline(always)]
#[allow(dead_code)]
pub(crate) fn to_bytes<Number: Serializable>(number: &Number) -> Result<Bytes> {
    number.export_bytes()
}

/// Export non-zero number to bytes, otherwise, export an empty buffer.
#[inline(always)]
#[allow(dead_code)]
pub(crate) fn nonzero_to_bytes<Number: Serializable>(number: &Number) -> Result<Bytes> {
    nonzero!(number, Number::zero(), to_bytes)
}

/// Export number to bytes with thousands separators.
#[inline(always)]
#[allow(dead_code)]
pub(crate) fn to_comma_bytes<Number: Comma>(number: &Number) -> Result<Bytes> {
    number.export_comma_bytes()
}

/// Export non-zero number to bytes with thousands separators.
#[inline(always)]
#[allow(dead_code)]
pub(crate) fn nonzero_to_comma_bytes<Number: Comma>(number: &Number) -> Result<Bytes> {
    nonzero!(number, Number::zero(), to_comma_bytes)
}

/// High-efficiency exporter of a number to string.
#[inline(always)]
#[allow(dead_code)]
pub(crate) fn to_string<Number: Serializable>(number: &Number) -> Result<String> {
    unsafe {
        Ok(String::from_utf8_unchecked(to_bytes(number)?))
    }
}

/// Export non-zero number to string, otherwise, export an empty string.
#[inline(always)]
#[allow(dead_code)]
pub(crate) fn nonzero_to_string<Number: Serializable>(number: &Number) -> Result<String> {
    unsafe {
        Ok(String::from_utf8_unchecked(nonzero_to_bytes(number)?))
    }
}

/// Export number to string with thousands separators.
#[inline(always)]
#[allow(dead_code)]
pub(crate) fn to_comma_string<Number: Comma>(number: &Number) -> Result<String> {
    unsafe {
        Ok(String::from_utf8_unchecked(to_comma_bytes(number)?))
    }
}

/// Export non-zero number to string with thousands separators.
#[inline(always)]
#[allow(dead_code)]
pub(crate) fn nonzero_to_comma_string<Number: Comma>(number: &Number) -> Result<String> {
    unsafe {
        Ok(String::from_utf8_unchecked(nonzero_to_comma_bytes(number)?))
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! repeat_comma {
        ($func:ident, $value:expr, $int:expr, $float:expr) => (
            let memo = $int;
            assert_eq!($func(&($value as u16)).unwrap(), memo);
            assert_eq!($func(&($value as i16)).unwrap(), memo);
            assert_eq!($func(&($value as u32)).unwrap(), memo);
            assert_eq!($func(&($value as i32)).unwrap(), memo);
            assert_eq!($func(&($value as u64)).unwrap(), memo);
            assert_eq!($func(&($value as i64)).unwrap(), memo);

            let memo = $float;
            assert_eq!($func(&($value as f32)).unwrap(), memo);
            assert_eq!($func(&($value as f64)).unwrap(), memo);
        );
    }

    macro_rules! repeat {
        ($func:ident, $value:expr, $int:expr, $float:expr) => (
            let memo = $int;
            assert_eq!($func(&($value as u8)).unwrap(), memo);
            assert_eq!($func(&($value as i8)).unwrap(), memo);
            repeat_comma!($func, $value, memo, $float)
        );
    }

    #[test]
    fn to_bytes_test() {
        repeat!(to_bytes, 0, b"0".to_vec(), b"0.0".to_vec());
        repeat!(to_bytes, 1, b"1".to_vec(), b"1.0".to_vec());
    }

    #[test]
    fn to_string_test() {
        repeat!(to_string, 0, "0", "0.0");
        repeat!(to_string, 1, "1", "1.0");
    }

    #[test]
    fn nonzero_to_bytes_test() {
        repeat!(nonzero_to_bytes, 0, b"".to_vec(), b"".to_vec());
        repeat!(nonzero_to_bytes, 1, b"1".to_vec(), b"1.0".to_vec());
    }

    #[test]
    fn nonzero_to_string_test() {
        repeat!(nonzero_to_string, 0, "", "");
        repeat!(nonzero_to_string, 1, "1", "1.0");
    }

    #[test]
    fn to_comma_bytes_test() {
        // Don't use u8/i8 here.
        repeat_comma!(to_comma_bytes, 0, b"0".to_vec(), b"0.0".to_vec());
        repeat_comma!(to_comma_bytes, 1, b"1".to_vec(), b"1.0".to_vec());
        repeat_comma!(to_comma_bytes, 1000, b"1,000".to_vec(), b"1,000.0".to_vec());
    }

    #[test]
    fn to_comma_string_test() {
        repeat_comma!(to_comma_string, 0, "0", "0.0");
        repeat_comma!(to_comma_string, 1, "1", "1.0");
        repeat_comma!(to_comma_string, 1000, "1,000", "1,000.0");
    }

    #[test]
    fn nonzero_to_comma_bytes_test() {
        // Don't use u8/i8 here.
        repeat_comma!(nonzero_to_comma_bytes, 0, b"".to_vec(), b"".to_vec());
        repeat_comma!(nonzero_to_comma_bytes, 1, b"1".to_vec(), b"1.0".to_vec());
        repeat_comma!(nonzero_to_comma_bytes, 1000, b"1,000".to_vec(), b"1,000.0".to_vec());
    }

    #[test]
    fn nonzero_to_comma_string_test() {
        repeat_comma!(nonzero_to_comma_string, 0, "", "");
        repeat_comma!(nonzero_to_comma_string, 1, "1", "1.0");
        repeat_comma!(nonzero_to_comma_string, 1000, "1,000", "1,000.0");
    }
}
