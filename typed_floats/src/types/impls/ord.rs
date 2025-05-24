#![allow(clippy::comparison_chain)]

use crate::{
    Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN, NonZeroNonNaNFinite, Positive,
    PositiveFinite, StrictlyNegative, StrictlyNegativeFinite, StrictlyPositive,
    StrictlyPositiveFinite,
};

macro_rules! impl_ord {
    ($type:ident) => {
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
    };
}

macro_rules! impl_fast_ord {
    ($type:ident) => {
        impl Ord for $type<f32> {
            #[inline]
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.get().to_bits().cmp(&other.get().to_bits())
            }
        }

        impl Ord for $type<f64> {
            #[inline]
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.get().to_bits().cmp(&other.get().to_bits())
            }
        }
    };
}

macro_rules! impl_fast_inv_ord {
    ($type:ident) => {
        impl Ord for $type<f32> {
            #[inline]
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                other.get().to_bits().cmp(&self.get().to_bits())
            }
        }

        impl Ord for $type<f64> {
            #[inline]
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                other.get().to_bits().cmp(&self.get().to_bits())
            }
        }
    };
}

macro_rules! impl_partial_ord {
    ($type:ident) => {
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
    };
}

impl_ord!(NonNaN);
impl_ord!(NonZeroNonNaN);
impl_ord!(NonNaNFinite);
impl_ord!(NonZeroNonNaNFinite);
impl_fast_ord!(Positive);
impl_fast_inv_ord!(Negative);
impl_fast_ord!(PositiveFinite);
impl_fast_inv_ord!(NegativeFinite);
impl_fast_ord!(StrictlyPositive);
impl_fast_inv_ord!(StrictlyNegative);
impl_fast_ord!(StrictlyPositiveFinite);
impl_fast_inv_ord!(StrictlyNegativeFinite);

impl_partial_ord!(NonNaN);
impl_partial_ord!(NonZeroNonNaN);
impl_partial_ord!(NonNaNFinite);
impl_partial_ord!(NonZeroNonNaNFinite);
impl_partial_ord!(Positive);
impl_partial_ord!(Negative);
impl_partial_ord!(PositiveFinite);
impl_partial_ord!(NegativeFinite);
impl_partial_ord!(StrictlyPositive);
impl_partial_ord!(StrictlyNegative);
impl_partial_ord!(StrictlyPositiveFinite);
impl_partial_ord!(StrictlyNegativeFinite);
