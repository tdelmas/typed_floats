#![doc = include_str!("../README.truncated.md")]
//! # Rules
//!
//! Conversions rules for operations are summarized in [`TypedFloat`].
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
//! assert_eq!(c, 2.0);
//! assert_eq!(d, 0.0);   
//! ```
//!
//! ```
//! use typed_floats::*;
//!
//! let a: StrictlyPositiveFinite = 1.0f64.try_into().unwrap();
//! let b: Positive = 0.0f64.try_into().unwrap();
//!
//! let c: StrictlyPositive = a + b;
//!
//! assert_eq!(c, 1.0);   
//! ```
//!
//! Operations that assign the result to the left operand are only
//! implemented when it is safe to do so:
//!
//! ```
//! use typed_floats::*;
//!
//! let mut a: StrictlyPositive = f64::MAX.try_into().unwrap();
//! let b: StrictlyPositive = f64::MAX.try_into().unwrap();
//!
//! a += b;// Would not compile with StrictlyPositiveFinite
//!
//! assert_eq!(a, f64::INFINITY);
//! ```
//!
//! ```compile_fail
//! use typed_floats::*;
//!
//! let mut a: StrictlyPositiveFinite = f64::MAX.try_into().unwrap();
//! let b: StrictlyPositiveFinite = f64::MAX.try_into().unwrap();
//!
//! a += b;// Does not compile
//!
//! assert_eq!(a, f64::INFINITY);
//! ```
//!
//! Conversions from non-zero integers are available:
//!
//! ```
//! use typed_floats::*;
//! use core::num::NonZeroU64;
//!
//! let a = NonZeroU64::new(1).unwrap();
//! let b: StrictlyPositive = a.into(); // no need for try_into
//!
//! assert_eq!(b, 1.0);
//! ```
//!
//! Also, comparaison between types is available:
//!
//! ```
//! use typed_floats::*;
//!
//! let a: f64 = 1.0;
//! let b: StrictlyPositive = 1.0.try_into().unwrap();
//! let c: StrictlyPositiveFinite = 1.0.try_into().unwrap();
//!
//! assert_eq!(a, b);
//! assert_eq!(b, a);
//! assert_eq!(b, c);
//! ```
//!
//! To return early in a function:
//! ```
//! use typed_floats::*;
//!
//! fn early_return(a:f64,b:f64) -> Result<PositiveFinite,InvalidNumber> {
//!   let a: StrictlyPositiveFinite = a.try_into()?;
//!   let b: StrictlyPositiveFinite = b.try_into()?;
//!
//!   Ok(a % b)
//! }
//!
//! assert_eq!(early_return(-1.0,2.0), Err(InvalidNumber::Negative));
//! assert_eq!(early_return(1.0,2.0).unwrap().get(), 1.0);
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
#![warn(missing_docs)]

/// This macros assert that two values are close to each other.
///
/// # Examples
///
/// ```
/// # use typed_floats::*;
/// assert_relative_eq!(1.0_f64, 1.0);
/// assert_relative_eq!(1.0_f64, 1.000000001, 1e-7);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_relative_eq!(2.0_f64, 1.0);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_relative_eq!(1.0_f64, 1.000001, 1e-7);
/// ```
#[macro_export]
macro_rules! assert_relative_eq {
    ($left:expr, $right:expr) => {{
        let epsilon = 1e-7;

        assert_relative_eq!($left, $right, epsilon);
    }};
    ($left:expr, $right:expr, $epsilon:expr) => {{
        let left_val = $left;
        let right_val = $right;
        let diff = (left_val - right_val).abs();

        assert!(
            diff <= $epsilon,
            "assertion failed: `(left ~= right)` \
             (left: `{:?}`, right: `{:?}`, (left - right): `{:?}`, epsilon: `{:?}`)",
            left_val,
            right_val,
            diff,
            $epsilon
        );
    }};
}

