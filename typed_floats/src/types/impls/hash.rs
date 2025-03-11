use crate::{
    Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN, NonZeroNonNaNFinite, Positive,
    PositiveFinite, StrictlyNegative, StrictlyNegativeFinite, StrictlyPositive,
    StrictlyPositiveFinite,
};

// > When implementing both Hash and Eq, it is important that the following property holds:
// > `k1 == k2 -> hash(k1) == hash(k2)`
// This is sound because `NaN` is not a possible value.
// https://doc.rust-lang.org/core/hash/trait.Hash.html

const ZERO_BITS_F32: u32 = 0;
const ZERO_BITS_F64: u64 = 0;

impl core::hash::Hash for NonNaN<f32> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let bits = if self.0 == 0.0 {
            // `+0.0` and `-0.0` are equal to they must have the same hash
            ZERO_BITS_F32
        } else {
            self.0.to_bits()
        };

        bits.hash(state);
    }
}

impl core::hash::Hash for NonNaN<f64> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let bits = if self.0 == 0.0 {
            // `+0.0` and `-0.0` are equal to they must have the same hash
            ZERO_BITS_F64
        } else {
            self.0.to_bits()
        };

        bits.hash(state);
    }
}

impl core::hash::Hash for NonNaNFinite<f32> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let bits = if self.0 == 0.0 {
            // `+0.0` and `-0.0` are equal to they must have the same hash
            ZERO_BITS_F32
        } else {
            self.0.to_bits()
        };

        bits.hash(state);
    }
}

impl core::hash::Hash for NonNaNFinite<f64> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let bits = if self.0 == 0.0 {
            // `+0.0` and `-0.0` are equal to they must have the same hash
            ZERO_BITS_F64
        } else {
            self.0.to_bits()
        };

        bits.hash(state);
    }
}

macro_rules! impl_hash {
    ($test:ident, $type:ident) => {
        impl core::hash::Hash for $type<f32> {
            #[inline]
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.0.to_bits().hash(state);
            }
        }

        impl core::hash::Hash for $type<f64> {
            #[inline]
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.0.to_bits().hash(state);
            }
        }

        #[cfg(test)]
        mod $test {
            extern crate std;
            use crate::*;
            use std::vec::Vec; // Required for the tests to compile in no_std mode

            #[test]
            fn f32() {
                let mut hash_set = std::collections::HashSet::new();

                let neg_zero = $type::<f32>::new(-0.0);
                let pos_zero = $type::<f32>::new(0.0);
                let accept_both_zeroes = neg_zero.is_ok() && pos_zero.is_ok();
                if accept_both_zeroes {
                    let pos_one = $type::<f32>::new(1.0);
                    let neg_one = $type::<f32>::new(-1.0);

                    hash_set.insert(neg_zero.unwrap());
                    hash_set.insert(pos_zero.unwrap());
                    let mut count = 1; // Zeros are equal

                    if pos_one.is_ok() {
                        hash_set.insert(pos_one.unwrap());
                        count += 1;
                    }

                    if neg_one.is_ok() {
                        hash_set.insert(neg_one.unwrap());
                        count += 1;
                    }

                    assert_eq!(hash_set.len(), count);
                }

                let values = tf32::get_test_values()
                    .iter()
                    .map(|&x| tf32::$type::new(x))
                    .filter_map(|x| x.ok())
                    .collect::<Vec<_>>();

                let mut distincs = Vec::new();
                for x in values.iter() {
                    if !distincs.contains(x) {
                        distincs.push(*x);
                    }
                }

                let mut hash_set = std::collections::HashSet::new();

                for value in &values {
                    hash_set.insert(value);
                }

                assert_eq!(hash_set.len(), distincs.len());
            }

            #[test]
            fn f64() {
                let mut hash_set = std::collections::HashSet::new();

                let neg_zero = $type::<f32>::new(-0.0);
                let pos_zero = $type::<f32>::new(0.0);
                let accept_both_zeroes = neg_zero.is_ok() && pos_zero.is_ok();
                if accept_both_zeroes {
                    let pos_one = $type::<f32>::new(1.0);
                    let neg_one = $type::<f32>::new(-1.0);

                    hash_set.insert(neg_zero.unwrap());
                    hash_set.insert(pos_zero.unwrap());
                    let mut count = 1; // Zeros are equal

                    if pos_one.is_ok() {
                        hash_set.insert(pos_one.unwrap());
                        count += 1;
                    }

                    if neg_one.is_ok() {
                        hash_set.insert(neg_one.unwrap());
                        count += 1;
                    }

                    assert_eq!(hash_set.len(), count);
                }

                let values = tf64::get_test_values()
                    .iter()
                    .map(|&x| tf64::$type::new(x))
                    .filter_map(|x| x.ok())
                    .collect::<Vec<_>>();

                let mut distincs = Vec::new();
                for x in values.iter() {
                    if !distincs.contains(x) {
                        distincs.push(*x);
                    }
                }

                let mut hash_set = std::collections::HashSet::new();

                for value in &values {
                    hash_set.insert(value);
                }

                assert_eq!(hash_set.len(), distincs.len());
            }
        }
    };
}

impl_hash!(non_zero_non_nan, NonZeroNonNaN);
impl_hash!(non_zero_non_nan_finite, NonZeroNonNaNFinite);
impl_hash!(positive, Positive);
impl_hash!(negative, Negative);
impl_hash!(positive_finite, PositiveFinite);
impl_hash!(negative_finite, NegativeFinite);
impl_hash!(strictly_positive, StrictlyPositive);
impl_hash!(strictly_negative, StrictlyNegative);
impl_hash!(strictly_positive_finite, StrictlyPositiveFinite);
impl_hash!(strictly_negative_finite, StrictlyNegativeFinite);
