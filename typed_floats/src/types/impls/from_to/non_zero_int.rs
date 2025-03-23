macro_rules! impl_from_int {
    ($type:ident,$int:ident) => {
        impl From<$int> for $type<f64> {
            #[inline]
            #[must_use]
            fn from(value: $int) -> Self {
                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_precision_loss)]
                #[allow(clippy::cast_lossless)]
                unsafe { Self::new_unchecked(value.get() as f64) }
            }
        }
        impl From<$int> for $type<f32> {
            #[inline]
            #[must_use]
            fn from(value: $int) -> Self {
                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_precision_loss)]
                #[allow(clippy::cast_lossless)]
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
                #[allow(clippy::cast_lossless)]
                #[allow(clippy::cast_precision_loss)]
                Self::new(value.get() as f64)
            }
        }
        impl TryFrom<$int> for $type<f32> {
            type Error = InvalidNumber;

            #[inline]
            fn try_from(value: $int) -> Result<Self, Self::Error> {
                #[allow(clippy::cast_lossless)]
                #[allow(clippy::cast_precision_loss)]
                Self::new(value.get() as f32)
            }
        }
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

#[cfg(test)]
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

#[cfg(test)]
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

// https://doc.rust-lang.org/1.49.0/reference/expressions/operator-expr.html#type-cast-expressions

// with the current set of numeric types, overflow can only happen on u128 as f64

// from non-zero signed integers

mod ints {
    use crate::{
        InvalidNumber, Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN,
        NonZeroNonNaNFinite, Positive, PositiveFinite, StrictlyNegative, StrictlyNegativeFinite,
        StrictlyPositive, StrictlyPositiveFinite,
    };

    use core::num::{NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8};

    impl_from_ints!(non_nan, NonNaN, NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64);
    impl_from_ints!(
        non_zero_non_nan,
        NonZeroNonNaN,
        NonZeroI8,
        NonZeroI16,
        NonZeroI32,
        NonZeroI64
    );
    impl_from_ints!(
        non_nan_finite,
        NonNaNFinite,
        NonZeroI8,
        NonZeroI16,
        NonZeroI32,
        NonZeroI64
    );
    impl_try_from_ints!(
        non_zero_non_nan_finite,
        NonZeroNonNaNFinite,
        NonZeroI8,
        NonZeroI16,
        NonZeroI32,
        NonZeroI64
    );
    impl_try_from_ints!(positive, Positive, NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64);
    impl_try_from_ints!(negative, Negative, NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64);
    impl_try_from_ints!(
        positive_finite,
        PositiveFinite,
        NonZeroI8,
        NonZeroI16,
        NonZeroI32,
        NonZeroI64
    );
    impl_try_from_ints!(
        negative_finite,
        NegativeFinite,
        NonZeroI8,
        NonZeroI16,
        NonZeroI32,
        NonZeroI64
    );
    impl_try_from_ints!(
        strictly_positive,
        StrictlyPositive,
        NonZeroI8,
        NonZeroI16,
        NonZeroI32,
        NonZeroI64
    );
    impl_try_from_ints!(
        strictly_negative,
        StrictlyNegative,
        NonZeroI8,
        NonZeroI16,
        NonZeroI32,
        NonZeroI64
    );
    impl_try_from_ints!(
        strictly_positive_finite,
        StrictlyPositiveFinite,
        NonZeroI8,
        NonZeroI16,
        NonZeroI32,
        NonZeroI64
    );
    impl_try_from_ints!(
        strictly_negative_finite,
        StrictlyNegativeFinite,
        NonZeroI8,
        NonZeroI16,
        NonZeroI32,
        NonZeroI64
    );
}

// from non-zero unsigned integers

mod uints {
    use crate::{
        InvalidNumber, Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN,
        NonZeroNonNaNFinite, Positive, PositiveFinite, StrictlyNegative, StrictlyNegativeFinite,
        StrictlyPositive, StrictlyPositiveFinite,
    };

    use core::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

    impl_from_ints!(non_nan, NonNaN, NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64);
    impl_from_ints!(
        non_zero_non_nan,
        NonZeroNonNaN,
        NonZeroU8,
        NonZeroU16,
        NonZeroU32,
        NonZeroU64
    );
    impl_from_ints!(
        non_nan_finite,
        NonNaNFinite,
        NonZeroU8,
        NonZeroU16,
        NonZeroU32,
        NonZeroU64
    );
    impl_from_ints!(
        non_zero_non_nan_finite,
        NonZeroNonNaNFinite,
        NonZeroU8,
        NonZeroU16,
        NonZeroU32,
        NonZeroU64
    );
    impl_from_ints!(positive, Positive, NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64);
    impl_try_from_ints!(negative, Negative, NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64);
    impl_from_ints!(
        positive_finite,
        PositiveFinite,
        NonZeroU8,
        NonZeroU16,
        NonZeroU32,
        NonZeroU64
    );
    impl_try_from_ints!(
        negative_finite,
        NegativeFinite,
        NonZeroU8,
        NonZeroU16,
        NonZeroU32,
        NonZeroU64
    );
    impl_from_ints!(
        strictly_positive,
        StrictlyPositive,
        NonZeroU8,
        NonZeroU16,
        NonZeroU32,
        NonZeroU64
    );
    impl_try_from_ints!(
        strictly_negative,
        StrictlyNegative,
        NonZeroU8,
        NonZeroU16,
        NonZeroU32,
        NonZeroU64
    );
    impl_from_ints!(
        strictly_positive_finite,
        StrictlyPositiveFinite,
        NonZeroU8,
        NonZeroU16,
        NonZeroU32,
        NonZeroU64
    );
    impl_try_from_ints!(
        strictly_negative_finite,
        StrictlyNegativeFinite,
        NonZeroU8,
        NonZeroU16,
        NonZeroU32,
        NonZeroU64
    );
}
