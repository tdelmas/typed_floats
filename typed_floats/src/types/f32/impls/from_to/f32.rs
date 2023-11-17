use crate::{
    InvalidNumber, Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN,
    NonZeroNonNaNFinite, Positive, PositiveFinite, StrictlyNegative, StrictlyNegativeFinite,
    StrictlyPositive, StrictlyPositiveFinite,
};

impl From<NonNaN<Self>> for f32 {
    #[inline]
    #[must_use]
    fn from(value: NonNaN<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f32> for NonNaN<f32> {
    type Error = InvalidNumber;

    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<NonZeroNonNaN<Self>> for f32 {
    #[inline]
    #[must_use]
    fn from(value: NonZeroNonNaN<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f32> for NonZeroNonNaN<f32> {
    type Error = InvalidNumber;

    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<NonNaNFinite<Self>> for f32 {
    #[inline]
    #[must_use]
    fn from(value: NonNaNFinite<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f32> for NonNaNFinite<f32> {
    type Error = InvalidNumber;

    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<NonZeroNonNaNFinite<Self>> for f32 {
    #[inline]
    #[must_use]
    fn from(value: NonZeroNonNaNFinite<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f32> for NonZeroNonNaNFinite<f32> {
    type Error = InvalidNumber;

    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Positive<Self>> for f32 {
    #[inline]
    #[must_use]
    fn from(value: Positive<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f32> for Positive<f32> {
    type Error = InvalidNumber;

    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Negative<Self>> for f32 {
    #[inline]
    #[must_use]
    fn from(value: Negative<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f32> for Negative<f32> {
    type Error = InvalidNumber;

    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PositiveFinite<Self>> for f32 {
    #[inline]
    #[must_use]
    fn from(value: PositiveFinite<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f32> for PositiveFinite<f32> {
    type Error = InvalidNumber;

    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<NegativeFinite<Self>> for f32 {
    #[inline]
    #[must_use]
    fn from(value: NegativeFinite<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f32> for NegativeFinite<f32> {
    type Error = InvalidNumber;

    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<StrictlyPositive<Self>> for f32 {
    #[inline]
    #[must_use]
    fn from(value: StrictlyPositive<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f32> for StrictlyPositive<f32> {
    type Error = InvalidNumber;

    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<StrictlyNegative<Self>> for f32 {
    #[inline]
    #[must_use]
    fn from(value: StrictlyNegative<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f32> for StrictlyNegative<f32> {
    type Error = InvalidNumber;

    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<StrictlyPositiveFinite<Self>> for f32 {
    #[inline]
    #[must_use]
    fn from(value: StrictlyPositiveFinite<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f32> for StrictlyPositiveFinite<f32> {
    type Error = InvalidNumber;

    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<StrictlyNegativeFinite<Self>> for f32 {
    #[inline]
    #[must_use]
    fn from(value: StrictlyNegativeFinite<Self>) -> Self {
        value.0
    }
}

impl TryFrom<f32> for StrictlyNegativeFinite<f32> {
    type Error = InvalidNumber;

    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
