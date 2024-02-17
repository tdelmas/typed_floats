#![allow(clippy::float_cmp)]

use crate::{
    InvalidNumber, Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN,
    NonZeroNonNaNFinite, Positive, PositiveFinite, StrictlyNegative, StrictlyNegativeFinite,
    StrictlyPositive, StrictlyPositiveFinite,
};

macro_rules! impl_from {
    ($test:ident, $type:ident) => {
        impl From<$type<Self>> for f32 {
            #[inline]
            #[must_use]
            fn from(value: $type<Self>) -> Self {
                value.0
            }
        }

        impl From<$type<Self>> for f64 {
            #[inline]
            #[must_use]
            fn from(value: $type<Self>) -> Self {
                value.0
            }
        }

        impl TryFrom<f32> for $type<f32> {
            type Error = InvalidNumber;

            #[inline]
            fn try_from(value: f32) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl TryFrom<f64> for $type<f64> {
            type Error = InvalidNumber;

            #[inline]
            fn try_from(value: f64) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        #[test]
        fn $test() {
            let values_f32 = crate::tf32::TEST_VALUES;

            for &value in &values_f32 {
                if let Ok(t) = $type::<f32>::new(value) {
                    assert_eq!(value, t.get());
                    assert_eq!(t, unsafe { $type::<f32>::new_unchecked(value) });
                }
            }

            let values_f64 = crate::tf64::TEST_VALUES;

            for &value in &values_f64 {
                if let Ok(t) = $type::<f64>::new(value) {
                    assert_eq!(value, t.get());
                    assert_eq!(t, unsafe { $type::<f64>::new_unchecked(value) });
                }
            }
        }
    };
}

impl_from!(non_nan, NonNaN);
impl_from!(non_zero_non_nan, NonZeroNonNaN);
impl_from!(non_nan_finite, NonNaNFinite);
impl_from!(non_zero_non_nan_finite, NonZeroNonNaNFinite);
impl_from!(positive, Positive);
impl_from!(negative, Negative);
impl_from!(positive_finite, PositiveFinite);
impl_from!(negative_finite, NegativeFinite);
impl_from!(strictly_positive, StrictlyPositive);
impl_from!(strictly_negative, StrictlyNegative);
impl_from!(strictly_positive_finite, StrictlyPositiveFinite);
impl_from!(strictly_negative_finite, StrictlyNegativeFinite);
