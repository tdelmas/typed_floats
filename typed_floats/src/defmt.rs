use defmt::{Format, Formatter, write};

use crate::types::{
    Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN, NonZeroNonNaNFinite, Positive,
    PositiveFinite, StrictlyNegative, StrictlyNegativeFinite, StrictlyPositive,
    StrictlyPositiveFinite,
};

macro_rules! impl_format {
    ($type:ident) => {
        #[cfg(feature = "f64")]
        impl Format for $type<f64> {
            fn format(&self, fmt: Formatter<'_>) {
                write!(fmt, "{=f64}", self.get());
            }
        }

        #[cfg(feature = "f32")]
        impl Format for $type<f32> {
            fn format(&self, fmt: Formatter<'_>) {
                write!(fmt, "{=f32}", self.get());
            }
        }
    };
}

impl_format!(NonNaN);
impl_format!(NonZeroNonNaN);
impl_format!(NonNaNFinite);
impl_format!(NonZeroNonNaNFinite);
impl_format!(Positive);
impl_format!(Negative);
impl_format!(PositiveFinite);
impl_format!(NegativeFinite);
impl_format!(StrictlyPositive);
impl_format!(StrictlyNegative);
impl_format!(StrictlyPositiveFinite);
impl_format!(StrictlyNegativeFinite);
