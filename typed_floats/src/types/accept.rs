macro_rules! accept_infinity {
    (Negative) => {
        true
    };
    (NegativeFinite) => {
        false
    };
    (NonNaN) => {
        true
    };
    (NonNaNFinite) => {
        false
    };
    (NonZeroNonNaN) => {
        true
    };
    (NonZeroNonNaNFinite) => {
        false
    };
    (Positive) => {
        true
    };
    (PositiveFinite) => {
        false
    };
    (StrictlyNegative) => {
        true
    };
    (StrictlyNegativeFinite) => {
        false
    };
    (StrictlyPositive) => {
        true
    };
    (StrictlyPositiveFinite) => {
        false
    };
}

macro_rules! accept_zero {
    (Negative) => {
        true
    };
    (NegativeFinite) => {
        true
    };
    (NonNaN) => {
        true
    };
    (NonNaNFinite) => {
        true
    };
    (NonZeroNonNaN) => {
        false
    };
    (NonZeroNonNaNFinite) => {
        false
    };
    (Positive) => {
        true
    };
    (PositiveFinite) => {
        true
    };
    (StrictlyNegative) => {
        false
    };
    (StrictlyNegativeFinite) => {
        false
    };
    (StrictlyPositive) => {
        false
    };
    (StrictlyPositiveFinite) => {
        false
    };
}

macro_rules! accept_negative {
    (Negative) => {
        true
    };
    (NegativeFinite) => {
        true
    };
    (NonNaN) => {
        true
    };
    (NonNaNFinite) => {
        true
    };
    (NonZeroNonNaN) => {
        true
    };
    (NonZeroNonNaNFinite) => {
        true
    };
    (Positive) => {
        false
    };
    (PositiveFinite) => {
        false
    };
    (StrictlyNegative) => {
        true
    };
    (StrictlyNegativeFinite) => {
        true
    };
    (StrictlyPositive) => {
        false
    };
    (StrictlyPositiveFinite) => {
        false
    };
}

macro_rules! accept_positive {
    (Negative) => {
        false
    };
    (NegativeFinite) => {
        false
    };
    (NonNaN) => {
        true
    };
    (NonNaNFinite) => {
        true
    };
    (NonZeroNonNaN) => {
        true
    };
    (NonZeroNonNaNFinite) => {
        true
    };
    (Positive) => {
        true
    };
    (PositiveFinite) => {
        true
    };
    (StrictlyNegative) => {
        false
    };
    (StrictlyNegativeFinite) => {
        false
    };
    (StrictlyPositive) => {
        true
    };
    (StrictlyPositiveFinite) => {
        true
    };
}

macro_rules! accept {
    ($type:ident) => {
        impl $type {
            /// Returns true if the type can accept infinity
            #[must_use]
            pub const fn accept_infinity() -> bool {
                accept_infinity!($type)
            }

            /// Returns true if the type can accept zero
            #[must_use]
            pub const fn accept_zero() -> bool {
                accept_zero!($type)
            }

            /// Returns true if the type can accept negative values
            #[must_use]
            pub const fn accept_negative() -> bool {
                accept_negative!($type)
            }

            /// Returns true if the type can accept positive values
            #[must_use]
            pub const fn accept_positive() -> bool {
                accept_positive!($type)
            }
        }

        #[cfg(feature = "f32")]
        impl $type<f32> {
            /// Creates a new value from a primitive type without checking that the value is valid
            ///
            /// # Safety
            /// This function is only meant to be used by the macros of this crate.
            #[inline]
            #[must_use]
            #[doc(hidden)]
            pub const unsafe fn internal_only_new_unchecked(value: f32) -> Self {
                Self(value)
            }
        }

        #[cfg(feature = "f64")]
        impl $type<f64> {
            /// Creates a new value from a primitive type without checking that the value is valid
            ///
            /// # Safety
            /// This function is only meant to be used by the macros of this crate.
            #[inline]
            #[must_use]
            #[doc(hidden)]
            pub const unsafe fn internal_only_new_unchecked(value: f64) -> Self {
                Self(value)
            }
        }
    };
}

#[cfg(feature = "f32")]
use crate::types::f32;
#[cfg(feature = "f64")]
use crate::types::f64;

use crate::types::{
    Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN, NonZeroNonNaNFinite, Positive,
    PositiveFinite, StrictlyNegative, StrictlyNegativeFinite, StrictlyPositive,
    StrictlyPositiveFinite,
};

accept!(Negative);
accept!(NegativeFinite);
accept!(NonNaN);
accept!(NonNaNFinite);
accept!(NonZeroNonNaN);
accept!(NonZeroNonNaNFinite);
accept!(Positive);
accept!(PositiveFinite);
accept!(StrictlyNegative);
accept!(StrictlyNegativeFinite);
accept!(StrictlyPositive);
accept!(StrictlyPositiveFinite);
