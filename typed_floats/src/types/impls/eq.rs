use crate::{
    Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN, NonZeroNonNaNFinite, Positive,
    PositiveFinite, StrictlyNegative, StrictlyNegativeFinite, StrictlyPositive,
    StrictlyPositiveFinite,
};

// This is safe because we know that both values are not NaN

macro_rules! impl_eq_self {
    ($type:ident) => {
        impl Eq for $type<f32> {}
        impl Eq for $type<f64> {}

        impl PartialEq for $type<f32> {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl PartialEq for $type<f64> {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
    };
}

macro_rules! impl_eq_base {
    ($type:ident) => {
        impl PartialEq<$type<f32>> for f32 {
            #[inline]
            fn eq(&self, other: &$type<f32>) -> bool {
                *self == other.0
            }
        }

        impl PartialEq<$type<f64>> for f64 {
            #[inline]
            fn eq(&self, other: &$type<f64>) -> bool {
                *self == other.0
            }
        }

        impl PartialEq<f32> for $type<f32> {
            #[inline]
            fn eq(&self, other: &f32) -> bool {
                self.0 == *other
            }
        }

        impl PartialEq<f64> for $type<f64> {
            #[inline]
            fn eq(&self, other: &f64) -> bool {
                self.0 == *other
            }
        }
    };
}

// This fast implementation can only be used for types that rejects `-0.0` and/or `+0.0`
macro_rules! impl_fast_eq_self {
    ($type:ident) => {
        impl Eq for $type<f32> {}
        impl Eq for $type<f64> {}

        impl PartialEq for $type<f32> {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0.to_bits() == other.0.to_bits()
            }
        }

        impl PartialEq for $type<f64> {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0.to_bits() == other.0.to_bits()
            }
        }
    };
}

// This fast implementation can only be used for types that rejects `-0.0` AND `+0.0`
macro_rules! impl_fast_eq_base {
    ($type:ident) => {
        impl PartialEq<$type<f32>> for f32 {
            #[inline]
            fn eq(&self, other: &$type<f32>) -> bool {
                self.to_bits() == (&other.0).to_bits()
            }
        }

        impl PartialEq<$type<f64>> for f64 {
            #[inline]
            fn eq(&self, other: &$type<f64>) -> bool {
                self.to_bits() == (&other.0).to_bits()
            }
        }

        impl PartialEq<f32> for $type<f32> {
            #[inline]
            fn eq(&self, other: &f32) -> bool {
                (&self.0).to_bits() == other.to_bits()
            }
        }

        impl PartialEq<f64> for $type<f64> {
            #[inline]
            fn eq(&self, other: &f64) -> bool {
                (&self.0).to_bits() == other.to_bits()
            }
        }
    };
}

impl_eq_base!(NonNaN);
impl_eq_self!(NonNaN);

impl_fast_eq_base!(NonZeroNonNaN);
impl_fast_eq_self!(NonZeroNonNaN);

impl_eq_base!(NonNaNFinite);
impl_eq_self!(NonNaNFinite);

impl_fast_eq_base!(NonZeroNonNaNFinite);
impl_fast_eq_self!(NonZeroNonNaNFinite);

impl_eq_base!(Positive);
impl_fast_eq_self!(Positive);

impl_eq_base!(Negative);
impl_fast_eq_self!(Negative);

impl_eq_base!(PositiveFinite);
impl_fast_eq_self!(PositiveFinite);

impl_eq_base!(NegativeFinite);
impl_fast_eq_self!(NegativeFinite);

impl_fast_eq_base!(StrictlyPositive);
impl_fast_eq_self!(StrictlyPositive);

impl_fast_eq_base!(StrictlyNegative);
impl_fast_eq_self!(StrictlyNegative);

impl_fast_eq_base!(StrictlyPositiveFinite);
impl_fast_eq_self!(StrictlyPositiveFinite);

impl_fast_eq_base!(StrictlyNegativeFinite);
impl_fast_eq_self!(StrictlyNegativeFinite);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! impl_eq_test {
        ($test:ident, $type:ident) => {
            #[test]
            fn $test() {
                let values_f32 = crate::tf32::get_test_values();
                let nan = f32::NAN;
                for &value in &values_f32 {
                    if let Ok(t) = $type::<f32>::new(value) {
                        assert_eq!(t, t);
                        assert_eq!(t, value);
                        assert_eq!(value, t);

                        assert_ne!(t, nan);
                        assert_ne!(nan, t);

                        if value == 0.0 {
                            assert_eq!(t, -value);
                            assert_eq!(-value, t);
                            assert_eq!(t, 0.0);
                            assert_eq!(t, -0.0);
                        } else {
                            assert_ne!(t, -value);
                            assert_ne!(-value, t);
                            assert_ne!(t, 0.0);
                            assert_ne!(t, -0.0);
                        }

                        for &other in &values_f32 {
                            if let Ok(other_t) = $type::<f32>::new(other) {
                                if other == value {
                                    assert_eq!(t, other_t);
                                    assert_eq!(other_t, t);
                                    assert_eq!(t, other);
                                    assert_eq!(other, t);
                                    assert_eq!(value, other);
                                    assert_eq!(other, value);
                                } else {
                                    assert_ne!(t, other_t);
                                    assert_ne!(other_t, t);
                                    assert_ne!(t, other);
                                    assert_ne!(other, t);
                                    assert_ne!(value, other);
                                    assert_ne!(other, value);
                                }
                            }
                        }
                    }
                }

                let nan = f64::NAN;
                let values_f64 = crate::tf64::get_test_values();
                for &value in &values_f64 {
                    if let Ok(t) = $type::<f64>::new(value) {
                        assert_eq!(t, t);
                        assert_eq!(t, value);
                        assert_eq!(value, t);

                        assert_ne!(t, nan);
                        assert_ne!(nan, t);

                        if value == 0.0 {
                            assert_eq!(t, -value);
                            assert_eq!(-value, t);
                            assert_eq!(t, 0.0);
                            assert_eq!(t, -0.0);
                        } else {
                            assert_ne!(t, -value);
                            assert_ne!(-value, t);
                            assert_ne!(t, 0.0);
                            assert_ne!(t, -0.0);
                        }

                        for &other in &values_f64 {
                            if let Ok(other_t) = $type::<f64>::new(other) {
                                if other == value {
                                    assert_eq!(t, other_t);
                                    assert_eq!(other_t, t);
                                    assert_eq!(t, other);
                                    assert_eq!(other, t);
                                } else {
                                    assert_ne!(t, other_t);
                                    assert_ne!(other_t, t);
                                    assert_ne!(t, other);
                                    assert_ne!(other, t);
                                }
                            }
                        }
                    }
                }
            }
        };
    }

    impl_eq_test!(non_nan, NonNaN);
    impl_eq_test!(non_nan_finite, NonNaNFinite);
    impl_eq_test!(non_zero_non_nan, NonZeroNonNaN);
    impl_eq_test!(non_zero_non_nan_finite, NonZeroNonNaNFinite);
    impl_eq_test!(positive, Positive);
    impl_eq_test!(negative, Negative);
    impl_eq_test!(positive_finite, PositiveFinite);
    impl_eq_test!(negative_finite, NegativeFinite);
    impl_eq_test!(strictly_positive, StrictlyPositive);
    impl_eq_test!(strictly_negative, StrictlyNegative);
    impl_eq_test!(strictly_positive_finite, StrictlyPositiveFinite);
    impl_eq_test!(strictly_negative_finite, StrictlyNegativeFinite);
}
