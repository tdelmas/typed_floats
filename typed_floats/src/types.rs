/// An error that can occur when converting from a string into a `TypedFloat`
#[derive(Debug)]
pub enum FromStrError {
    /// The string did not contain a valid float number
    ParseFloatError(core::num::ParseFloatError),
    /// The string contained a valid float number but it didn't fit in the target type
    InvalidNumber(InvalidNumber),
}

impl core::fmt::Display for FromStrError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ParseFloatError(e) => write!(f, "{e}"),
            Self::InvalidNumber(e) => write!(f, "{e}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FromStrError {}

/// An error that can occur when converting into a typed float
#[derive(Debug, Eq, PartialEq)]
pub enum InvalidNumber {
    /// Any variant of `Nan`
    NaN,
    /// `+0.0` or `-0.0`
    Zero,
    /// Any negative number, including `-0.0` and `-inf`
    Negative,
    /// Any positive number, including `+0.0` and `+inf`
    Positive,
    /// `+inf` or `-inf`
    Infinite,
}

impl core::fmt::Display for InvalidNumber {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NaN => write!(f, "Number is NaN"),
            Self::Zero => write!(f, "Number is zero"),
            Self::Negative => write!(f, "Number is negative"),
            Self::Positive => write!(f, "Number is positive"),
            Self::Infinite => write!(f, "Number is infinite"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidNumber {}

/// A non-NaN floating point number
///
/// It satisfies the following constraints:
/// - It is not NaN.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct NonNaN<T = f64>(pub(crate) T);

/// A non-NaN floating point number different from zero
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not zero.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct NonZeroNonNaN<T = f64>(pub(crate) T);

/// A non-NaN finite floating point number different from zero
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not infinite.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct NonNaNFinite<T = f64>(pub(crate) T);

/// A non-NaN finite floating point number different from zero
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not infinite.
/// - It is not zero.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct NonZeroNonNaNFinite<T = f64>(pub(crate) T);

/// A non-NaN positive floating point number
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not negative.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Positive<T = f64>(pub(crate) T);

/// A non-NaN negative floating point number
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not positive.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Negative<T = f64>(pub(crate) T);

/// A non-NaN positive finite floating point number
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not infinite.
/// - It is not negative.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct PositiveFinite<T = f64>(pub(crate) T);

/// A non-NaN negative finite floating point number
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not infinite.
/// - It is not positive.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct NegativeFinite<T = f64>(pub(crate) T);

/// A non-NaN strictly positive floating point number
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not zero.
/// - It is not negative.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct StrictlyPositive<T = f64>(pub(crate) T);

/// A non-NaN strictly negative floating point number
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not zero.
/// - It is not positive.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct StrictlyNegative<T = f64>(pub(crate) T);

/// A non-NaN strictly positive finite floating point number
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not negative.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct StrictlyPositiveFinite<T = f64>(pub(crate) T);

/// A non-NaN strictly negative finite floating point number
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not positive.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct StrictlyNegativeFinite<T = f64>(pub(crate) T);

use crate::traits::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize};

use core::num::{NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8};
use core::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

typed_floats_macros::generate_floats!();
