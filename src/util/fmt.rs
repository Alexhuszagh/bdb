//! Formatting utilities.

use dtoa;
use itoa;

use super::alias::ResultType;

// MACROS

/// Convert integer to string.
macro_rules! int_to_string {
    // Case with explicit capacity.
    ($i:expr, $capacity:expr) => ({
        let mut v: Vec<u8> = Vec::with_capacity($capacity);
        itoa::write(&mut v, $i)?;
        v.shrink_to_fit();
        Ok(unsafe { String::from_utf8_unchecked(v) } )
    });
    // Non-explicit capacity, assume worse-case (20 digits for u64).
    ($i:expr) => ({
        int_to_string!($i, 20)
    });
}

/// Convert float to string.
macro_rules! float_to_string {
    // Case with explicit capacity.
    ($f:expr, $capacity:expr) => ({
        let mut v: Vec<u8> = Vec::with_capacity($capacity);
        dtoa::write(&mut v, $f)?;
        v.shrink_to_fit();
        Ok(unsafe { String::from_utf8_unchecked(v) } )
    });
    // Non-explicit capacity, assume worse-case (300 digits for f64).
    ($f:expr) => ({
        float_to_string!($f, 300)
    });
}

// TRAITS

/// Convert number to string efficiently.
pub trait Ntoa {
    /// Efficient number to string conversion.
    fn ntoa(&self) -> ResultType<String>;

    /// Ntoa with a custom capacity.
    fn ntoa_with_capacity(&self, capacity: usize) -> ResultType<String>;
}

impl Ntoa for u8 {
    #[inline]
    fn ntoa(&self) -> ResultType<String> {
        int_to_string!(*self, 3)
    }

    #[inline]
    fn ntoa_with_capacity(&self, capacity: usize) -> ResultType<String> {
        int_to_string!(*self, capacity)
    }
}

impl Ntoa for i8 {
    #[inline]
    fn ntoa(&self) -> ResultType<String> {
        int_to_string!(*self, 3)
    }

    #[inline]
    fn ntoa_with_capacity(&self, capacity: usize) -> ResultType<String> {
        int_to_string!(*self, capacity)
    }
}

impl Ntoa for u16 {
    #[inline]
    fn ntoa(&self) -> ResultType<String> {
        int_to_string!(*self, 5)
    }

    #[inline]
    fn ntoa_with_capacity(&self, capacity: usize) -> ResultType<String> {
        int_to_string!(*self, capacity)
    }
}

impl Ntoa for i16 {
    #[inline]
    fn ntoa(&self) -> ResultType<String> {
        int_to_string!(*self, 5)
    }

    #[inline]
    fn ntoa_with_capacity(&self, capacity: usize) -> ResultType<String> {
        int_to_string!(*self, capacity)
    }
}

impl Ntoa for u32 {
    #[inline]
    fn ntoa(&self) -> ResultType<String> {
        int_to_string!(*self, 10)
    }

    #[inline]
    fn ntoa_with_capacity(&self, capacity: usize) -> ResultType<String> {
        int_to_string!(*self, capacity)
    }
}

impl Ntoa for i32 {
    #[inline]
    fn ntoa(&self) -> ResultType<String> {
        int_to_string!(*self, 10)
    }

    #[inline]
    fn ntoa_with_capacity(&self, capacity: usize) -> ResultType<String> {
        int_to_string!(*self, capacity)
    }
}

impl Ntoa for u64 {
    #[inline]
    fn ntoa(&self) -> ResultType<String> {
        int_to_string!(*self, 20)
    }

    #[inline]
    fn ntoa_with_capacity(&self, capacity: usize) -> ResultType<String> {
        int_to_string!(*self, capacity)
    }
}

impl Ntoa for i64 {
    #[inline]
    fn ntoa(&self) -> ResultType<String> {
        int_to_string!(*self, 20)
    }

    #[inline]
    fn ntoa_with_capacity(&self, capacity: usize) -> ResultType<String> {
        int_to_string!(*self, capacity)
    }
}

impl Ntoa for f32 {
    #[inline]
    fn ntoa(&self) -> ResultType<String> {
        float_to_string!(*self)
    }

    #[inline]
    fn ntoa_with_capacity(&self, capacity: usize) -> ResultType<String> {
        float_to_string!(*self, capacity)
    }
}

impl Ntoa for f64 {
    #[inline]
    fn ntoa(&self) -> ResultType<String> {
        float_to_string!(*self)
    }

    #[inline]
    fn ntoa_with_capacity(&self, capacity: usize) -> ResultType<String> {
        float_to_string!(*self, capacity)
    }
}
