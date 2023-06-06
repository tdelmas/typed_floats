mod errors;

#[cfg(feature = "serde")]
use serde::Serialize;

pub use errors::*;

/// impl `TryFrom` for `$U`: try to convert `$T` into `$U`
macro_rules! add_try_from {
    ($T:ty, $U:ty) => {
        impl core::convert::TryFrom<$T> for $U {
            type Error = InvalidNumber;

            #[inline]
            fn try_from(value: $T) -> Result<Self, Self::Error> {
                <$U>::try_from(f64::from(value))
            }
        }
    };
}

/// impl `From` for `$T` from `$U`: convert `$T` into `$U`
macro_rules! add_from {
    ($T:ty, $U:ty) => {
        impl From<$T> for $U {
            #[inline]
            fn from(value: $T) -> Self {
                unsafe { Self::new_unchecked(f64::from(value)) }
            }
        }
    };
}

macro_rules! impl_float {
    ($name: ident, $type:ty, $nan:expr, $inf:expr, $zero:expr, $positive:expr, $negative:expr) => {
        #[derive(Debug, Default, Copy, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(Serialize))]
        pub struct $name($type);

        impl $name {
            /// # Safety
            /// The caller must ensure that the value is valid
            /// It will panic in debug mode if the value is not valid
            /// but in release mode the behavior is undefined
            #[inline]
            #[must_use]
            pub unsafe fn new_unchecked(value: f64) -> Self {
                debug_assert!(
                    Self::try_from(value).is_ok(),
                    "{value} is not a valid {name}",
                    value = value,
                    name = stringify!($name)
                );

                Self(value)
            }
        }

        impl Eq for $name {}

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                // This is safe because we know that both values are not NaN
                self.0.partial_cmp(&other.0).unwrap()
            }
        }
        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl From<$name> for f64 {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::ops::Deref for $name {
            type Target = $type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl TryFrom<$type> for $name {
            type Error = InvalidNumber;

            fn try_from(value: $type) -> Result<Self, Self::Error> {
                if !$nan && value.is_nan() {
                    return Err(InvalidNumber::NaN);
                }

                if !$inf && value.is_infinite() {
                    return Err(InvalidNumber::Infinite);
                }

                if !$zero && value == 0.0 {
                    return Err(InvalidNumber::Zero);
                }

                if !$positive && value.is_sign_positive() {
                    return Err(InvalidNumber::Positive);
                }

                if !$negative && value.is_sign_negative() {
                    return Err(InvalidNumber::Negative);
                }

                Ok($name(value))
            }
        }
    };
}

#[cfg_attr(doc, aquamarine::aquamarine)]
/// ```mermaid
/// graph LR
///     ninf([-Inf]) --> n[Negative]
///     n --> nzero([-0])
///     nzero --> zero([+0])
///     zero --> p[Positive]
///     p --> pinf([+Inf])
///     subgraph finite[Finite]
///     n -. nzero .-> zero .-> p
///     end
/// ```
pub enum FloatType {
    NaN,
    Infinite,
    NegativeInfinite,
    Zero,
    PositiveZero,
    NormalPositive,
    NormalNegative,
    SubnormalPositive,
    SubnormalNegative,
}

impl FloatType {
    #[inline]
    #[must_use]
    pub fn from(value: f64) -> Self {
        if value.is_nan() {
            Self::NaN
        } else if value.is_infinite() {
            if value.is_sign_positive() {
                Self::Infinite
            } else {
                Self::NegativeInfinite
            }
        } else if value == 0.0 {
            if value.is_sign_positive() {
                Self::PositiveZero
            } else {
                Self::Zero
            }
        } else if value.is_sign_positive() {
            Self::NormalPositive
        } else {
            Self::NormalNegative
        }
    }
}

