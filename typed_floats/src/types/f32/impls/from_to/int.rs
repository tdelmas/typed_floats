use crate::{
    InvalidNumber, Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN,
    NonZeroNonNaNFinite, Positive, PositiveFinite, StrictlyNegative, StrictlyNegativeFinite,
    StrictlyPositive, StrictlyPositiveFinite,
};

macro_rules! impl_from_int {
    ($type:ident,$int:ident) => {
        impl From<$int> for $type<f32> {
            #[inline]
            #[must_use]
            fn from(value: $int) -> Self {
                unsafe { Self::new_unchecked(value as f32) }
            }
        }
    };
}

macro_rules! impl_try_from_int {
    ($type:ident,$int:ident) => {
        impl TryFrom<$int> for $type<f32> {
            type Error = InvalidNumber;

            #[inline]
            fn try_from(value: $int) -> Result<Self, Self::Error> {
                Self::new(value as f32)
            }
        }
    };
}

macro_rules! impl_from_ints {
    ($type:ident, $($int:ident),*) => {
        $(
            impl_from_int!($type, $int);
        )*
    };
}

macro_rules! impl_try_from_ints {
    ($type:ident, $($int:ident),*) => {
        $(
            impl_try_from_int!($type, $int);
        )*
    };
}

// https://doc.rust-lang.org/1.49.0/reference/expressions/operator-expr.html#type-cast-expressions

// with the current set of numeric types, overflow can only happen on u128 as f32

// from signed integers

impl_from_ints!(NonNaN, i8, i16, i32, i64);
impl_try_from_ints!(NonZeroNonNaN, i8, i16, i32, i64);
impl_from_ints!(NonNaNFinite, i8, i16, i32, i64);
impl_try_from_ints!(NonZeroNonNaNFinite, i8, i16, i32, i64);
impl_try_from_ints!(Positive, i8, i16, i32, i64);
impl_try_from_ints!(Negative, i8, i16, i32, i64);
impl_try_from_ints!(PositiveFinite, i8, i16, i32, i64);
impl_try_from_ints!(NegativeFinite, i8, i16, i32, i64);
impl_try_from_ints!(StrictlyPositive, i8, i16, i32, i64);
impl_try_from_ints!(StrictlyNegative, i8, i16, i32, i64);
impl_try_from_ints!(StrictlyPositiveFinite, i8, i16, i32, i64);
impl_try_from_ints!(StrictlyNegativeFinite, i8, i16, i32, i64);

// from unsigned integers

impl_from_ints!(NonNaN, u8, u16, u32, u64);
impl_from_ints!(NonZeroNonNaN, u8, u16, u32, u64);
impl_from_ints!(NonNaNFinite, u8, u16, u32, u64);
impl_from_ints!(NonZeroNonNaNFinite, u8, u16, u32, u64);
impl_from_ints!(Positive, u8, u16, u32, u64);
impl_try_from_ints!(Negative, u8, u16, u32, u64);
impl_from_ints!(PositiveFinite, u8, u16, u32, u64);
impl_try_from_ints!(NegativeFinite, u8, u16, u32, u64);
impl_from_ints!(StrictlyPositive, u8, u16, u32, u64);
impl_try_from_ints!(StrictlyNegative, u8, u16, u32, u64);
impl_from_ints!(StrictlyPositiveFinite, u8, u16, u32, u64);
impl_try_from_ints!(StrictlyNegativeFinite, u8, u16, u32, u64);
