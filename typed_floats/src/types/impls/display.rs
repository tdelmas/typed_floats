use crate::{
    Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN, NonZeroNonNaNFinite, Positive,
    PositiveFinite, StrictlyNegative, StrictlyNegativeFinite, StrictlyPositive,
    StrictlyPositiveFinite,
};

macro_rules! impl_display {
    ($test:ident, $type:ident) => {
        impl core::fmt::Display for $type<f32> {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        
        impl core::fmt::Display for $type<f64> {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.0)
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
            ];

            for &value in &values_f32 {
                if let Ok(t) = $type::<f32>::new(value) {
                    let str1 = format!("{}", t);
                    let str2 = format!("{}", value);
                    assert_eq!(str1, str2);
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
            ];

            for &value in &values_f64 {
                if let Ok(t) = $type::<f64>::new(value) {
                    let str1 = format!("{}", t);
                    let str2 = format!("{}", value);
                    assert_eq!(str1, str2);
                }
            }
        }
    };
}

impl_display!(non_nan, NonNaN);
impl_display!(non_zero_non_nan, NonZeroNonNaN);
impl_display!(non_nan_finite, NonNaNFinite);
impl_display!(non_zero_non_nan_finite, NonZeroNonNaNFinite);
impl_display!(positive, Positive);
impl_display!(negative, Negative);
impl_display!(positive_finite, PositiveFinite);
impl_display!(negative_finite, NegativeFinite);
impl_display!(strictly_positive, StrictlyPositive);
impl_display!(strictly_negative, StrictlyNegative);
impl_display!(strictly_positive_finite, StrictlyPositiveFinite);
impl_display!(strictly_negative_finite, StrictlyNegativeFinite);
