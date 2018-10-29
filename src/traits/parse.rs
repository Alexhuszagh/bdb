use std::str as stdstr;

use util::alias::Result;
use super::num::Zero;

// DESERIALIZABLE

/// Efficient conversion of bytes or string to numbers.
pub(crate) trait Deserializable: Zero {
    /// Efficient bytes to number conversion.
    #[inline]
    fn import_bytes(bytes: &[u8]) -> Result<Self>;
}


/// Implementation of deserializable integers.
macro_rules! deserializable_impl {
    ($($t:ty)*) => ($(
        impl Deserializable for $t {
            #[inline]
            fn import_bytes(bytes: &[u8]) -> Result<$t> {
                // TODO(ahuszagh)   Make more efficient, using custom routines.
                // Remove parse and from_utf8
                Ok(stdstr::from_utf8(bytes)?.parse::<$t>()?)
            }
        }
    )*)
}

deserializable_impl! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize f32 f64 }
