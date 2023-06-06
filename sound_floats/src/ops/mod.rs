mod add;
mod div;
mod mul;
mod rem;
mod sub;

use crate::types::{
    Negative, NegativeFinite, NonNaN, NonZeroNonNaN, Positive, PositiveFinite, StrictlyNegative,
    StrictlyNegativeFinite, StrictlyPositive, StrictlyPositiveFinite,
};

use core::ops::Neg;

#[macro_export]
macro_rules! add_fn {
    ($function:ident, $T:ty, f64) => {
        impl $T {
            /// <https://doc.rust-lang.org/std/primitive.f64.html#method.$function>
            #[inline]
            #[must_use]
            pub fn $function(self) -> $T_Output {
                f64::from(self).$function().into()
            }
        }
    };
    ($function:ident, $T:ty, $T_Output:ty) => {
        impl $T {
            /// <https://doc.rust-lang.org/std/primitive.f64.html#method.$function>
            #[inline]
            #[must_use]
            pub fn $function(self) -> $T_Output {
                unsafe { <$T_Output>::new_unchecked(f64::from(self).$function()) }
            }
        }
    };
    ($function:ident, $T:ty, $U:ty, $T_Output:ty) => {
        impl $T {
            /// <https://doc.rust-lang.org/std/primitive.f64.html#method.$function>
            #[inline]
            #[must_use]
            pub fn $function(self, other: $U) -> $T_Output {
                unsafe { <$T_Output>::new_unchecked(f64::from(self).$function(other)) }
            }
        }
    };
}

#[macro_export]
macro_rules! add_impl {
    ($trait:ident,$function:ident, $T:ty, f64) => {
        impl $trait for $T {
            type Output = $T_Output;

            #[inline]
            fn $function(self) -> Self::Output {
                f64::from(self).$function().into()
            }
        }
    };
    ($trait:ident,$function:ident, $T:ty, $T_Output:ty) => {
        impl $trait for $T {
            type Output = $T_Output;

            #[inline]
            fn $function(self) -> Self::Output {
                unsafe { <$T_Output>::new_unchecked(f64::from(self).$function().into()) }
            }
        }
    };
    ($trait:ident,$function:ident, $T:ty, $U:ty, f64) => {
        impl $trait<$U> for $T {
            type Output = f64;

            #[inline]
            fn $function(self, other: $U) -> Self::Output {
                f64::from(self).$function(f64::from(other))
            }
        }
    };
    ($trait:ident,$function:ident, $T:ty, $U:ty, $T_Output:ty) => {
        impl $trait<$U> for $T {
            type Output = $T_Output;

            #[inline]
            fn $function(self, other: $U) -> Self::Output {
                unsafe { <$T_Output>::new_unchecked(f64::from(self).$function(f64::from(other))) }
            }
        }
    };
}

add_fn!(abs, NonNaN, Positive);
add_fn!(abs, NonZeroNonNaN, StrictlyPositive);
add_fn!(abs, Positive, Positive);
add_fn!(abs, Negative, Positive);
add_fn!(abs, StrictlyPositive, StrictlyPositive);
add_fn!(abs, StrictlyNegative, StrictlyPositive);
add_fn!(abs, NegativeFinite, PositiveFinite);
add_fn!(abs, PositiveFinite, PositiveFinite);
add_fn!(abs, StrictlyNegativeFinite, StrictlyPositiveFinite);
add_fn!(abs, StrictlyPositiveFinite, StrictlyPositiveFinite);

add_impl!(Neg, neg, NonNaN, NonNaN);
add_impl!(Neg, neg, NonZeroNonNaN, NonZeroNonNaN);
add_impl!(Neg, neg, Positive, Negative);
add_impl!(Neg, neg, Negative, Positive);
add_impl!(Neg, neg, StrictlyPositive, StrictlyNegative);
add_impl!(Neg, neg, StrictlyNegative, StrictlyPositive);
add_impl!(Neg, neg, NegativeFinite, PositiveFinite);
add_impl!(Neg, neg, PositiveFinite, NegativeFinite);
add_impl!(Neg, neg, StrictlyNegativeFinite, StrictlyPositiveFinite);
add_impl!(Neg, neg, StrictlyPositiveFinite, StrictlyNegativeFinite);

// todo when possible: AddAssign, add_assign
// todo when possible: DivAssign, div_assign
// todo when possible: MulAssign, mul_assign
// todo when possible: RemAssign, rem_assign
// todo when possible: SubAssign, sub_assign