/// This macros assert that the value is NaN.
///
/// # Examples
///
/// ```
/// # use typed_floats::*;
/// assert_is_nan!(0.0_f64 / 0.0);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_is_nan!(2.0_f64);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_is_nan!(core::f64::INFINITY);
/// ```
#[macro_export]
macro_rules! assert_is_nan {
    ($left:expr) => {{
        let left_val = $left;

        assert!(
            left_val.is_nan(),
            "assertion failed: `(value is NaN)` \
             (value: `{:?}`)",
            left_val,
        );
    }};
}

/// This macros assert that the value is positive zero.
///
/// # Examples
///
/// ```
/// # use typed_floats::*;
/// assert_is_positive_zero!(0.0_f64);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_is_positive_zero!(-0.0_f64);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_is_positive_zero!(core::f64::INFINITY);
/// ```
#[macro_export]
macro_rules! assert_is_positive_zero {
    ($left:expr) => {{
        let val = $left;

        assert!(
            val == 0.0 && val.is_sign_positive(),
            "assertion failed: `(value is positive zero)` \
             (value: `{:?}`)",
            val,
        );
    }};
}

/// This macros assert that the value is negative zero.
///
/// # Examples
///
/// ```
/// # use typed_floats::*;
/// assert_is_negative_zero!(-0.0_f64);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_is_negative_zero!(0.0_f64);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_is_negative_zero!(core::f64::NEG_INFINITY);
/// ```
#[macro_export]
macro_rules! assert_is_negative_zero {
    ($left:expr) => {{
        let val = $left;

        assert!(
            val == 0.0 && val.is_sign_negative(),
            "assertion failed: `(value is negative zero)` \
             (value: `{:?}`)",
            val,
        );
    }};
}

/// This trait is used to specify the return type of the [`Hypot::hypot()`] function.
pub trait Hypot<T> {
    /// The resulting type after applying [`Hypot::hypot()`].
    type Output;

    /// Compute the distance between the origin and a point (`x`, `y`) on the
    /// Euclidean plane. Equivalently, compute the length of the hypotenuse of a
    /// right-angle triangle with other sides having length `x.abs()` and
    /// `y.abs()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    /// let x: NonNaN = 3.0.try_into().unwrap();
    /// let y: NonNaN = 4.0.try_into().unwrap();
    ///
    /// assert_eq!(x.hypot(y), 5.0);
    /// ```
    ///
    /// See [`f64::hypot()`] for more details.
    fn hypot(self, rhs: T) -> Self::Output;
}

/// This trait is used to specify the return type of the [`Min::min()`] function.
pub trait Min<T> {
    /// The resulting type after applying [`Min::min()`].
    type Output;

    /// Returns the minimum of the two numbers.
    ///
    /// This follows the IEEE 754-2008 semantics for minNum.
    /// This also matches the behavior of libm’s fmin.
    ///
    /// The min of `+0.0` and `-0.0` may return either operand.
    /// <https://llvm.org/docs/LangRef.html#llvm-minnum-intrinsic>
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    /// let x: NonNaN = 3.0.try_into().unwrap();
    /// let y: NonNaN = 4.0.try_into().unwrap();
    ///
    /// assert_eq!(Min::min(x, y), 3.0);
    /// ```
    ///
    /// See [`f64::min()`] for more details.
    fn min(self, rhs: T) -> Self::Output;
}

/// This trait is used to specify the return type of the [`Max::max()`] function.
pub trait Max<T> {
    /// The resulting type after applying [`Max::max()`].
    type Output;

    /// Returns the maximum of the two numbers.
    ///
    /// This follows the IEEE 754-2008 semantics for maxNum;
    /// This also matches the behavior of libm’s fmax.
    ///
    /// The max of `+0.0` and `-0.0` may return either operand.
    /// <https://llvm.org/docs/LangRef.html#llvm-maxnum-intrinsic>
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    /// let x: NonNaN = 3.0.try_into().unwrap();
    /// let y: NonNaN = 4.0.try_into().unwrap();
    ///
    /// assert_eq!(Max::max(x, y), 4.0);
    /// ```
    ///
    /// See [`f64::max()`] for more details.
    fn max(self, rhs: T) -> Self::Output;
}

