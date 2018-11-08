use lexical;

use util::alias::Result;
use util::error::ErrorKind;
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
                match lexical::try_parse(bytes) {
                    Ok(v)  => Ok(v),
                    Err(_) => Err(From::from(ErrorKind::InvalidInput))
                }
            }
        }
    )*)
}

deserializable_impl! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize f32 f64 }
