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

        impl PartialOrd for $type<f64> {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        #[cfg(test)]
        mod $test {
            use crate::*;

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

                let sorted = values.iter().collect::<Vec<_>>();

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

                let sorted = values.iter().collect::<Vec<_>>();

                assert!(is_sorted(&sorted));
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