/// This trait is used to specify the return type of the [`Copysign::copysign()`] function.
pub trait Copysign<T> {
    /// The resulting type after applying [`Copysign::copysign()`].
    type Output;

    /// Returns a number composed of the magnitude of self and the sign of sign.
    /// Equal to self if the sign of self and sign are the same, otherwise equal to -self.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    /// let x: NonNaN = 3.5.try_into().unwrap();
    /// let y: NonNaN = (-3.5).try_into().unwrap();
    /// let a: NonNaN = 1.0.try_into().unwrap();
    /// let b: NonNaN = 0.0.try_into().unwrap();
    /// let c: NonNaN = (-0.0).try_into().unwrap();
    /// let d: NonNaN = (-1.0).try_into().unwrap();
    ///
    /// assert_eq!(x.copysign(a), 3.5);
    /// assert_eq!(x.copysign(b), 3.5);
    /// assert_eq!(x.copysign(c), -3.5);
    /// assert_eq!(x.copysign(d), -3.5);
    ///
    /// assert_eq!(y.copysign(a), 3.5);
    /// assert_eq!(y.copysign(b), 3.5);
    /// assert_eq!(y.copysign(c), -3.5);
    /// assert_eq!(y.copysign(d), -3.5);
    /// ```
    ///
    /// See [`f64::copysign()`] for more details.
    fn copysign(self, rhs: T) -> Self::Output;
}

/// This trait is used to specify the return type of the [`DivEuclid::div_euclid()`] function.
pub trait DivEuclid<T> {
    /// The resulting type after applying [`DivEuclid::div_euclid()`].
    type Output;

    /// Calculates Euclidean division, the matching method for `rem_euclid`.
    /// Equal to self if the sign of self and sign are the same, otherwise equal to -self.
    ///
    /// This computes the integer `n` such that
    /// `self = n * rhs + self.rem_euclid(rhs)`.
    /// In other words, the result is `self / rhs` rounded to the integer `n`
    /// such that `self >= n * rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    /// let a: NonNaN = 7.0.try_into().unwrap();
    /// let b: NonNaN = 4.0.try_into().unwrap();
    ///
    /// assert_eq!(a.div_euclid(b), 1.0);
    /// assert_eq!((-a).div_euclid(b), -2.0);
    /// assert_eq!(a.div_euclid(-b), -1.0);
    /// assert_eq!((-a).div_euclid(-b), 2.0);
    /// ```
    ///
    /// See [`f64::div_euclid()`] for more details.
    fn div_euclid(self, rhs: T) -> Self::Output;
}

typed_floats_macros::generate_docs!(
    pub trait TypedFloat {}
);

/// An error that can occur when converting from a string into a `TypedFloat`
#[derive(Error, Debug)]
pub enum FromStrError {
    /// The string did not contain a valid float number
    #[error("{0:?}")]
    ParseFloatError(core::num::ParseFloatError),
    /// The string contained a valid float number but it didn't fit in the target type
    #[error("{0:?}")]
    InvalidNumber(InvalidNumber),
}

use core::num::{NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8};
use core::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

use thiserror::Error;

/// An error that can occur when converting into a typed float
#[derive(Error, Debug, Eq, PartialEq)]
pub enum InvalidNumber {
    /// Any variant of `Nan`
    #[error("Number is NaN")]
    NaN,
    /// `+0.0` or `-0.0`
    #[error("Number is zero")]
    Zero,
    /// Any negative number, including `-0.0` and `-inf`
    #[error("Number is negative")]
    Negative,
    /// Any positive number, including `+0.0` and `+inf`
    #[error("Number is positive")]
    Positive,
    /// `+inf` or `-inf`
    #[error("Number is infinite")]
    Infinite,
}

