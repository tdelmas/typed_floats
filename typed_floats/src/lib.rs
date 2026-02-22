#![doc = include_str!("../README.truncated.md")]
//! # Rules
//!
//! Conversions rules for operations are summarized in [`conversions_rules`].
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
//! Also, comparison between types is available:
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
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(any(feature = "f32", feature = "f64")))]
compile_error!("At least one of the features `f32` or `f64` must be enabled.");

// `format!` is used during the tests even in `no_std` environments
#[cfg(all(test, not(feature = "std")))]
#[macro_use]
extern crate alloc;

mod macros;
#[cfg(feature = "f32")]
pub mod tf32;
#[cfg(feature = "f64")]
pub mod tf64;
mod traits;
mod types;

#[cfg(feature = "serde")]
mod serde;

pub use traits::*;
pub use types::*;

typed_floats_macros::generate_docs!(
    pub mod conversions_rules {}
);

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_sorted<T: PartialEq + PartialOrd + core::fmt::Debug>(values: &[T], zero: T) {
        assert!(values.len() >= 2);
        for i in 1..values.len() {
            let a = &values[i - 1];
            let b = &values[i];

            if a != &zero || b != &zero {
                assert!(a < b, "{a:?} < {b:?} should be true");
            }
        }
    }

    #[cfg(feature = "f32")]
    #[test]
    fn test_f32() {
        let mut values = tf32::get_test_values().to_vec();
        let first = values.remove(0);
        assert!(first.is_nan());

        let others = values.as_slice();

        assert_sorted(others, 0.0);

        let mut count_inf = 0;
        let mut count_zero = 0;
        let mut count_subnormal = 0;

        for value in others {
            if value.is_infinite() {
                count_inf += 1;
            }
            if value == &0.0 {
                // 0.0 or -0.0
                count_zero += 1;
            }
            if value.is_subnormal() {
                count_subnormal += 1;
            }
        }

        assert_eq!(count_inf, 2);
        assert_eq!(count_zero, 2);
        assert!(count_subnormal >= 3);
    }

    #[test]
    #[cfg(feature = "f64")]
    fn test_f64() {
        let mut values = tf64::get_test_values().to_vec();
        let first = values.remove(0);
        assert!(first.is_nan());

        let others = values.as_slice();

        assert_sorted(others, 0.0);

        let mut count_inf = 0;
        let mut count_zero = 0;
        let mut count_subnormal = 0;

        for value in others {
            if value.is_infinite() {
                count_inf += 1;
            }
            if value == &0.0 {
                // 0.0 or -0.0
                count_zero += 1;
            }
            if value.is_subnormal() {
                count_subnormal += 1;
            }
        }

        assert_eq!(count_inf, 2);
        assert_eq!(count_zero, 2);
        assert!(count_subnormal >= 3);
    }

    #[test]
    #[cfg(feature = "f32")]
    fn test_subnormals_f32() {
        const SUBNORMALS_F32: [f32; 4] = [
            tf32::MAX_SUBNORMAL_NEGATIVE.get(),
            tf32::MIN_SUBNORMAL_NEGATIVE.get(),
            tf32::MIN_SUBNORMAL_POSITIVE.get(),
            tf32::MAX_SUBNORMAL_POSITIVE.get(),
        ];

        assert_sorted(&SUBNORMALS_F32, 0.0);

        for value in SUBNORMALS_F32 {
            assert!(value.is_subnormal());
        }
    }

    #[test]
    #[cfg(feature = "f64")]
    fn test_subnormals_f64() {
        const SUBNORMALS_F64: [f64; 4] = [
            tf64::MAX_SUBNORMAL_NEGATIVE.get(),
            tf64::MIN_SUBNORMAL_NEGATIVE.get(),
            tf64::MIN_SUBNORMAL_POSITIVE.get(),
            tf64::MAX_SUBNORMAL_POSITIVE.get(),
        ];

        assert_sorted(&SUBNORMALS_F64, 0.0);

        for value in SUBNORMALS_F64 {
            assert!(value.is_subnormal());
        }
    }
}
