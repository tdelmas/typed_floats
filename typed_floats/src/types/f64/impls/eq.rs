use crate::*;

// This is safe because we know that both values are not NaN

impl Eq for NonNaN<f64> {}
impl Eq for NonZeroNonNaN<f64> {}
impl Eq for NonNaNFinite<f64> {}
impl Eq for NonZeroNonNaNFinite<f64> {}
impl Eq for Positive<f64> {}
impl Eq for Negative<f64> {}
impl Eq for PositiveFinite<f64> {}
impl Eq for NegativeFinite<f64> {}
impl Eq for StrictlyPositive<f64> {}
impl Eq for StrictlyNegative<f64> {}
impl Eq for StrictlyPositiveFinite<f64> {}
impl Eq for StrictlyNegativeFinite<f64> {}

impl PartialEq for NonNaN<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for NonZeroNonNaN<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for NonNaNFinite<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for NonZeroNonNaNFinite<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for Positive<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for Negative<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for PositiveFinite<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for NegativeFinite<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for StrictlyPositive<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for StrictlyNegative<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for StrictlyPositiveFinite<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for StrictlyNegativeFinite<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
