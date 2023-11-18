use crate::{
    FromStrError, Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN,
    NonZeroNonNaNFinite, Positive, PositiveFinite, StrictlyNegative, StrictlyNegativeFinite,
    StrictlyPositive, StrictlyPositiveFinite,
};

macro_rules! impl_from_str {
    ($type:ident) => {
        impl core::str::FromStr for $type<f32> {
            type Err = FromStrError;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let f: f32 = s.parse::<f32>().map_err(FromStrError::ParseFloatError)?;

                Self::try_from(f).map_err(FromStrError::InvalidNumber)
            }
        }
    };
}

impl_from_str!(NonNaN);
impl_from_str!(NonZeroNonNaN);
impl_from_str!(NonNaNFinite);
impl_from_str!(NonZeroNonNaNFinite);
impl_from_str!(Positive);
impl_from_str!(Negative);
impl_from_str!(PositiveFinite);
impl_from_str!(NegativeFinite);
impl_from_str!(StrictlyPositive);
impl_from_str!(StrictlyNegative);
impl_from_str!(StrictlyPositiveFinite);
impl_from_str!(StrictlyNegativeFinite);
