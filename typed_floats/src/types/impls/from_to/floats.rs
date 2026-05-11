use crate::{
    InvalidNormalizedNumber, InvalidNumber, Negative, NegativeFinite, NonNaN, NonNaNFinite,
    NonZeroNonNaN, NonZeroNonNaNFinite, Normalized, Positive, PositiveFinite, StrictlyNegative,
    StrictlyNegativeFinite, StrictlyPositive, StrictlyPositiveFinite,
};

macro_rules! impl_from {
    ($test:ident, $type:ident, $error:ident) => {
        #[cfg(feature = "f32")]
        impl From<$type<Self>> for f32 {
            #[inline]
            fn from(value: $type<Self>) -> Self {
                value.0
            }
        }

        #[cfg(feature = "f64")]
        impl From<$type<Self>> for f64 {
            #[inline]
            fn from(value: $type<Self>) -> Self {
                value.0
            }
        }

        #[cfg(feature = "f32")]
        impl TryFrom<f32> for $type<f32> {
            type Error = $error;

            #[inline]
            fn try_from(value: f32) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        #[cfg(feature = "f64")]
        impl TryFrom<f64> for $type<f64> {
            type Error = $error;

            #[inline]
            fn try_from(value: f64) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        #[test]
        fn $test() {
            #[cfg(feature = "f32")]
            {
                let values_f32 = crate::tf32::get_test_values();

                for &value in &values_f32 {
                    if let Ok(t) = $type::<f32>::new(value) {
                        crate::assert_float_eq!(value, t.get());
                        assert_eq!(t, unsafe { $type::<f32>::new_unchecked(value) });
                    }
                }
            }
            #[cfg(feature = "f64")]
            {
                let values_f64 = crate::tf64::get_test_values();

                for &value in &values_f64 {
                    if let Ok(t) = $type::<f64>::new(value) {
                        crate::assert_float_eq!(value, t.get());
                        assert_eq!(t, unsafe { $type::<f64>::new_unchecked(value) });
                    }
                }
            }
        }
    };
}

impl_from!(non_nan, NonNaN, InvalidNumber);
impl_from!(non_zero_non_nan, NonZeroNonNaN, InvalidNumber);
impl_from!(non_nan_finite, NonNaNFinite, InvalidNumber);
impl_from!(non_zero_non_nan_finite, NonZeroNonNaNFinite, InvalidNumber);
impl_from!(normalized, Normalized, InvalidNormalizedNumber);
impl_from!(positive, Positive, InvalidNumber);
impl_from!(negative, Negative, InvalidNumber);
impl_from!(positive_finite, PositiveFinite, InvalidNumber);
impl_from!(negative_finite, NegativeFinite, InvalidNumber);
impl_from!(strictly_positive, StrictlyPositive, InvalidNumber);
impl_from!(strictly_negative, StrictlyNegative, InvalidNumber);
impl_from!(
    strictly_positive_finite,
    StrictlyPositiveFinite,
    InvalidNumber
);
impl_from!(
    strictly_negative_finite,
    StrictlyNegativeFinite,
    InvalidNumber
);