typed_floats_macros::generate_floats!();

macro_rules! add_const {
    ($name:ident, $type:ident, $typed:ident, $value:expr, $doc:expr) => {
        #[doc = $doc]
        pub const $name: crate::$typed<$type> = crate::$typed::<$type>($value);
    };
    ($name:ident, $type:ident, $typed:ident, $doc:expr) => {
        add_const!($name, $type, $typed, core::$type::consts::$name, $doc);
    };
}

/// This module contains constants from [`core::f64`], casted to the corresponding type
pub mod tf64 {
    /// Equivalent to `NonNaN<f64>`
    pub type NonNaN = crate::NonNaN<f64>;

    /// Equivalent to `NonNaNFinite<f64>`
    pub type NonNaNFinite = crate::NonNaNFinite<f64>;

    /// Equivalent to `NonZeroNonNaN<f64>`
    pub type NonZero = crate::NonZeroNonNaN<f64>;

    /// Equivalent to `NonZeroNonNaNFinite<f64>`
    pub type NonZeroFinite = crate::NonZeroNonNaNFinite<f64>;

    /// Equivalent to `StrictlyPositive<f64>`
    pub type StrictlyPositive = crate::StrictlyPositive<f64>;

    /// Equivalent to `StrictlyNegative<f64>`
    pub type StrictlyNegative = crate::StrictlyNegative<f64>;

    /// Equivalent to `Positive<f64>`
    pub type Positive = crate::Positive<f64>;

    /// Equivalent to `Negative<f64>`
    pub type Negative = crate::Negative<f64>;

    /// Equivalent to `StrictlyPositiveFinite<f64>`
    pub type StrictlyPositiveFinite = crate::StrictlyPositiveFinite<f64>;

    /// Equivalent to `StrictlyNegativeFinite<f64>`
    pub type StrictlyNegativeFinite = crate::StrictlyNegativeFinite<f64>;

    /// Equivalent to `PositiveFinite<f64>`
    pub type PositiveFinite = crate::PositiveFinite<f64>;

    /// Equivalent to `NegativeFinite<f64>`
    pub type NegativeFinite = crate::NegativeFinite<f64>;

    /// Returns `true` if the number is positive zero.
    ///     
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    ///
    /// assert_eq!(tf64::is_positive_zero(3.0), false);
    /// assert_eq!(tf64::is_positive_zero(-0.0), false);
    /// assert_eq!(tf64::is_positive_zero(0.0), true);
    /// ```
    #[inline]
    #[must_use]
    pub fn is_positive_zero(x: f64) -> bool {
        x == 0.0 && x.is_sign_positive()
    }

    /// Returns `true` if the number is negative zero.
    ///    
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    ///
    /// assert_eq!(tf64::is_negative_zero(3.0), false);
    /// assert_eq!(tf64::is_negative_zero(-0.0), true);
    /// assert_eq!(tf64::is_negative_zero(0.0), false);
    /// ```
    #[inline]
    #[must_use]
    pub fn is_negative_zero(x: f64) -> bool {
        x == 0.0 && x.is_sign_negative()
    }

    add_const!(
        INFINITY,
        f64,
        StrictlyPositive,
        f64::INFINITY,
        "Infinity (∞)."
    );
    add_const!(
        NEG_INFINITY,
        f64,
        StrictlyNegative,
        f64::NEG_INFINITY,
        "Negative infinity (−∞)."
    );

