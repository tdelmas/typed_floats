use crate::{
    Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN, NonZeroNonNaNFinite, Positive,
    PositiveFinite, StrictlyNegative, StrictlyNegativeFinite, StrictlyPositive,
    StrictlyPositiveFinite,
};

// This is safe because we know that both values are not NaN

macro_rules! impl_eq {
    (  $type:ident  ) => {
        impl Eq for $type<f32> {}

        impl PartialEq for $type<f32> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl PartialEq<$type<f32>> for f32 {
            #[inline]
            fn eq(&self, other: &$type<f32>) -> bool {
                self == &other.0
            }
        }

        impl PartialEq<f32> for $type<f32> {
            #[inline]
            fn eq(&self, other: &f32) -> bool {
                &self.0 == other
            }
        }
    };
}

impl_eq!(NonNaN);
impl_eq!(NonZeroNonNaN);
impl_eq!(NonNaNFinite);
impl_eq!(NonZeroNonNaNFinite);
impl_eq!(Positive);
impl_eq!(Negative);
impl_eq!(PositiveFinite);
impl_eq!(NegativeFinite);
impl_eq!(StrictlyPositive);
impl_eq!(StrictlyNegative);
impl_eq!(StrictlyPositiveFinite);
impl_eq!(StrictlyNegativeFinite);
