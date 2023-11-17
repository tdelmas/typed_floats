use crate::*;

impl From<NonNaN<Self>> for f64 {
    #[inline]
    #[must_use]
    fn from(value: NonNaN<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f64> for NonNaN<f64> {
    type Error = InvalidNumber;

    #[inline]
    #[must_use]
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<NonZeroNonNaN<Self>> for f64 {
    #[inline]
    #[must_use]
    fn from(value: NonZeroNonNaN<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f64> for NonZeroNonNaN<f64> {
    type Error = InvalidNumber;

    #[inline]
    #[must_use]
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<NonNaNFinite<Self>> for f64 {
    #[inline]
    #[must_use]
    fn from(value: NonNaNFinite<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f64> for NonNaNFinite<f64> {
    type Error = InvalidNumber;

    #[inline]
    #[must_use]
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<NonZeroNonNaNFinite<Self>> for f64 {
    #[inline]
    #[must_use]
    fn from(value: NonZeroNonNaNFinite<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f64> for NonZeroNonNaNFinite<f64> {
    type Error = InvalidNumber;

    #[inline]
    #[must_use]
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Positive<Self>> for f64 {
    #[inline]
    #[must_use]
    fn from(value: Positive<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f64> for Positive<f64> {
    type Error = InvalidNumber;

    #[inline]
    #[must_use]
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Negative<Self>> for f64 {
    #[inline]
    #[must_use]
    fn from(value: Negative<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f64> for Negative<f64> {
    type Error = InvalidNumber;

    #[inline]
    #[must_use]
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PositiveFinite<Self>> for f64 {
    #[inline]
    #[must_use]
    fn from(value: PositiveFinite<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f64> for PositiveFinite<f64> {
    type Error = InvalidNumber;

    #[inline]
    #[must_use]
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<NegativeFinite<Self>> for f64 {
    #[inline]
    #[must_use]
    fn from(value: NegativeFinite<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f64> for NegativeFinite<f64> {
    type Error = InvalidNumber;

    #[inline]
    #[must_use]
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<StrictlyPositive<Self>> for f64 {
    #[inline]
    #[must_use]
    fn from(value: StrictlyPositive<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f64> for StrictlyPositive<f64> {
    type Error = InvalidNumber;

    #[inline]
    #[must_use]
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<StrictlyNegative<Self>> for f64 {
    #[inline]
    #[must_use]
    fn from(value: StrictlyNegative<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f64> for StrictlyNegative<f64> {
    type Error = InvalidNumber;

    #[inline]
    #[must_use]
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<StrictlyPositiveFinite<Self>> for f64 {
    #[inline]
    #[must_use]
    fn from(value: StrictlyPositiveFinite<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f64> for StrictlyPositiveFinite<f64> {
    type Error = InvalidNumber;

    #[inline]
    #[must_use]
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<StrictlyNegativeFinite<Self>> for f64 {
    #[inline]
    #[must_use]
    fn from(value: StrictlyNegativeFinite<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f64> for StrictlyNegativeFinite<f64> {
    type Error = InvalidNumber;

    #[inline]
    #[must_use]
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
