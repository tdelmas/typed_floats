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
#![warn(clippy::indexing_slicing)]
#![warn(clippy::nursery)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::unwrap_used)]
#![warn(missing_docs)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_crate_dependencies)]
//#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

mod traits;
mod types;

#[cfg(feature = "serde")]
mod serde;

pub use traits::*;
pub use types::*;

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
        let left_val: f64 = $left.into();
        let right_val: f64 = $right.into();

        assert!(
            (left_val == right_val) || (left_val - right_val).abs() <= $epsilon,
            "assertion failed: `(left ~= right)` \
             (left: `{:?}`, right: `{:?}`, (left - right): `{:?}` > epsilon: `{:?}`)",
            left_val,
            right_val,
            left_val - right_val,
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

typed_floats_macros::generate_docs!(
    pub trait TypedFloat {}
);

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

    /// Infinity (∞).
    pub const INFINITY: crate::StrictlyPositive<f64> =
        crate::StrictlyPositive::<f64>(core::f64::INFINITY);
    /// Negative infinity (−∞).
    pub const NEG_INFINITY: crate::StrictlyNegative<f64> =
        crate::StrictlyNegative::<f64>(core::f64::NEG_INFINITY);
    /// Largest finite `f64` value.
    pub const MAX: crate::StrictlyPositiveFinite<f64> =
        crate::StrictlyPositiveFinite::<f64>(core::f64::MAX);
    /// Smallest finite `f64` value.
    pub const MIN: crate::StrictlyNegativeFinite<f64> =
        crate::StrictlyNegativeFinite::<f64>(core::f64::MIN);
    /// Smallest positive normal `f64` value.
    pub const MIN_POSITIVE: crate::StrictlyPositiveFinite<f64> =
        crate::StrictlyPositiveFinite::<f64>(core::f64::MIN_POSITIVE);
    /// Positive zero (+0.0).
    pub const ZERO: crate::PositiveFinite<f64> = crate::PositiveFinite::<f64>(0.0f64);
    /// Negative zero (-0.0).
    pub const NEG_ZERO: crate::NegativeFinite<f64> = crate::NegativeFinite::<f64>(-0.0f64);

    /// This module contains constants from [`core::f64::consts`], casted to the corresponding type
    pub mod consts {
        /// Archimedes' constant (π)
        pub const PI: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::PI);
        /// The full circle constant (τ). Equal to 2π.
        pub const TAU: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::TAU);
        /// π/2
        pub const FRAC_PI_2: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::FRAC_PI_2);
        /// π/3
        pub const FRAC_PI_3: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::FRAC_PI_3);
        /// π/4
        pub const FRAC_PI_4: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::FRAC_PI_4);
        /// π/6
        pub const FRAC_PI_6: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::FRAC_PI_6);
        /// π/8
        pub const FRAC_PI_8: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::FRAC_PI_8);
        /// 1/π
        pub const FRAC_1_PI: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::FRAC_1_PI);
        /// 2/π
        pub const FRAC_2_PI: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::FRAC_2_PI);
        /// 2/sqrt(π)
        pub const FRAC_2_SQRT_PI: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::FRAC_2_SQRT_PI);
        /// sqrt(2)
        pub const SQRT_2: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::SQRT_2);
        /// 1/sqrt(2)
        pub const FRAC_1_SQRT_2: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::FRAC_1_SQRT_2);
        /// Euler's number (e)
        pub const E: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::E);
        /// log<sub>2</sub>(10)
        pub const LOG2_10: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::LOG2_10);
        /// log<sub>2</sub>(e)
        pub const LOG2_E: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::LOG2_E);
        /// log<sub>10</sub>(2)
        pub const LOG10_2: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::LOG10_2);
        /// log<sub>10</sub>(e)
        pub const LOG10_E: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::LOG10_E);
        /// ln(2)
        pub const LN_2: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::LN_2);
        /// ln(10)
        pub const LN_10: crate::PositiveFinite<f64> =
            crate::PositiveFinite::<f64>(core::f64::consts::LN_10);
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

    /// Infinity (∞).
    pub const INFINITY: crate::StrictlyPositive<f32> =
        crate::StrictlyPositive::<f32>(core::f32::INFINITY);
    /// Negative infinity (−∞).
    pub const NEG_INFINITY: crate::StrictlyNegative<f32> =
        crate::StrictlyNegative::<f32>(core::f32::NEG_INFINITY);
    /// Largest finite `f32` value.
    pub const MAX: crate::StrictlyPositiveFinite<f32> =
        crate::StrictlyPositiveFinite::<f32>(core::f32::MAX);
    /// Smallest finite `f32` value.
    pub const MIN: crate::StrictlyNegativeFinite<f32> =
        crate::StrictlyNegativeFinite::<f32>(core::f32::MIN);
    /// Smallest positive normal `f32` value.
    pub const MIN_POSITIVE: crate::StrictlyPositiveFinite<f32> =
        crate::StrictlyPositiveFinite::<f32>(core::f32::MIN_POSITIVE);
    /// Positive zero (+0.0).
    pub const ZERO: crate::PositiveFinite<f32> = crate::PositiveFinite::<f32>(0.0f32);
    /// Negative zero (-0.0).
    pub const NEG_ZERO: crate::NegativeFinite<f32> = crate::NegativeFinite::<f32>(-0.0f32);

    /// This module contains constants from [`core::f32::consts`], casted to the corresponding type
    pub mod consts {
        /// Archimedes' constant (π)
        pub const PI: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::PI);
        /// The full circle constant (τ). Equal to 2π.
        pub const TAU: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::TAU);
        /// π/2
        pub const FRAC_PI_2: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::FRAC_PI_2);
        /// π/3
        pub const FRAC_PI_3: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::FRAC_PI_3);
        /// π/4
        pub const FRAC_PI_4: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::FRAC_PI_4);
        /// π/6
        pub const FRAC_PI_6: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::FRAC_PI_6);
        /// π/8
        pub const FRAC_PI_8: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::FRAC_PI_8);
        /// 1/π
        pub const FRAC_1_PI: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::FRAC_1_PI);
        /// 2/π
        pub const FRAC_2_PI: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::FRAC_2_PI);
        /// 2/sqrt(π)
        pub const FRAC_2_SQRT_PI: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::FRAC_2_SQRT_PI);
        /// sqrt(2)
        pub const SQRT_2: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::SQRT_2);
        /// 1/sqrt(2)
        pub const FRAC_1_SQRT_2: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::FRAC_1_SQRT_2);
        /// Euler's number (e)
        pub const E: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::E);
        /// log<sub>2</sub>(10)
        pub const LOG2_10: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::LOG2_10);
        /// log<sub>2</sub>(e)
        pub const LOG2_E: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::LOG2_E);
        /// log<sub>10</sub>(2)
        pub const LOG10_2: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::LOG10_2);
        /// log<sub>10</sub>(e)
        pub const LOG10_E: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::LOG10_E);
        /// ln(2)
        pub const LN_2: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::LN_2);
        /// ln(10)
        pub const LN_10: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::LN_10);
    }
}
