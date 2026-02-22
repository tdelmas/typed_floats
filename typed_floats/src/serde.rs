use serde::{Deserialize, Deserializer};

use crate::types::{
    Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN, NonZeroNonNaNFinite, Positive,
    PositiveFinite, StrictlyNegative, StrictlyNegativeFinite, StrictlyPositive,
    StrictlyPositiveFinite,
};

macro_rules! impl_deserialize {
    ($type:ident) => {
        #[cfg(feature = "f64")]
        impl<'de> Deserialize<'de> for $type<f64> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let val: f64 = Deserialize::deserialize(deserializer)?;

                val.try_into().map_err(serde::de::Error::custom)
            }
        }

        #[cfg(feature = "f32")]
        impl<'de> Deserialize<'de> for $type<f32> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let val: f32 = Deserialize::deserialize(deserializer)?;

                val.try_into().map_err(serde::de::Error::custom)
            }
        }
    };
}

impl_deserialize!(NonNaN);
impl_deserialize!(NonZeroNonNaN);
impl_deserialize!(NonNaNFinite);
impl_deserialize!(NonZeroNonNaNFinite);
impl_deserialize!(Positive);
impl_deserialize!(Negative);
impl_deserialize!(PositiveFinite);
impl_deserialize!(NegativeFinite);
impl_deserialize!(StrictlyPositive);
impl_deserialize!(StrictlyNegative);
impl_deserialize!(StrictlyPositiveFinite);
impl_deserialize!(StrictlyNegativeFinite);
