use crate::*;

impl core::str::FromStr for NonNaN<f32> {
    type Err = FromStrError;

    #[inline]
    #[must_use]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f32 = s.parse::<f32>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for NonZeroNonNaN<f32> {
    type Err = FromStrError;

    #[inline]
    #[must_use]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f32 = s.parse::<f32>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for NonNaNFinite<f32> {
    type Err = FromStrError;

    #[inline]
    #[must_use]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f32 = s.parse::<f32>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for NonZeroNonNaNFinite<f32> {
    type Err = FromStrError;

    #[inline]
    #[must_use]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f32 = s.parse::<f32>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for Positive<f32> {
    type Err = FromStrError;

    #[inline]
    #[must_use]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f32 = s.parse::<f32>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for Negative<f32> {
    type Err = FromStrError;

    #[inline]
    #[must_use]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f32 = s.parse::<f32>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for PositiveFinite<f32> {
    type Err = FromStrError;

    #[inline]
    #[must_use]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f32 = s.parse::<f32>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for NegativeFinite<f32> {
    type Err = FromStrError;

    #[inline]
    #[must_use]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f32 = s.parse::<f32>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for StrictlyPositive<f32> {
    type Err = FromStrError;

    #[inline]
    #[must_use]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f32 = s.parse::<f32>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for StrictlyNegative<f32> {
    type Err = FromStrError;

    #[inline]
    #[must_use]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f32 = s.parse::<f32>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for StrictlyPositiveFinite<f32> {
    type Err = FromStrError;

    #[inline]
    #[must_use]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f32 = s.parse::<f32>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}

impl core::str::FromStr for StrictlyNegativeFinite<f32> {
    type Err = FromStrError;

    #[inline]
    #[must_use]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f32 = s.parse::<f32>().map_err(FromStrError::ParseFloatError)?;

        Self::try_from(f).map_err(FromStrError::InvalidNumber)
    }
}
