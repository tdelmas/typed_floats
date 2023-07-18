#![doc = include_str!("../README.md")]
//! # Rules
//!
//! Conversions rules for operations are summarized in [`Float`].
//!
//! # Examples
//!
//! Operations will return the strictest type possible.
//!
//! ```
//! use typed_floats::*;
//!
//! let a: StrictlyPositiveFinite = 1.0f64.try_into().unwrap();
//! let b: StrictlyNegativeFinite = (-1.0f64).try_into().unwrap();
//!
//! let c: StrictlyPositive = a - b;
//! let d: NonNaNFinite = a + b;
//!
//! assert_eq!(c.get(), 2.0f64);
//! assert_eq!(d.get(), 0.0f64);   
//! ```
//!
//! ```
//! use typed_floats::*;
//!
//! let a: StrictlyPositiveFinite = 1.0f64.try_into().unwrap();
//! let b: Positive = 0.0f64.try_into().unwrap();
//!
//! let c = a + b;
//!
//! assert_eq!(c, StrictlyPositive::try_from(1.0f64).unwrap());   
//! ```
//!
//! Operations that assign the result to the left operand are only
//! implemented when it is safe to do so.
//!
//! ```
//! use typed_floats::*;
//!
//! let mut a: StrictlyPositive = MAX.try_into().unwrap();
//! let b: StrictlyPositive = MAX.try_into().unwrap();
//!
//! a += b;// Would not compile with StrictlyPositiveFinite
//!
//! assert_eq!(a.get(), INFINITY.get());
//! ```
//!
//!
//! ```
//! use typed_floats::*;
//! use core::num::NonZeroU64;
//!
//! let a = NonZeroU64::new(1).unwrap();
//! let b: StrictlyPositive = a.into(); // no need for try_into
//!
//! assert_eq!(b.get(), 1.0);
//! ```
//!
//!
//!
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::panic)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::indexing_slicing)]

pub trait Hypot<T> {
    type Output;

    fn hypot(self, rhs: T) -> Self::Output;
}

pub trait Min<T> {
    type Output;

    fn min(self, rhs: T) -> Self::Output;
}

pub trait Max<T> {
    type Output;

    fn max(self, rhs: T) -> Self::Output;
}

typed_floats_macros::generate_floats!();

pub trait Float: Eq + Copy + Ord + core::fmt::Debug + core::str::FromStr {
    type Content: Sized
        + Clone
        + Copy
        + PartialOrd
        + PartialEq
        + core::fmt::Debug
        + core::fmt::Display;

    fn new(value: Self::Content) -> Result<Self, InvalidNumber>;

    fn get(&self) -> Self::Content;

    fn is_infinite(&self) -> bool;
    fn is_sign_positive(&self) -> bool;
    fn is_sign_negative(&self) -> bool;
}

pub enum FromStrError {
    ParseFloatError(core::num::ParseFloatError),
    InvalidNumber(InvalidNumber),
}

use core::num::{NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8};
use core::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum InvalidNumber {
    #[error("Number is NaN")]
    NaN,
    #[error("Number is zero")]
    Zero,
    #[error("Number is negative")]
    Negative,
    #[error("Number is positive")]
    Positive,
    #[error("Number is infinite")]
    Infinite,
}

pub const INFINITY: StrictlyPositive = StrictlyPositive(f64::INFINITY);
pub const NEG_INFINITY: StrictlyNegative = StrictlyNegative(f64::NEG_INFINITY);
pub const MAX: StrictlyPositiveFinite = StrictlyPositiveFinite(f64::MAX);
pub const MIN: StrictlyNegativeFinite = StrictlyNegativeFinite(f64::MIN);
pub const MIN_POSITIVE: StrictlyPositiveFinite = StrictlyPositiveFinite(f64::MIN_POSITIVE);
pub const ZERO: PositiveFinite = PositiveFinite(0.0f64);
pub const NEGATIVE_ZERO: NegativeFinite = NegativeFinite(-0.0f64);
