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

#[cfg(test)]
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

#[cfg(test)]
macro_rules! impl_tests {
    ($type:ident) => {
        impl_test!($type, i8, u8);
        impl_test!($type, i16, u16);
        impl_test!($type, i32, u32);
        impl_test!($type, i64, u64);
    };
}

macro_rules! impl_from_ints {
    ($test:ident, $type:ident, $($int:ident),*) => {
        $(
            impl_from_int!($type, $int);
        )*

        #[test]
        fn $test() {
            impl_tests!($type);
        }
    };
}

macro_rules! impl_try_from_ints {
    ($test:ident, $type:ident, $($int:ident),*) => {
        $(
            impl_try_from_int!($type, $int);
        )*

        #[test]
        fn $test() {
            impl_tests!($type);
        }
    };
}

// https://doc.rust-lang.org/1.49.0/reference/expressions/operator-expr.html#type-cast-expressions

// with the current set of numeric types, overflow can only happen on u128 as f64

// from signed integers
mod ints {
    use crate::{
        InvalidNumber, Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN,
        NonZeroNonNaNFinite, Positive, PositiveFinite, StrictlyNegative, StrictlyNegativeFinite,
        StrictlyPositive, StrictlyPositiveFinite,
    };

    impl_from_ints!(non_nan, NonNaN, i8, i16, i32, i64);
    impl_try_from_ints!(non_zero_non_nan, NonZeroNonNaN, i8, i16, i32, i64);
    impl_from_ints!(non_nan_finite, NonNaNFinite, i8, i16, i32, i64);
    impl_try_from_ints!(
        non_zero_non_nan_finite,
        NonZeroNonNaNFinite,
        i8,
        i16,
        i32,
        i64
    );
    impl_try_from_ints!(positive, Positive, i8, i16, i32, i64);
    impl_try_from_ints!(negative, Negative, i8, i16, i32, i64);
    impl_try_from_ints!(positive_finite, PositiveFinite, i8, i16, i32, i64);
    impl_try_from_ints!(negative_finite, NegativeFinite, i8, i16, i32, i64);
    impl_try_from_ints!(strictly_positive, StrictlyPositive, i8, i16, i32, i64);
    impl_try_from_ints!(strictly_negative, StrictlyNegative, i8, i16, i32, i64);
    impl_try_from_ints!(
        strictly_positive_finite,
        StrictlyPositiveFinite,
        i8,
        i16,
        i32,
        i64
    );
    impl_try_from_ints!(
        strictly_negative_finite,
        StrictlyNegativeFinite,
        i8,
        i16,
        i32,
        i64
    );
}

// from unsigned integers

mod uints {
    use crate::{
        InvalidNumber, Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN,
        NonZeroNonNaNFinite, Positive, PositiveFinite, StrictlyNegative, StrictlyNegativeFinite,
        StrictlyPositive, StrictlyPositiveFinite,
    };

    impl_from_ints!(non_nan, NonNaN, u8, u16, u32, u64);
    impl_try_from_ints!(non_zero_non_nan, NonZeroNonNaN, u8, u16, u32, u64);
    impl_from_ints!(non_nan_finite, NonNaNFinite, u8, u16, u32, u64);
    impl_try_from_ints!(
        non_zero_non_nan_finite,
        NonZeroNonNaNFinite,
        u8,
        u16,
        u32,
        u64
    );
    impl_from_ints!(positive, Positive, u8, u16, u32, u64);
    impl_try_from_ints!(negative, Negative, u8, u16, u32, u64);
    impl_from_ints!(positive_finite, PositiveFinite, u8, u16, u32, u64);
    impl_try_from_ints!(negative_finite, NegativeFinite, u8, u16, u32, u64);
    impl_try_from_ints!(strictly_positive, StrictlyPositive, u8, u16, u32, u64);
    impl_try_from_ints!(strictly_negative, StrictlyNegative, u8, u16, u32, u64);
    impl_try_from_ints!(
        strictly_positive_finite,
        StrictlyPositiveFinite,
        u8,
        u16,
        u32,
        u64
    );
    impl_try_from_ints!(
        strictly_negative_finite,
        StrictlyNegativeFinite,
        u8,
        u16,
        u32,
        u64
    );
}
