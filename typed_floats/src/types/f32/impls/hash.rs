use crate::{
    Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN, NonZeroNonNaNFinite, Positive,
    PositiveFinite, StrictlyNegative, StrictlyNegativeFinite, StrictlyPositive,
    StrictlyPositiveFinite,
};

// > When implementing both Hash and Eq, it is important that the following property holds:
// > `k1 == k2 -> hash(k1) == hash(k2)`
// This is sound because `NaN` is not a possible value.
// https://doc.rust-lang.org/core/hash/trait.Hash.html

const ZERO_BITS: u32 = 0;

impl core::hash::Hash for NonNaN<f32> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let bits = if self.0 == 0.0 {
            // `+0.0` and `-0.0` are equal to they must have the same hash
            ZERO_BITS
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
            ZERO_BITS
        } else {
            self.0.to_bits()
        };

        bits.hash(state);
    }
}

macro_rules! impl_hash {
    ($type:ident) => {
        impl core::hash::Hash for $type<f32> {
            #[inline]
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.0.to_bits().hash(state);
            }
        }
    };
}

impl_hash!(NonZeroNonNaN);
impl_hash!(NonZeroNonNaNFinite);
impl_hash!(Positive);
impl_hash!(Negative);
impl_hash!(PositiveFinite);
impl_hash!(NegativeFinite);
impl_hash!(StrictlyPositive);
impl_hash!(StrictlyNegative);
impl_hash!(StrictlyPositiveFinite);
impl_hash!(StrictlyNegativeFinite);