macro_rules! impl_from_or_try_from {
    (
        ($namea: ident, $typea:ty, $nana:expr, $infa:expr, $zeroa:expr, true, false),
        ($nameb: ident, $typeb:ty, $nanb:expr, $infb:expr, $zerob:expr, false, true)
    ) => {
        // No conversion between positive and negative
    };
    (
        ($namea: ident, $typea:ty, $nana:expr, $infa:expr, $zeroa:expr, false, true),
        ($nameb: ident, $typeb:ty, $nanb:expr, $infb:expr, $zerob:expr, true, false)
    ) => {
        // No conversion between positive and negative
    };
    (
        ($namea: ident, $typea:ty, $nana:expr, $infa:expr, $zeroa:expr, $positivea:expr, false),
        ($nameb: ident, $typeb:ty, $nanb:expr, $infb:expr, $zerob:expr, $positiveb:expr, true)
    ) => {
        add_try_from!($namea, $nameb);
    };
    (
        ($namea: ident, $typea:ty, $nana:expr, $infa:expr, $zeroa:expr, false, $negativea:expr),
        ($nameb: ident, $typeb:ty, $nanb:expr, $infb:expr, $zerob:expr, true, $negativeb:expr)
    ) => {
        add_try_from!($namea, $nameb);
    };
    (
        ($namea: ident, $typea:ty, $nana:expr, $infa:expr, false, $positivea:expr, $negativea:expr),
        ($nameb: ident, $typeb:ty, $nanb:expr, $infb:expr, true, $positiveb:expr, $negativeb:expr)
    ) => {
        add_try_from!($namea, $nameb);
    };
    (
        ($namea: ident, $typea:ty, $nana:expr, false, $zeroa:expr, $positivea:expr, $negativea:expr),
        ($nameb: ident, $typeb:ty, $nanb:expr, true, $zerob:expr, $positiveb:expr, $negativeb:expr)
    ) => {
        add_try_from!($namea, $nameb);
    };
    (
        ($namea: ident, $typea:ty, false, $infa:expr, $zeroa:expr, $positivea:expr, $negativea:expr),
        ($nameb: ident, $typeb:ty, true, $infb:expr, $zerob:expr, $positiveb:expr, $negativeb:expr)
    ) => {
        add_try_from!($namea, $nameb);
    };

    (
        ($namea: ident, $typea:ty, $nana:expr, $infa:expr, $zeroa:expr, $positivea:expr, $negativea:expr),
        ($nameb: ident, $typeb:ty, $nanb:expr, $infb:expr, $zerob:expr, $positiveb:expr, $negativeb:expr)
    ) => {
        add_from!($namea, $nameb);
    };
}

macro_rules! impl_from_float {
    ($a:tt,$b:tt,$($c:tt)*) => {
        impl_from_float!($a, $($c)*);
        impl_from_or_try_from!($a, $b);
    };
    ($a:tt,$b:tt) => {
        impl_from_or_try_from!($a, $b);
    };
}

macro_rules! impl_floats2 {
    ($a:tt,$($b:tt)*) => {
        impl_float!$a;
        impl_floats2!($($b)*);

        impl_from_float!($a, $($b)*);
    };
    ($a:tt) => {
        impl_float!$a;
    };
}

macro_rules! impl_floats {
    ($type:ty) => {
        impl_floats2!(
            (NonNaN, $type, false, true, true, true, true),
            (NonZeroNonNaN, $type, false, true, false, true, true),
            (NonNaNFinite, $type, false, false, true, true, true),
            (NonZeroNonNaNFinite, $type, false, false, false, true, true),
            (Positive, $type, false, true, true, true, false),
            (Negative, $type, false, true, true, false, true),
            (PositiveFinite, $type, false, false, true, true, false),
            (NegativeFinite, $type, false, false, true, false, true),
            (StrictlyPositive, $type, false, true, false, true, false),
            (StrictlyNegative, $type, false, true, false, false, true),
            (
                StrictlyPositiveFinite,
                $type,
                false,
                false,
                false,
                true,
                false
            ),
            (
                StrictlyNegativeFinite,
                $type,
                false,
                false,
                false,
                false,
                true
            )
        );
    };
}

impl_floats!(f64);
