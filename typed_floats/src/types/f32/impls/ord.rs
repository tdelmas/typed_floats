use crate::*;

impl Ord for NonNaN<f32> {
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

impl Ord for NonZeroNonNaN<f32> {
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

impl Ord for NonNaNFinite<f32> {
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

impl Ord for NonZeroNonNaNFinite<f32> {
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

impl Ord for Positive<f32> {
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

impl Ord for Negative<f32> {
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

impl Ord for PositiveFinite<f32> {
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

impl Ord for NegativeFinite<f32> {
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

impl Ord for StrictlyPositive<f32> {
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

impl Ord for StrictlyNegative<f32> {
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

impl Ord for StrictlyPositiveFinite<f32> {
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

impl Ord for StrictlyNegativeFinite<f32> {
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

impl PartialOrd for NonNaN<f32> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for NonZeroNonNaN<f32> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for NonNaNFinite<f32> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for NonZeroNonNaNFinite<f32> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for Positive<f32> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for Negative<f32> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for PositiveFinite<f32> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for NegativeFinite<f32> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for StrictlyPositive<f32> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for StrictlyNegative<f32> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for StrictlyPositiveFinite<f32> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for StrictlyNegativeFinite<f32> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
