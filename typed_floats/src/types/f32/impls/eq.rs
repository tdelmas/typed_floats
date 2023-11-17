use crate::{
    Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN, NonZeroNonNaNFinite, Positive,
    PositiveFinite, StrictlyNegative, StrictlyNegativeFinite, StrictlyPositive,
    StrictlyPositiveFinite,
};

// This is safe because we know that both values are not NaN

impl Eq for NonNaN<f32> {}
impl Eq for NonZeroNonNaN<f32> {}
impl Eq for NonNaNFinite<f32> {}
impl Eq for NonZeroNonNaNFinite<f32> {}
impl Eq for Positive<f32> {}
impl Eq for Negative<f32> {}
impl Eq for PositiveFinite<f32> {}
impl Eq for NegativeFinite<f32> {}
impl Eq for StrictlyPositive<f32> {}
impl Eq for StrictlyNegative<f32> {}
impl Eq for StrictlyPositiveFinite<f32> {}
impl Eq for StrictlyNegativeFinite<f32> {}

impl PartialEq for NonNaN<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for NonZeroNonNaN<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for NonNaNFinite<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for NonZeroNonNaNFinite<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for Positive<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for Negative<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for PositiveFinite<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for NegativeFinite<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for StrictlyPositive<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for StrictlyNegative<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for StrictlyPositiveFinite<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for StrictlyNegativeFinite<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
