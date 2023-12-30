use crate::{
    Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN, NonZeroNonNaNFinite, Positive,
    PositiveFinite, StrictlyNegative, StrictlyNegativeFinite, StrictlyPositive,
    StrictlyPositiveFinite,
};

// This is safe because we know that both values are not NaN

macro_rules! impl_eq {
    ($test:ident, $type:ident) => {
        impl Eq for $type<f32> {}
        impl Eq for $type<f64> {}

        impl PartialEq for $type<f32> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl PartialEq for $type<f64> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl PartialEq<$type<f32>> for f32 {
            #[inline]
            fn eq(&self, other: &$type<f32>) -> bool {
                self == &other.0
            }
        }

        impl PartialEq<$type<f64>> for f64 {
            #[inline]
            fn eq(&self, other: &$type<f64>) -> bool {
                self == &other.0
            }
        }

        impl PartialEq<f32> for $type<f32> {
            #[inline]
            fn eq(&self, other: &f32) -> bool {
                &self.0 == other
            }
        }

        impl PartialEq<f64> for $type<f64> {
            #[inline]
            fn eq(&self, other: &f64) -> bool {
                &self.0 == other
            }
        }

        #[test]
        fn $test() {
            let values_f32 = [-1.0, 1.0];

            for &value in &values_f32 {
                if let Ok(t) = $type::<f32>::new(value) {
                    assert_eq!(t, t);
                    assert_eq!(t, value);
                    assert_eq!(value, t);

                    assert_ne!(t, -value);
                    assert_ne!(-value, t);
                    assert_ne!(t, 0.0);


                }
            }
        }
    };
}

impl_eq!(non_nan, NonNaN);
impl_eq!(non_zero_non_nan, NonZeroNonNaN);
impl_eq!(non_nan_finite, NonNaNFinite);
impl_eq!(non_zero_non_nan_finite, NonZeroNonNaNFinite);
impl_eq!(positive, Positive);
impl_eq!(negative, Negative);
impl_eq!(positive_finite, PositiveFinite);
impl_eq!(negative_finite, NegativeFinite);
impl_eq!(strictly_positive, StrictlyPositive);
impl_eq!(strictly_negative, StrictlyNegative);
impl_eq!(strictly_positive_finite, StrictlyPositiveFinite);
impl_eq!(strictly_negative_finite, StrictlyNegativeFinite);
