use crate::{
    FromStrError, Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN,
    NonZeroNonNaNFinite, Positive, PositiveFinite, StrictlyNegative, StrictlyNegativeFinite,
    StrictlyPositive, StrictlyPositiveFinite,
};

impl core::str::FromStr for NonNaN<f64> {
    type Err = FromStrError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f64 = s.parse::<f64>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for NonZeroNonNaN<f64> {
    type Err = FromStrError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f64 = s.parse::<f64>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for NonNaNFinite<f64> {
    type Err = FromStrError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f64 = s.parse::<f64>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for NonZeroNonNaNFinite<f64> {
    type Err = FromStrError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f64 = s.parse::<f64>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for Positive<f64> {
    type Err = FromStrError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f64 = s.parse::<f64>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for Negative<f64> {
    type Err = FromStrError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f64 = s.parse::<f64>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for PositiveFinite<f64> {
    type Err = FromStrError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f64 = s.parse::<f64>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for NegativeFinite<f64> {
    type Err = FromStrError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f64 = s.parse::<f64>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for StrictlyPositive<f64> {
    type Err = FromStrError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f64 = s.parse::<f64>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for StrictlyNegative<f64> {
    type Err = FromStrError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f64 = s.parse::<f64>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for StrictlyPositiveFinite<f64> {
    type Err = FromStrError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f64 = s.parse::<f64>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for StrictlyNegativeFinite<f64> {
    type Err = FromStrError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f64 = s.parse::<f64>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}
