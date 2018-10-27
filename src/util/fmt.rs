//! Formatting utilities.

use dtoa;
use itoa;

use super::alias::ResultType;

// MACROS

/// Convert integer to string.
macro_rules! int_to_string {
    ($f:expr) => ({
        let mut v: Vec<u8> = Vec::with_capacity(20);
        itoa::write(&mut v, $f)?;
        v.shrink_to_fit();
        Ok(unsafe { String::from_utf8_unchecked(v) } )
    })
}

/// Convert float to string.
macro_rules! float_to_string {
    ($f:expr) => ({
        let mut v: Vec<u8> = Vec::with_capacity(300);
        dtoa::write(&mut v, $f)?;
        v.shrink_to_fit();
        Ok(unsafe { String::from_utf8_unchecked(v) } )
    })
}

// TRAITS

/// Convert number to string efficiently.
pub trait Ntoa {
    /// Efficient number to string conversion.
    fn ntoa(&self) -> ResultType<String>;
}

impl Ntoa for u8 {
    fn ntoa(&self) -> ResultType<String> {
        int_to_string!(*self)
    }
}

impl Ntoa for i8 {
    fn ntoa(&self) -> ResultType<String> {
        int_to_string!(*self)
    }
}

impl Ntoa for u16 {
    fn ntoa(&self) -> ResultType<String> {
        int_to_string!(*self)
    }
}

impl Ntoa for i16 {
    fn ntoa(&self) -> ResultType<String> {
        int_to_string!(*self)
    }
}

impl Ntoa for u32 {
    fn ntoa(&self) -> ResultType<String> {
        int_to_string!(*self)
    }
}

impl Ntoa for i32 {
    fn ntoa(&self) -> ResultType<String> {
        int_to_string!(*self)
    }
}

impl Ntoa for u64 {
    fn ntoa(&self) -> ResultType<String> {
        int_to_string!(*self)
    }
}

impl Ntoa for i64 {
    fn ntoa(&self) -> ResultType<String> {
        int_to_string!(*self)
    }
}

impl Ntoa for f32 {
    fn ntoa(&self) -> ResultType<String> {
        float_to_string!(*self)
    }
}

impl Ntoa for f64 {
    fn ntoa(&self) -> ResultType<String> {
        float_to_string!(*self)
    }
}
