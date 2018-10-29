use dtoa;
use itoa;

use util::alias::{Bytes, Result};
use super::num::Zero;

// SERIALIZABLE

/// Efficient conversion of numbers to bytes or string.
pub(crate) trait Serializable: Zero + PartialEq {
    /// Efficient number to bytes conversion.
    fn export_bytes(&self) -> Result<Bytes>;
}

/// Convert integer to bytes.
macro_rules! int_to_bytes {
    // Case with explicit capacity.
    ($i:expr, $capacity:expr) => ({
        let mut v: Bytes = Bytes::with_capacity($capacity);
        itoa::write(&mut v, $i)?;
        v.shrink_to_fit();
        Ok(v)
    });
    // Non-explicit capacity, assume worse-case (20 digits for u64).
    ($i:expr) => ({
        int_to_bytes!($i, 20)
    });
}

/// Implementation of serializable integers.
macro_rules! serializable_int_impl {
    ($t:ty, $capacity:expr) => (
        impl Serializable for $t {
            #[inline]
            fn export_bytes(&self) -> Result<Bytes> {
                int_to_bytes!(*self, $capacity)
            }
        }
    )
}

serializable_int_impl!(u8, 3);
serializable_int_impl!(i8, 3);
serializable_int_impl!(u16, 5);
serializable_int_impl!(i16, 5);
serializable_int_impl!(u32, 10);
serializable_int_impl!(i32, 10);
serializable_int_impl!(u64, 20);
serializable_int_impl!(i64, 20);
serializable_int_impl!(usize, 20);
serializable_int_impl!(isize, 20);

/// Convert float to bytes.
macro_rules! float_to_bytes {
    // Case with explicit capacity.
    ($f:expr, $capacity:expr) => ({
        let mut v: Vec<u8> = Vec::with_capacity($capacity);
        dtoa::write(&mut v, $f)?;
        v.shrink_to_fit();
        Ok(v)
    });
    // Non-explicit capacity, assume worse-case (300 digits for f64).
    ($f:expr) => ({
        float_to_string!($f, 300)
    });
}

/// Implementation of serializable floats.
macro_rules! serializable_float_impl {
    ($t:ty, $capacity:expr) => (
        impl Serializable for $t {
            #[inline]
            fn export_bytes(&self) -> Result<Bytes> {
                float_to_bytes!(*self, $capacity)
            }
        }
    )
}

serializable_float_impl!(f32, 25);
serializable_float_impl!(f64, 50);

