use crate::{
    Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN, NonZeroNonNaNFinite, Positive,
    PositiveFinite, StrictlyNegative, StrictlyNegativeFinite, StrictlyPositive,
    StrictlyPositiveFinite,
};

macro_rules! impl_display {
    ($type:ident) => {
        impl core::fmt::Display for $type {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

impl_display!(NonNaN);
impl_display!(NonZeroNonNaN);
impl_display!(NonNaNFinite);
impl_display!(NonZeroNonNaNFinite);
impl_display!(Positive);
impl_display!(Negative);
impl_display!(PositiveFinite);
impl_display!(NegativeFinite);
impl_display!(StrictlyPositive);
impl_display!(StrictlyNegative);
impl_display!(StrictlyPositiveFinite);
impl_display!(StrictlyNegativeFinite);
