#![allow(clippy::comparison_chain)]

use crate::{
    Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN, NonZeroNonNaNFinite, Positive,
    PositiveFinite, StrictlyNegative, StrictlyNegativeFinite, StrictlyPositive,
    StrictlyPositiveFinite,
};

macro_rules! impl_ord {
    ($test:ident, $type:ident) => {
        impl Ord for $type<f32> {
            #[inline]
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                #[allow(clippy::float_cmp)]
                if self.get() < other.get() {
                    core::cmp::Ordering::Less
                } else if self.get() == other.get() {
                    core::cmp::Ordering::Equal
                } else {
                    core::cmp::Ordering::Greater
                }
            }
        }

        impl Ord for $type<f64> {
            #[inline]
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                #[allow(clippy::float_cmp)]
                if self.get() < other.get() {
                    core::cmp::Ordering::Less
                } else if self.get() == other.get() {
                    core::cmp::Ordering::Equal
                } else {
                    core::cmp::Ordering::Greater
                }
            }
        }

        impl PartialOrd for $type<f32> {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialOrd<f32> for $type<f32> {
            #[inline]
            fn partial_cmp(&self, other: &f32) -> Option<core::cmp::Ordering> {
                self.get().partial_cmp(other)
            }
        }

        impl PartialOrd<$type<f32>> for f32 {
            #[inline]
            fn partial_cmp(&self, other: &$type<f32>) -> Option<core::cmp::Ordering> {
                self.partial_cmp(&other.0)
            }
        }

        impl PartialOrd for $type<f64> {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialOrd<f64> for $type<f64> {
            #[inline]
            fn partial_cmp(&self, other: &f64) -> Option<core::cmp::Ordering> {
                self.get().partial_cmp(other)
            }
        }

        impl PartialOrd<$type<f64>> for f64 {
            #[inline]
            fn partial_cmp(&self, other: &$type<f64>) -> Option<core::cmp::Ordering> {
                self.partial_cmp(&other.0)
            }
        }

        #[cfg(test)]
        mod $test {
            extern crate std;
            use crate::*;
            use std::vec::Vec; // Required for the tests to compile in no_std mode

            fn is_sorted<T: Ord>(slice: &[T]) -> bool {
                slice.windows(2).all(|w| w[0] <= w[1])
            }

            #[test]
            fn f32() {
                let values = tf32::TEST_VALUES
                    .iter()
                    .map(|&x| tf32::$type::new(x))
                    .filter_map(|x| x.ok())
                    .collect::<Vec<_>>();

                assert!(is_sorted(&values));

                let reversed = values.iter().rev().collect::<Vec<_>>();

                assert!(!is_sorted(&reversed));

                let mut sorted = reversed.clone();
                sorted.sort();

                assert!(is_sorted(&sorted));
            }

            #[test]
            fn f64() {
                let values = tf64::TEST_VALUES
                    .iter()
                    .map(|&x| tf64::$type::new(x))
                    .filter_map(|x| x.ok())
                    .collect::<Vec<_>>();

                assert!(is_sorted(&values));

                let reversed = values.iter().rev().collect::<Vec<_>>();

                assert!(!is_sorted(&reversed));

                let mut sorted = reversed.clone();
                sorted.sort();

                assert!(is_sorted(&sorted));
            }

            #[test]
            fn cmp_with_others() {
                let values_non_nan = tf64::TEST_VALUES
                    .iter()
                    .map(|&x| tf64::NonNaN::new(x))
                    .filter_map(|x| x.ok())
                    .collect::<Vec<_>>();
                
                let values_non_zero_non_nan = tf64::TEST_VALUES
                    .iter()
                    .map(|&x| tf64::NonZeroNonNaN::new(x))
                    .filter_map(|x| x.ok())
                    .collect::<Vec<_>>();
                
                let values = tf64::TEST_VALUES
                    .iter()
                    .map(|&x| tf64::$type::new(x))
                    .filter_map(|x| x.ok())
                    .collect::<Vec<_>>();

                for a in &values {
                    for b in &values_non_nan {
                        let res = a.get().partial_cmp(&b.get());
                        let reversed = b.get().partial_cmp(&a.get());

                        assert_eq!(res, a.partial_cmp(b));
                        assert_eq!(reversed, b.partial_cmp(a));
                    }
                    
                    for b in &values_non_zero_non_nan {
                        let res = a.get().partial_cmp(&b.get());
                        let reversed = b.get().partial_cmp(&a.get());

                        assert_eq!(res, a.partial_cmp(b));
                        assert_eq!(reversed, b.partial_cmp(a));
                    }
                    
                    for b in &tf64::TEST_VALUES {
                        let res = a.get().partial_cmp(b);
                        let reversed = b.partial_cmp(&a.get());

                        assert_eq!(res, a.partial_cmp(b));
                        assert_eq!(reversed, b.partial_cmp(a));
                    }
                }

            }
        }
    };
}

impl_ord!(non_nan, NonNaN);
impl_ord!(non_zero_non_nan, NonZeroNonNaN);
impl_ord!(non_nan_finite, NonNaNFinite);
impl_ord!(non_zero_non_nan_finite, NonZeroNonNaNFinite);
impl_ord!(positive, Positive);
impl_ord!(negative, Negative);
impl_ord!(positive_finite, PositiveFinite);
impl_ord!(negative_finite, NegativeFinite);
impl_ord!(strictly_positive, StrictlyPositive);
impl_ord!(strictly_negative, StrictlyNegative);
impl_ord!(strictly_positive_finite, StrictlyPositiveFinite);
impl_ord!(strictly_negative_finite, StrictlyNegativeFinite);
