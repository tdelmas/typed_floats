#![doc = include_str!("../README.truncated.md")]
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
    pub trait Float:
        Eq
        + Copy
        + Ord
        + core::fmt::Debug
        + core::str::FromStr
        + TryFrom<Self::Content, Error = InvalidNumber>
        + TryFrom<u8>
        + TryFrom<u16>
        + TryFrom<u32>
        + TryFrom<u64>
        + TryFrom<i8>
        + TryFrom<i16>
        + TryFrom<i32>
        + TryFrom<i64>
        + TryFrom<NonZeroU8>
        + TryFrom<NonZeroU16>
        + TryFrom<NonZeroU32>
        + TryFrom<NonZeroU64>
        + TryFrom<NonZeroI8>
        + TryFrom<NonZeroI16>
        + TryFrom<NonZeroI32>
        + TryFrom<NonZeroI64>
        + std::ops::Add
        + std::ops::Sub
        + std::ops::Mul
        + std::ops::Div
        + std::ops::Rem
    {
        /// The primitive float type (f32 or f64)
        type Content: Sized
            + Clone
            + Copy
            + PartialOrd
            + PartialEq
            + core::fmt::Debug
            + core::fmt::Display
            + std::ops::Add<Output = Self::Content>
            + std::ops::Sub<Output = Self::Content>
            + std::ops::Mul<Output = Self::Content>
            + std::ops::Div<Output = Self::Content>
            + std::ops::Rem<Output = Self::Content>
            + std::ops::AddAssign
            + std::ops::SubAssign
            + std::ops::MulAssign
            + std::ops::DivAssign
            + std::ops::RemAssign;

        /// Creates a new value from a primitive type
        /// It adds a little overhead compared to `new_unchecked`
        /// because it checks that the value is valid
        ///
        /// # Examples
        ///
        /// ```
        /// # use typed_floats::*;
        /// let x: NonNaN = NonNaN::new(3.0).unwrap();
        ///
        /// assert_eq!(x, 3.0);
        /// ```
        ///
        /// # Errors
        /// Returns an error if the value is not valid
        #[inline]
        fn new(value: Self::Content) -> Result<Self, InvalidNumber> {
            Self::try_from(value)
        }

        /// Creates a new value from a primitive type with zero overhead (in release mode).
        /// It is up to the caller to ensure that the value is valid
        ///
        /// # Examples
        ///
        /// ```
        /// # use typed_floats::*;
        /// let x: NonNaN = unsafe { NonNaN::new_unchecked(3.0) };
        ///
        /// assert_eq!(x, 3.0);
        /// ```
        /// # Safety
        /// The caller must ensure that the value is valid
        /// It will panic in debug mode if the value is not valid
        /// but in release mode the behavior is undefined
        unsafe fn new_unchecked(value: Self::Content) -> Self;

        /// Returns the value as a primitive type
        fn get(&self) -> Self::Content;

        /// Returns `true` if this value is NaN.
        /// This is never the case for the provided types
        ///
        /// # Examples
        ///
        /// ```
        /// # use typed_floats::*;
        /// let x: NonNaN = 3.0.try_into().unwrap();
        ///
        /// assert_eq!(x.is_nan(), false);
        /// ```
        ///
        /// See [`f64::is_nan()`] for more details.
        #[must_use]
        fn is_nan(&self) -> bool {
            false
        }

        /// Returns `true` if this value is positive infinity or negative infinity.
        ///
        /// # Examples
        ///
        /// ```
        /// # use typed_floats::*;
        /// let x: NonNaN = 3.0.try_into().unwrap();
        ///
        /// assert_eq!(x.is_infinite(), false);
        /// ```
        ///
        /// See [`f64::is_infinite()`] for more details.
        #[must_use]
        fn is_infinite(&self) -> bool;

        /// Returns `true` if this number is positive infinity nor negative infinity.
        ///
        /// # Examples
        ///
        /// ```
        /// # use typed_floats::*;
        /// let x: NonNaN = 3.0.try_into().unwrap();
        ///
        /// assert_eq!(x.is_finite(), true);
        /// ```
        ///
        /// See [`f64::is_finite()`] for more details.
        #[must_use]
        fn is_finite(&self) -> bool;

        /// Returns `true` if the number is [subnormal](https://en.wikipedia.org/wiki/Denormal_number).
        ///
        /// # Examples
        ///
        /// ```
        /// # use typed_floats::*;
        /// let x: NonNaN = 3.0.try_into().unwrap();
        ///
        /// assert_eq!(x.is_subnormal(), false);
        /// ```
        ///
        /// See [`f64::is_subnormal()`] for more details.
        #[must_use]
        fn is_subnormal(&self) -> bool;

        /// Returns `true` if the number is neither zero, infinite or [subnormal](https://en.wikipedia.org/wiki/Denormal_number).
        ///
        /// # Examples
        ///
        /// ```
        /// # use typed_floats::*;
        /// let x: NonNaN = 3.0.try_into().unwrap();
        ///
        /// assert_eq!(x.is_normal(), true);
        /// ```
        ///
        /// See [`f64::is_normal()`] for more details.
        #[must_use]
        fn is_normal(&self) -> bool;

        /// Returns the floating point category of the number. If only one property
        /// is going to be tested, it is generally faster to use the specific
        /// predicate instead.
        ///
        /// # Examples
        ///
        /// ```
        /// # use typed_floats::*;
        /// let x: NonNaN = 3.0.try_into().unwrap();
        ///
        /// assert_eq!(x.classify(), core::num::FpCategory::Normal);
        /// ```
        ///
        /// See [`f64::classify()`] for more details.
        #[must_use]
        fn classify(&self) -> core::num::FpCategory;

        /// Returns `true` if `self` has a positive sign, including `+0.0` and positive infinity.
        ///
        /// # Examples
        ///
        /// ```
        /// # use typed_floats::*;
        /// let x: NonNaN = 3.0.try_into().unwrap();
        ///
        /// assert_eq!(x.is_sign_positive(), true);
        /// ```
        ///
        /// See [`f64::is_sign_positive()`] for more details.
        #[must_use]
        fn is_sign_positive(&self) -> bool;

        /// Returns `true` if `self` has a negative sign, including `-0.0` and negative infinity.
        ///
        /// # Examples
        ///
        /// ```
        /// # use typed_floats::*;
        /// let x: NonNaN = 3.0.try_into().unwrap();
        ///
        /// assert_eq!(x.is_sign_negative(), false);
        /// ```
        ///
        /// See [`f64::is_sign_negative()`] for more details.
        #[must_use]
        fn is_sign_negative(&self) -> bool;

        /// Returns `true` if the number is positive zero.
        ///
        /// # Examples
        ///
        /// ```
        /// # use typed_floats::*;
        /// let x: NonNaN = 3.0.try_into().unwrap();
        /// let y: NonNaN = (-0.0).try_into().unwrap();
        /// let z: NonNaN = (0.0).try_into().unwrap();
        ///
        /// assert_eq!(x.is_positive_zero(), false);
        /// assert_eq!(y.is_positive_zero(), false);
        /// assert_eq!(z.is_positive_zero(), true);
        /// ```
        fn is_positive_zero(&self) -> bool;

        /// Returns `true` if the number is negative zero.
        ///     
        /// # Examples
        ///
        /// ```
        /// # use typed_floats::*;
        /// let x: NonNaN = 3.0.try_into().unwrap();
        /// let y: NonNaN = (-0.0).try_into().unwrap();
        /// let z: NonNaN = (0.0).try_into().unwrap();
        ///
        /// assert_eq!(x.is_negative_zero(), false);
        /// assert_eq!(y.is_negative_zero(), true);
        /// assert_eq!(z.is_negative_zero(), false);
        /// ```
        fn is_negative_zero(&self) -> bool;
    }
);

/// An error that can occur when converting from a string into a typed `Float`
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
#[derive(Error, Debug)]
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
