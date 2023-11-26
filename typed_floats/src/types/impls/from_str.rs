use crate::{
    FromStrError, Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN,
    NonZeroNonNaNFinite, Positive, PositiveFinite, StrictlyNegative, StrictlyNegativeFinite,
    StrictlyPositive, StrictlyPositiveFinite,
};

macro_rules! impl_from_str {
    ($test:ident, $type:ident) => {
        impl core::str::FromStr for $type<f32> {
            type Err = FromStrError;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let f: f32 = s.parse::<f32>().map_err(FromStrError::ParseFloatError)?;

                Self::try_from(f).map_err(FromStrError::InvalidNumber)
            }
        }

        impl core::str::FromStr for $type<f64> {
            type Err = FromStrError;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let f: f64 = s.parse::<f64>().map_err(FromStrError::ParseFloatError)?;

                Self::try_from(f).map_err(FromStrError::InvalidNumber)
            }
        }
        
        #[test]
        fn $test() {
            let values_f32 = [
                f32::NEG_INFINITY,
                f32::MIN,
                f32::MIN / 2.0,
                -1.0,
                -0.0,
                0.0,
                1.0,
                f32::MAX / 2.0,
                f32::MAX,
                f32::INFINITY,
            ];

            for &value in &values_f32 {
                if $type::<f32>::new(value).is_ok() {
                    let str = format!("{}", value);

                    let t = str.parse::<$type<f32>>().unwrap();

                    assert_eq!(t.get(), value);
                }
            }

            let values_f64 = [
                f64::NEG_INFINITY,
                f64::MIN,
                f64::MIN / 2.0,
                -1.0,
                -0.0,
                0.0,
                1.0,
                f64::MAX / 2.0,
                f64::MAX,
                f64::INFINITY,
            ];

            for &value in &values_f64 {
                if $type::<f64>::new(value).is_ok() {
                    let str = format!("{}", value);

                    let t = str.parse::<$type<f64>>().unwrap();
                    
                    assert_eq!(t.get(), value);
                }
            }
        }

    };
}

impl_from_str!(non_nan, NonNaN);
impl_from_str!(non_zero_non_nan, NonZeroNonNaN);
impl_from_str!(non_nan_finite, NonNaNFinite);
impl_from_str!(non_zero_non_nan_finite, NonZeroNonNaNFinite);
impl_from_str!(positive, Positive);
impl_from_str!(negative, Negative);
impl_from_str!(positive_finite, PositiveFinite);
impl_from_str!(negative_finite, NegativeFinite);
impl_from_str!(strictly_positive, StrictlyPositive);
impl_from_str!(strictly_negative, StrictlyNegative);
impl_from_str!(strictly_positive_finite, StrictlyPositiveFinite);
impl_from_str!(strictly_negative_finite, StrictlyNegativeFinite);
