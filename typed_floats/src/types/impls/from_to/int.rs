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
        impl From<$int> for $type<f64> {
            #[inline]
            #[must_use]
            fn from(value: $int) -> Self {
                unsafe { Self::new_unchecked(value as f64) }
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
        impl TryFrom<$int> for $type<f64> {
            type Error = InvalidNumber;

            #[inline]
            fn try_from(value: $int) -> Result<Self, Self::Error> {
                Self::new(value as f64)
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

#[cfg(test)]
mod tests {
    macro_rules! impl_test {
        ($type:ident, $int:ident, $uint:ident) => {
            let ints = [
                $int::MIN,
                $int::MIN + 1,
                $int::MIN / 2,
                -2,
                -1,
                0,
                1,
                2,
                $int::MAX / 2,
                $int::MAX - 1,
                $int::MAX,
            ];

            for &i in &ints {
                // Will panic if an invalid value is created.
                let _ = crate::$type::<f32>::try_from(i);
                let _ = crate::$type::<f64>::try_from(i);
            }

            let uints = [0, 1, 2, $uint::MAX / 2, $uint::MAX - 1, $uint::MAX];

            for &i in &uints {
                // Will panic if an invalid value is created.
                let _ = crate::$type::<f32>::try_from(i);
                let _ = crate::$type::<f64>::try_from(i);
            }
        };
    }

    macro_rules! impl_tests {
        ($type:ident) => {
            impl_test!($type, i8, u8);
            impl_test!($type, i16, u16);
            impl_test!($type, i32, u32);
            impl_test!($type, i64, u64);
        };
    }

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
impl_try_from_ints!(NonZeroNonNaN, u8, u16, u32, u64);
impl_from_ints!(NonNaNFinite, u8, u16, u32, u64);
impl_try_from_ints!(NonZeroNonNaNFinite, u8, u16, u32, u64);
impl_from_ints!(Positive, u8, u16, u32, u64);
impl_try_from_ints!(Negative, u8, u16, u32, u64);
impl_from_ints!(PositiveFinite, u8, u16, u32, u64);
impl_try_from_ints!(NegativeFinite, u8, u16, u32, u64);
impl_try_from_ints!(StrictlyPositive, u8, u16, u32, u64);
impl_try_from_ints!(StrictlyNegative, u8, u16, u32, u64);
impl_try_from_ints!(StrictlyPositiveFinite, u8, u16, u32, u64);
impl_try_from_ints!(StrictlyNegativeFinite, u8, u16, u32, u64);
