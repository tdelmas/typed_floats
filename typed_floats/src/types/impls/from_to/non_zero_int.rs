use crate::{
    InvalidNumber, Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN,
    NonZeroNonNaNFinite, Positive, PositiveFinite, StrictlyNegative, StrictlyNegativeFinite,
    StrictlyPositive, StrictlyPositiveFinite,
};

use core::num::{NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8};
use core::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

macro_rules! impl_from_int {
    ($type:ident,$int:ident) => {
        impl From<$int> for $type<f64> {
            #[inline]
            #[must_use]
            fn from(value: $int) -> Self {
                unsafe { Self::new_unchecked(value.get() as f64) }
            }
        }
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
        impl TryFrom<$int> for $type<f64> {
            type Error = InvalidNumber;

            #[inline]
            fn try_from(value: $int) -> Result<Self, Self::Error> {
                Self::new(value.get() as f64)
            }
        }
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

macro_rules! impl_test {
    ($type:ident, $int:ident, $uint:ident, $non_zero_int:ident, $non_zero_uint:ident) => {
        let ints = [
            $int::MIN,
            $int::MIN + 1,
            $int::MIN / 2,
            -2,
            -1,
            1,
            2,
            $int::MAX / 2,
            $int::MAX - 1,
            $int::MAX,
        ];

        for &i in &ints {
            // Will panic if an invalid value is created.
            let _ = crate::$type::<f64>::try_from($non_zero_int::new(i).unwrap());
            let _ = crate::$type::<f32>::try_from($non_zero_int::new(i).unwrap());
        }

        let uints = [1, 2, $uint::MAX / 2, $uint::MAX - 1, $uint::MAX];

        for &i in &uints {
            // Will panic if an invalid value is created.
            let _ = crate::$type::<f64>::try_from($non_zero_uint::new(i).unwrap());
            let _ = crate::$type::<f32>::try_from($non_zero_uint::new(i).unwrap());
        }
    };
}

macro_rules! impl_tests {
    ($type:ident) => {
        use core::num::{NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8};
        use core::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

        impl_test!($type, i8, u8, NonZeroI8, NonZeroU8);
        impl_test!($type, i16, u16, NonZeroI16, NonZeroU16);
        impl_test!($type, i32, u32, NonZeroI32, NonZeroU32);
        impl_test!($type, i64, u64, NonZeroI64, NonZeroU64);
    };
}

mod test {
    #[test]
    fn non_nan() {
        impl_tests!(NonNaN);
    }

    #[test]
    fn non_zero_non_nan() {
        impl_tests!(NonZeroNonNaN);
    }

    #[test]
    fn non_nan_finite() {
        impl_tests!(NonNaNFinite);
    }

    #[test]
    fn non_zero_non_nan_finite() {
        impl_tests!(NonZeroNonNaNFinite);
    }

    #[test]
    fn positive() {
        impl_tests!(Positive);
    }

    #[test]
    fn negative() {
        impl_tests!(Negative);
    }

    #[test]
    fn positive_finite() {
        impl_tests!(PositiveFinite);
    }

    #[test]
    fn negative_finite() {
        impl_tests!(NegativeFinite);
    }

    #[test]
    fn strictly_positive() {
        impl_tests!(StrictlyPositive);
    }

    #[test]
    fn strictly_negative() {
        impl_tests!(StrictlyNegative);
    }

    #[test]
    fn strictly_positive_finite() {
        impl_tests!(StrictlyPositiveFinite);
    }

    #[test]
    fn strictly_negative_finite() {
        impl_tests!(StrictlyNegativeFinite);
    }
}

// https://doc.rust-lang.org/1.49.0/reference/expressions/operator-expr.html#type-cast-expressions

// with the current set of numeric types, overflow can only happen on u128 as f64

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
