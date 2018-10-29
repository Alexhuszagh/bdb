// INTEGER

/// Null, integer trait.
pub(crate) trait Integer {
}

macro_rules! integer_impl {
    ($($t:ty)*) => ($(
        impl Integer for $t {
        }
    )*)
}

integer_impl! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }

// FLOAT

/// Null, float trait.
pub(crate) trait Float {
}

macro_rules! float_impl {
    ($($t:ty)*) => ($(
        impl Float for $t {
        }
    )*)
}

float_impl! { f32 f64 }

// ZERO

/// Get literal zero for type.
pub(crate) trait Zero: Sized {
    /// Get a literal 0 for the type.
    fn zero() -> Self;
}

macro_rules! zero_int_impl {
    ($($t:ty)*) => ($(
        impl Zero for $t {
            #[inline(always)]
            fn zero() -> Self { 0 }
        }
    )*)
}

zero_int_impl! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }

macro_rules! zero_float_impl {
    ($($t:ty)*) => ($(
        impl Zero for $t {
            #[inline(always)]
            fn zero() -> Self { 0.0 }
        }
    )*)
}

zero_float_impl! { f32 f64 }
