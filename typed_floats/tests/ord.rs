macro_rules! impl_ord_test {
    ($test:ident, $type:ident) => {
        mod $test {
            extern crate std;
            use std::vec::Vec; // Required for the tests to compile in no_std mode
            use typed_floats::*;

            fn is_sorted<T: PartialOrd>(slice: &[T]) -> bool {
                slice.windows(2).all(|w| w[0] <= w[1])
            }

            #[test]
            #[cfg(feature = "f32")]
            fn f32() {
                let min_positive = tf32::MIN_POSITIVE;
                let max = tf32::MAX;
                let infinity = tf32::INFINITY;

                assert!(min_positive < max);
                assert!(min_positive < infinity);
                assert!(max < infinity);

                let values = tf32::get_test_values()
                    .iter()
                    .map(|&x| tf32::$type::new(x))
                    .filter_map(|x| x.ok())
                    .collect::<Vec<_>>();

                assert!(is_sorted(&values));
                let floats = values.iter().map(|x| x.get()).collect::<Vec<_>>();
                assert!(is_sorted(&floats));

                let reversed = values.iter().rev().collect::<Vec<_>>();

                assert!(!is_sorted(&reversed));
                let floats = reversed.iter().map(|x| x.get()).collect::<Vec<_>>();
                assert!(!is_sorted(&floats));

                let mut sorted = reversed.clone();
                sorted.sort();

                assert!(is_sorted(&sorted));
                let floats = sorted.iter().map(|x| x.get()).collect::<Vec<_>>();
                assert!(is_sorted(&floats));
            }

            #[test]
            #[cfg(feature = "f64")]
            fn f64() {
                let min_positive = tf64::MIN_POSITIVE;
                let max = tf64::MAX;
                let infinity = tf64::INFINITY;

                assert!(min_positive < max);
                assert!(min_positive < infinity);
                assert!(max < infinity);

                let values = tf64::get_test_values()
                    .iter()
                    .map(|&x| tf64::$type::new(x))
                    .filter_map(|x| x.ok())
                    .collect::<Vec<_>>();

                assert!(is_sorted(&values));
                let floats = values.iter().map(|x| x.get()).collect::<Vec<_>>();
                assert!(is_sorted(&floats));

                let reversed = values.iter().rev().collect::<Vec<_>>();

                assert!(!is_sorted(&reversed));
                let floats = reversed.iter().map(|x| x.get()).collect::<Vec<_>>();
                assert!(!is_sorted(&floats));

                let mut sorted = reversed.clone();
                sorted.sort();

                assert!(is_sorted(&sorted));
                let floats = sorted.iter().map(|x| x.get()).collect::<Vec<_>>();
                assert!(is_sorted(&floats));
            }

            #[test]
            #[cfg(feature = "f64")]
            fn cmp_with_others() {
                let values_non_nan = tf64::get_test_values()
                    .iter()
                    .map(|&x| tf64::NonNaN::new(x))
                    .filter_map(|x| x.ok())
                    .collect::<Vec<_>>();

                let values_non_zero_non_nan = tf64::get_test_values()
                    .iter()
                    .map(|&x| tf64::NonZeroNonNaN::new(x))
                    .filter_map(|x| x.ok())
                    .collect::<Vec<_>>();

                let values = tf64::get_test_values()
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

                    for b in &tf64::get_test_values() {
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

impl_ord_test!(ord_non_nan, NonNaN);
impl_ord_test!(ord_non_zero_non_nan, NonZeroNonNaN);
impl_ord_test!(ord_non_nan_finite, NonNaNFinite);
impl_ord_test!(ord_non_zero_non_nan_finite, NonZeroNonNaNFinite);
impl_ord_test!(ord_positive, Positive);
impl_ord_test!(ord_negative, Negative);
impl_ord_test!(ord_positive_finite, PositiveFinite);
impl_ord_test!(ord_negative_finite, NegativeFinite);
impl_ord_test!(ord_strictly_positive, StrictlyPositive);
impl_ord_test!(ord_strictly_negative, StrictlyNegative);
impl_ord_test!(ord_strictly_positive_finite, StrictlyPositiveFinite);
impl_ord_test!(ord_strictly_negative_finite, StrictlyNegativeFinite);
