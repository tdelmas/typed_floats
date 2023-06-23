//! This crate helps you to ensure the kind of floats you are using.
//!
//! zero overhead :everything is checked at compile time.
//! (only `try_from` adds a little overhead at runtime)
//!
//! `NaN` is rejected by all types.
//!
//! The types provided by this crate are:
//! - [`NonNaN`],[`NonNaNFinite`], [`NonZeroNonNaN`], [`NonZeroNonNaNFinite`]
//! Their positive and negative counterparts:
//! - [`Positive`],[`PositiveFinite`], [`StrictlyPositive`], [`StrictlyPositiveFinite`]
//! - [`Negative`],[`NegativeFinite`], [`StrictlyNegative`], [`StrictlyNegativeFinite`]
//!
//! By default all types are [`f64`] but you can use the [`f32`] like `Positive<f32>`.
//!
//! The following conversions are implemented:
//! - Between all the types of this crate
//! - From [`f64`]
//! - From integers types (expect [`u128`] and [`i128`])
//! - From [`NonZeroUsize`](core::num::NonZeroUsize) and others `NonZero*`
//!
//! # Rules
//!
//! Non-trivial conversions rules for operations are summarized in [`Float`].
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
typed_floats_macros::generate_floats!();

#[cfg(feature = "serde")]
use serde::Serialize;

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

#[cfg(test)]
mod tests {

    use crate::*;

    typed_floats_macros::generate_tests!();

    #[test]
    fn test_others() {
        let values = [
            (f64::NAN, false),
            (f64::INFINITY, true),
            (f64::NEG_INFINITY, true),
            (0.0f64, true),
            (-0.0f64, true),
            (1.0f64, true),
            (-1.0f64, true),
        ];

        for (value, expected) in &values {
            let num = NonNaN::try_from(*value);
            let result = num.is_ok();
            assert_eq!(result, *expected);

            match num {
                Ok(num) => {
                    let v: f64 = num.into();
                    assert_eq!(v, *value);
                }
                Err(_) => {}
            }
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        let map = serde_json::json!({
            "a": 1.0,
        });

        #[derive(Serialize)]
        struct A {
            a: NonNaN,
        }

        let a = A {
            a: NonNaN::try_from(1.0).unwrap(),
        };

        let a_json = serde_json::to_value(a).unwrap();

        assert_eq!(a_json, map);
    }
}
