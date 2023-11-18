use crate::{
    InvalidNumber, Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN,
    NonZeroNonNaNFinite, Positive, PositiveFinite, StrictlyNegative, StrictlyNegativeFinite,
    StrictlyPositive, StrictlyPositiveFinite,
};

macro_rules! impl_from {
    ($type:ident) => {
        impl From<$type<Self>> for f64 {
            #[inline]
            #[must_use]
            fn from(value: $type<Self>) -> Self {
                value.0
            }
        }

        impl TryFrom<f64> for $type<f64> {
            type Error = InvalidNumber;

            #[inline]
            fn try_from(value: f64) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

impl_from!(NonNaN);
impl_from!(NonZeroNonNaN);
impl_from!(NonNaNFinite);
impl_from!(NonZeroNonNaNFinite);
impl_from!(Positive);
impl_from!(Negative);
impl_from!(PositiveFinite);
impl_from!(NegativeFinite);
impl_from!(StrictlyPositive);
impl_from!(StrictlyNegative);
impl_from!(StrictlyPositiveFinite);
impl_from!(StrictlyNegativeFinite);
