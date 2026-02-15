use crate::container::{Contained, Container};
use const_fn::const_fn;

/// An error that can occur when converting from a string into a typed float
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

#[cfg(feature = "serde")]
use serde::Serialize;

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
pub struct NonNaN<T: Contained = f64>(Container<T>);

/// A non-NaN floating point number different from zero
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not zero.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct NonZeroNonNaN<T: Contained = f64>(Container<T>);

/// A non-NaN finite floating point number
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not infinite.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct NonNaNFinite<T: Contained = f64>(Container<T>);

/// A non-NaN finite floating point number different from zero
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not infinite.
/// - It is not zero.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct NonZeroNonNaNFinite<T: Contained = f64>(Container<T>);

/// A non-NaN positive floating point number
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not negative.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Positive<T: Contained = f64>(Container<T>);

/// A non-NaN negative floating point number
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not positive.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Negative<T: Contained = f64>(Container<T>);

/// A non-NaN positive finite floating point number
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not infinite.
/// - It is not negative.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct PositiveFinite<T: Contained = f64>(Container<T>);

/// A non-NaN negative finite floating point number
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not infinite.
/// - It is not positive.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct NegativeFinite<T: Contained = f64>(Container<T>);

/// A non-NaN strictly positive floating point number
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not zero.
/// - It is not negative.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct StrictlyPositive<T: Contained = f64>(Container<T>);

/// A non-NaN strictly negative floating point number
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not zero.
/// - It is not positive.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct StrictlyNegative<T: Contained = f64>(Container<T>);

/// A non-NaN strictly positive finite floating point number
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not negative.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct StrictlyPositiveFinite<T: Contained = f64>(Container<T>);

/// A non-NaN strictly negative finite floating point number
///
/// It satisfies the following constraints:
/// - It is not NaN.
/// - It is not positive.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct StrictlyNegativeFinite<T: Contained = f64>(Container<T>);

use crate::traits::{Max, Min};

#[cfg(any(feature = "std", feature = "libm"))]
use crate::traits::{Atan2, Copysign, DivEuclid, Hypot, Powf};

#[rustversion::since(1.85)]
use crate::traits::Midpoint;

#[cfg(all(feature = "libm", not(feature = "std")))]
#[allow(unused_imports)]
use num_traits::Float;

mod accept;
mod f32;
mod f64;
mod impls;

typed_floats_macros::generate_floats!();
