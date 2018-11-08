use lexical;

use util::alias::{Bytes, Result};
use super::num::Zero;

// SERIALIZABLE

/// Efficient conversion of numbers to bytes or string.
pub(crate) trait Serializable: Zero + PartialEq {
    /// Efficient number to bytes conversion.
    fn export_bytes(&self) -> Result<Bytes>;
}

/// Implementation of serializable integers.
macro_rules! serializable_impl {
    ($($t:ty)*) => ($(
        impl Serializable for $t {
            #[inline]
            fn export_bytes(&self) -> Result<Bytes> {
                Ok(lexical::to_string(*self).into_bytes())
            }
        }
    )*)
}

serializable_impl! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize f32 f64 }