    add_const!(
        MAX,
        f64,
        StrictlyPositiveFinite,
        f64::MAX,
        "Largest finite `f64` value."
    );
    add_const!(
        MIN,
        f64,
        StrictlyNegativeFinite,
        f64::MIN,
        "Smallest finite `f64` value."
    );
    add_const!(
        MIN_POSITIVE,
        f64,
        StrictlyPositiveFinite,
        f64::MIN_POSITIVE,
        "Smallest positive normal `f64` value."
    );
    add_const!(ZERO, f64, PositiveFinite, 0.0f64, "Positive zero (+0.0).");
    add_const!(
        NEG_ZERO,
        f64,
        NegativeFinite,
        -0.0f64,
        "Negative zero (-0.0)."
    );

    /// This module contains constants from [`core::f64::consts`], casted to the corresponding type
    pub mod consts {
        add_const!(PI, f64, PositiveFinite, "Archimedes' constant (π)");
        add_const!(
            TAU,
            f64,
            PositiveFinite,
            "The full circle constant (τ). Equal to 2π."
        );
        add_const!(FRAC_PI_2, f64, PositiveFinite, "π/2");
        add_const!(FRAC_PI_3, f64, PositiveFinite, "π/3");
        add_const!(FRAC_PI_4, f64, PositiveFinite, "π/4");
        add_const!(FRAC_PI_6, f64, PositiveFinite, "π/6");
        add_const!(FRAC_PI_8, f64, PositiveFinite, "π/8");
        add_const!(FRAC_1_PI, f64, PositiveFinite, "1/π");
        add_const!(FRAC_2_PI, f64, PositiveFinite, "2/π");
        add_const!(FRAC_2_SQRT_PI, f64, PositiveFinite, "2/sqrt(π)");
        add_const!(SQRT_2, f64, PositiveFinite, "sqrt(2)");
        add_const!(FRAC_1_SQRT_2, f64, PositiveFinite, "1/sqrt(2)");
        add_const!(E, f64, PositiveFinite, "Euler's number (e)");
        add_const!(LOG2_10, f64, PositiveFinite, "log<sub>2</sub>(10)");
        add_const!(LOG2_E, f64, PositiveFinite, "log<sub>2</sub>(e)");
        add_const!(LOG10_2, f64, PositiveFinite, "log<sub>10</sub>(2)");
        add_const!(LOG10_E, f64, PositiveFinite, "log<sub>10</sub>(e)");
        add_const!(LN_2, f64, PositiveFinite, "ln(2)");
        add_const!(LN_10, f64, PositiveFinite, "ln(10)");
    }
}

/// This module contains constants from [`core::f32`], casted to the corresponding type
pub mod tf32 {
    /// Equivalent to `NonNaN<f32>`
    pub type NonNaN = crate::NonNaN<f32>;

    /// Equivalent to `NonNaNFinite<f32>`
    pub type NonNaNFinite = crate::NonNaNFinite<f32>;

    /// Equivalent to `NonZeroNonNaN<f32>`
    pub type NonZero = crate::NonZeroNonNaN<f32>;

    /// Equivalent to `NonZeroNonNaNFinite<f32>`
    pub type NonZeroFinite = crate::NonZeroNonNaNFinite<f32>;

    /// Equivalent to `StrictlyPositive<f32>`
    pub type StrictlyPositive = crate::StrictlyPositive<f32>;

    /// Equivalent to `StrictlyNegative<f32>`
    pub type StrictlyNegative = crate::StrictlyNegative<f32>;

    /// Equivalent to `Positive<f32>`
    pub type Positive = crate::Positive<f32>;

    /// Equivalent to `Negative<f32>`
    pub type Negative = crate::Negative<f32>;

    /// Equivalent to `StrictlyPositiveFinite<f32>`
    pub type StrictlyPositiveFinite = crate::StrictlyPositiveFinite<f32>;

    /// Equivalent to `StrictlyNegativeFinite<f32>`
    pub type StrictlyNegativeFinite = crate::StrictlyNegativeFinite<f32>;

    /// Equivalent to `PositiveFinite<f32>`
    pub type PositiveFinite = crate::PositiveFinite<f32>;

    /// Equivalent to `NegativeFinite<f32>`
    pub type NegativeFinite = crate::NegativeFinite<f32>;

