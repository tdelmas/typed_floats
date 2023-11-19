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
                if *self < *other {
                    core::cmp::Ordering::Less
                } else if *self == *other {
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
    };
}

impl_ord!(NonNaN);
impl_ord!(NonZeroNonNaN);
impl_ord!(NonNaNFinite);
impl_ord!(NonZeroNonNaNFinite);
impl_ord!(Positive);
impl_ord!(Negative);
impl_ord!(PositiveFinite);
impl_ord!(NegativeFinite);
impl_ord!(StrictlyPositive);
impl_ord!(StrictlyNegative);
impl_ord!(StrictlyPositiveFinite);
impl_ord!(StrictlyNegativeFinite);
