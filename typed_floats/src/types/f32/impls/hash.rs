use crate::*;

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

        bits.hash(state)
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

        bits.hash(state)
    }
}

impl core::hash::Hash for NonZeroNonNaN<f32> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl core::hash::Hash for NonZeroNonNaNFinite<f32> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl core::hash::Hash for Positive<f32> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl core::hash::Hash for Negative<f32> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl core::hash::Hash for PositiveFinite<f32> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl core::hash::Hash for NegativeFinite<f32> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl core::hash::Hash for StrictlyPositive<f32> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl core::hash::Hash for StrictlyNegative<f32> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl core::hash::Hash for StrictlyPositiveFinite<f32> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl core::hash::Hash for StrictlyNegativeFinite<f32> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}