    /// Returns `true` if the number is positive zero.
    ///     
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    ///
    /// assert_eq!(tf32::is_positive_zero(3.0), false);
    /// assert_eq!(tf32::is_positive_zero(-0.0), false);
    /// assert_eq!(tf32::is_positive_zero(0.0), true);
    /// ```
    #[inline]
    #[must_use]
    pub fn is_positive_zero(x: f32) -> bool {
        x == 0.0 && x.is_sign_positive()
    }

    /// Returns `true` if the number is negative zero.
    ///    
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    ///
    /// assert_eq!(tf32::is_negative_zero(3.0), false);
    /// assert_eq!(tf32::is_negative_zero(-0.0), true);
    /// assert_eq!(tf32::is_negative_zero(0.0), false);
    /// ```
    #[inline]
    #[must_use]
    pub fn is_negative_zero(x: f32) -> bool {
        x == 0.0 && x.is_sign_negative()
    }

    add_const!(
        INFINITY,
        f32,
        StrictlyPositive,
        f32::INFINITY,
        "Infinity (∞)."
    );
    add_const!(
        NEG_INFINITY,
        f32,
        StrictlyNegative,
        f32::NEG_INFINITY,
        "Negative infinity (−∞)."
    );

    add_const!(
        MAX,
        f32,
        StrictlyPositiveFinite,
        f32::MAX,
        "Largest finite `f32` value."
    );
    add_const!(
        MIN,
        f32,
        StrictlyNegativeFinite,
        f32::MIN,
        "Smallest finite `f32` value."
    );
    add_const!(
        MIN_POSITIVE,
        f32,
        StrictlyPositiveFinite,
        f32::MIN_POSITIVE,
        "Smallest positive normal `f32` value."
    );
    add_const!(ZERO, f32, PositiveFinite, 0.0f32, "Positive zero (+0.0).");
    add_const!(
        NEG_ZERO,
        f32,
        NegativeFinite,
        -0.0f32,
        "Negative zero (-0.0)."
    );

    /// This module contains constants from [`core::f32::consts`], casted to the corresponding type
    pub mod consts {
        add_const!(PI, f32, PositiveFinite, "Archimedes' constant (π)");
        add_const!(
            TAU,
            f32,
            PositiveFinite,
            "The full circle constant (τ). Equal to 2π."
        );
        add_const!(FRAC_PI_2, f32, PositiveFinite, "π/2");
        add_const!(FRAC_PI_3, f32, PositiveFinite, "π/3");
        add_const!(FRAC_PI_4, f32, PositiveFinite, "π/4");
        add_const!(FRAC_PI_6, f32, PositiveFinite, "π/6");
        add_const!(FRAC_PI_8, f32, PositiveFinite, "π/8");
        add_const!(FRAC_1_PI, f32, PositiveFinite, "1/π");
        add_const!(FRAC_2_PI, f32, PositiveFinite, "2/π");
        add_const!(FRAC_2_SQRT_PI, f32, PositiveFinite, "2/sqrt(π)");
        add_const!(SQRT_2, f32, PositiveFinite, "sqrt(2)");
        add_const!(FRAC_1_SQRT_2, f32, PositiveFinite, "1/sqrt(2)");
        add_const!(E, f32, PositiveFinite, "Euler's number (e)");
        add_const!(LOG2_10, f32, PositiveFinite, "log<sub>2</sub>(10)");
        add_const!(LOG2_E, f32, PositiveFinite, "log<sub>2</sub>(e)");
        add_const!(LOG10_2, f32, PositiveFinite, "log<sub>10</sub>(2)");
        add_const!(LOG10_E, f32, PositiveFinite, "log<sub>10</sub>(e)");
        add_const!(LN_2, f32, PositiveFinite, "ln(2)");
        add_const!(LN_10, f32, PositiveFinite, "ln(10)");
    }
}
