use crate::{
    InvalidNumber, Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN,
    NonZeroNonNaNFinite, Positive, PositiveFinite, StrictlyNegative, StrictlyNegativeFinite,
    StrictlyPositive, StrictlyPositiveFinite,
};

use core::num::{NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8};
use core::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

macro_rules! impl_from_int {
    ($type:ident,$int:ident) => {
        impl From<$int> for $type<f32> {
            #[inline]
            #[must_use]
            fn from(value: $int) -> Self {
                unsafe { Self::new_unchecked(value.get() as f32) }
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
                Self::new(value.get() as f32)
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

// from non-zero signed integers

impl_from_ints!(NonNaN, NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64);
impl_from_ints!(NonZeroNonNaN, NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64);
impl_from_ints!(NonNaNFinite, NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64);
impl_try_from_ints!(
    NonZeroNonNaNFinite,
    NonZeroI8,
    NonZeroI16,
    NonZeroI32,
    NonZeroI64
);
impl_try_from_ints!(Positive, NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64);
impl_try_from_ints!(Negative, NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64);
impl_try_from_ints!(
    PositiveFinite,
    NonZeroI8,
    NonZeroI16,
    NonZeroI32,
    NonZeroI64
);
impl_try_from_ints!(
    NegativeFinite,
    NonZeroI8,
    NonZeroI16,
    NonZeroI32,
    NonZeroI64
);
impl_try_from_ints!(
    StrictlyPositive,
    NonZeroI8,
    NonZeroI16,
    NonZeroI32,
    NonZeroI64
);
impl_try_from_ints!(
    StrictlyNegative,
    NonZeroI8,
    NonZeroI16,
    NonZeroI32,
    NonZeroI64
);
impl_try_from_ints!(
    StrictlyPositiveFinite,
    NonZeroI8,
    NonZeroI16,
    NonZeroI32,
    NonZeroI64
);
impl_try_from_ints!(
    StrictlyNegativeFinite,
    NonZeroI8,
    NonZeroI16,
    NonZeroI32,
    NonZeroI64
);

// from non-zero unsigned integers

impl_from_ints!(NonNaN, NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64);
impl_from_ints!(NonZeroNonNaN, NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64);
impl_from_ints!(NonNaNFinite, NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64);
impl_from_ints!(
    NonZeroNonNaNFinite,
    NonZeroU8,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64
);
impl_from_ints!(Positive, NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64);
impl_try_from_ints!(Negative, NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64);
impl_from_ints!(
    PositiveFinite,
    NonZeroU8,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64
);
impl_try_from_ints!(
    NegativeFinite,
    NonZeroU8,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64
);
impl_from_ints!(
    StrictlyPositive,
    NonZeroU8,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64
);
impl_try_from_ints!(
    StrictlyNegative,
    NonZeroU8,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64
);
impl_from_ints!(
    StrictlyPositiveFinite,
    NonZeroU8,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64
);
impl_try_from_ints!(
    StrictlyNegativeFinite,
    NonZeroU8,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64
);
