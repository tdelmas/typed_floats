use serde::{Deserialize, Deserializer};

use crate::types::*;

impl<'de> Deserialize<'de> for NonNaN<f64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f64 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for NonNaN<f32> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f32 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for NonZeroNonNaN<f64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f64 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for NonZeroNonNaN<f32> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f32 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for NonNaNFinite<f64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f64 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for NonNaNFinite<f32> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f32 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for NonZeroNonNaNFinite<f64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f64 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for NonZeroNonNaNFinite<f32> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f32 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for Positive<f64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f64 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for Positive<f32> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f32 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for Negative<f64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f64 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for Negative<f32> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f32 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for PositiveFinite<f64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f64 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for PositiveFinite<f32> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f32 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for NegativeFinite<f64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f64 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for NegativeFinite<f32> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f32 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for StrictlyPositive<f64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f64 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for StrictlyPositive<f32> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f32 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for StrictlyNegative<f64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f64 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for StrictlyNegative<f32> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f32 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for StrictlyPositiveFinite<f64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f64 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for StrictlyPositiveFinite<f32> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f32 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for StrictlyNegativeFinite<f64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f64 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for StrictlyNegativeFinite<f32> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: f32 = Deserialize::deserialize(deserializer)?;

        val.try_into().map_err(serde::de::Error::custom)
    }
}
