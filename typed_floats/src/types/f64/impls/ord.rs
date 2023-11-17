#![allow(clippy::comparison_chain)]

use crate::{
    Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN, NonZeroNonNaNFinite, Positive,
    PositiveFinite, StrictlyNegative, StrictlyNegativeFinite, StrictlyPositive,
    StrictlyPositiveFinite,
};

impl Ord for NonNaN<f64> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if *self < *other {
            core::cmp::Ordering::Less
        } else if *self == *other {
            core::cmp::Ordering::Equal
        } else {
            core::cmp::Ordering::Greater
        }
    }
}

impl Ord for NonZeroNonNaN<f64> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if *self < *other {
            core::cmp::Ordering::Less
        } else if *self == *other {
            core::cmp::Ordering::Equal
        } else {
            core::cmp::Ordering::Greater
        }
    }
}

impl Ord for NonNaNFinite<f64> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if *self < *other {
            core::cmp::Ordering::Less
        } else if *self == *other {
            core::cmp::Ordering::Equal
        } else {
            core::cmp::Ordering::Greater
        }
    }
}

impl Ord for NonZeroNonNaNFinite<f64> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if *self < *other {
            core::cmp::Ordering::Less
        } else if *self == *other {
            core::cmp::Ordering::Equal
        } else {
            core::cmp::Ordering::Greater
        }
    }
}

impl Ord for Positive<f64> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if *self < *other {
            core::cmp::Ordering::Less
        } else if *self == *other {
            core::cmp::Ordering::Equal
        } else {
            core::cmp::Ordering::Greater
        }
    }
}

impl Ord for Negative<f64> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if *self < *other {
            core::cmp::Ordering::Less
        } else if *self == *other {
            core::cmp::Ordering::Equal
        } else {
            core::cmp::Ordering::Greater
        }
    }
}

impl Ord for PositiveFinite<f64> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if *self < *other {
            core::cmp::Ordering::Less
        } else if *self == *other {
            core::cmp::Ordering::Equal
        } else {
            core::cmp::Ordering::Greater
        }
    }
}

impl Ord for NegativeFinite<f64> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if *self < *other {
            core::cmp::Ordering::Less
        } else if *self == *other {
            core::cmp::Ordering::Equal
        } else {
            core::cmp::Ordering::Greater
        }
    }
}

impl Ord for StrictlyPositive<f64> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if *self < *other {
            core::cmp::Ordering::Less
        } else if *self == *other {
            core::cmp::Ordering::Equal
        } else {
            core::cmp::Ordering::Greater
        }
    }
}

impl Ord for StrictlyNegative<f64> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if *self < *other {
            core::cmp::Ordering::Less
        } else if *self == *other {
            core::cmp::Ordering::Equal
        } else {
            core::cmp::Ordering::Greater
        }
    }
}

impl Ord for StrictlyPositiveFinite<f64> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if *self < *other {
            core::cmp::Ordering::Less
        } else if *self == *other {
            core::cmp::Ordering::Equal
        } else {
            core::cmp::Ordering::Greater
        }
    }
}

impl Ord for StrictlyNegativeFinite<f64> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if *self < *other {
            core::cmp::Ordering::Less
        } else if *self == *other {
            core::cmp::Ordering::Equal
        } else {
            core::cmp::Ordering::Greater
        }
    }
}

impl PartialOrd for NonNaN<f64> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for NonZeroNonNaN<f64> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for NonNaNFinite<f64> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for NonZeroNonNaNFinite<f64> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for Positive<f64> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for Negative<f64> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for PositiveFinite<f64> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for NegativeFinite<f64> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for StrictlyPositive {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for StrictlyNegative {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for StrictlyPositiveFinite {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for StrictlyNegativeFinite {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
