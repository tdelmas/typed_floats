use crate::{
    FromStrError, Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN,
    NonZeroNonNaNFinite, Normalized, NormalizedFromStrError, Positive, PositiveFinite,
    StrictlyNegative, StrictlyNegativeFinite, StrictlyPositive, StrictlyPositiveFinite,
};

macro_rules! impl_from_str {
    ($test:ident, $type:ident, $error:ident) => {
        #[cfg(feature = "f32")]
        impl core::str::FromStr for $type<f32> {
            type Err = $error;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let f: f32 = s.parse::<f32>().map_err($error::ParseFloatError)?;

                Self::try_from(f).map_err(Self::Err::InvalidNumber)
            }
        }

        #[cfg(feature = "f64")]
        impl core::str::FromStr for $type<f64> {
            type Err = $error;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let f: f64 = s.parse::<f64>().map_err($error::ParseFloatError)?;

                Self::try_from(f).map_err(Self::Err::InvalidNumber)
            }
        }

        #[test]
        fn $test() {
            #[cfg(feature = "f32")]
            {
                let values_f32 = crate::tf32::get_test_values();

                for &value in &values_f32 {
                    if $type::<f32>::new(value).is_ok() {
                        let str = format!("{}", value);

                        let t = str.parse::<$type<f32>>().unwrap();

                        assert_eq!(t.get(), value);
                    }
                }
            }

            #[cfg(feature = "f64")]
            {
                let values_f64 = crate::tf64::get_test_values();

                for &value in &values_f64 {
                    if $type::<f64>::new(value).is_ok() {
                        let str = format!("{}", value);

                        let t = str.parse::<$type<f64>>().unwrap();

                        assert_eq!(t.get(), value);
                    }
                }
            }
        }
    };
}

impl_from_str!(non_nan, NonNaN, FromStrError);
impl_from_str!(non_zero_non_nan, NonZeroNonNaN, FromStrError);
impl_from_str!(non_nan_finite, NonNaNFinite, FromStrError);
impl_from_str!(non_zero_non_nan_finite, NonZeroNonNaNFinite, FromStrError);
impl_from_str!(normalized, Normalized, NormalizedFromStrError);
impl_from_str!(positive, Positive, FromStrError);
impl_from_str!(negative, Negative, FromStrError);
impl_from_str!(positive_finite, PositiveFinite, FromStrError);
impl_from_str!(negative_finite, NegativeFinite, FromStrError);
impl_from_str!(strictly_positive, StrictlyPositive, FromStrError);
impl_from_str!(strictly_negative, StrictlyNegative, FromStrError);
impl_from_str!(
    strictly_positive_finite,
    StrictlyPositiveFinite,
    FromStrError
);
impl_from_str!(
    strictly_negative_finite,
    StrictlyNegativeFinite,
    FromStrError
);
