mod negative;
mod negative_finite;
mod non_nan;
mod non_nan_finite;
mod non_zero_non_nan;
mod non_zero_non_nan_finite;
mod positive;
mod positive_finite;
mod strictly_negative;
mod strictly_negative_finite;
mod strictly_positive;
mod strictly_positive_finite;

#[cfg(test)]
macro_rules! test_type {
    ($test:ident, $type:ty) => {
        #[test]
        fn $test() {
            let values = crate::tf64::TEST_VALUES;

            for &value in &values {
                let v: Option<$type> = value.try_into().ok();
                if let Some(v) = v {
                    crate::assert_float_eq!(v.get(), value);
                    assert_eq!(v.is_nan(), false);
                    assert_eq!(v.is_infinite(), value.is_infinite());
                    assert_eq!(v.is_finite(), value.is_finite());
                    assert_eq!(v.is_subnormal(), value.is_subnormal());
                    assert_eq!(v.is_normal(), value.is_normal());
                    assert_eq!(v.classify(), value.classify());
                    assert_eq!(v.is_sign_positive(), value.is_sign_positive());
                    assert_eq!(v.is_sign_negative(), value.is_sign_negative());
                    assert_eq!(
                        v.is_positive_zero(),
                        value.is_sign_positive() && value == 0.0
                    );
                    assert_eq!(
                        v.is_negative_zero(),
                        value.is_sign_negative() && value == 0.0
                    );
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::tf64::*;

    test_type!(negative, Negative);
    test_type!(negative_finite, NegativeFinite);
    test_type!(non_nan, NonNaN);
    test_type!(non_nan_finite, NonNaNFinite);
    test_type!(non_zero_non_nan, NonZeroNonNaN);
    test_type!(non_zero_non_nan_finite, NonZeroNonNaNFinite);
    test_type!(positive, Positive);
    test_type!(positive_finite, PositiveFinite);
    test_type!(strictly_negative, StrictlyNegative);
    test_type!(strictly_negative_finite, StrictlyNegativeFinite);
    test_type!(strictly_positive, StrictlyPositive);
    test_type!(strictly_positive_finite, StrictlyPositiveFinite);
}
