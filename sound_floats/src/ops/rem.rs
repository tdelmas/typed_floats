use core::ops::Rem;

use crate::types::{
    Negative, NegativeFinite, NonNaN, NonZeroNonNaN, Positive, PositiveFinite, StrictlyNegative,
    StrictlyNegativeFinite, StrictlyPositive, StrictlyPositiveFinite,
};

crate::add_impl!(Rem, rem, NonNaN, NonNaN, f64);
crate::add_impl!(Rem, rem, NonNaN, NonZeroNonNaN, f64);
crate::add_impl!(Rem, rem, NonNaN, Positive, f64);
crate::add_impl!(Rem, rem, NonNaN, Negative, f64);
crate::add_impl!(Rem, rem, NonNaN, StrictlyPositive, f64);
crate::add_impl!(Rem, rem, NonNaN, StrictlyNegative, f64);
crate::add_impl!(Rem, rem, NonNaN, NegativeFinite, f64);
crate::add_impl!(Rem, rem, NonNaN, PositiveFinite, f64);
crate::add_impl!(Rem, rem, NonNaN, StrictlyNegativeFinite, f64);
crate::add_impl!(Rem, rem, NonNaN, StrictlyPositiveFinite, f64);

crate::add_impl!(Rem, rem, NonZeroNonNaN, NonNaN, f64);
crate::add_impl!(Rem, rem, NonZeroNonNaN, NonZeroNonNaN, f64);
crate::add_impl!(Rem, rem, NonZeroNonNaN, Positive, f64);
crate::add_impl!(Rem, rem, NonZeroNonNaN, Negative, f64);
crate::add_impl!(Rem, rem, NonZeroNonNaN, StrictlyPositive, f64);
crate::add_impl!(Rem, rem, NonZeroNonNaN, StrictlyNegative, f64);
crate::add_impl!(Rem, rem, NonZeroNonNaN, NegativeFinite, f64);
crate::add_impl!(Rem, rem, NonZeroNonNaN, PositiveFinite, f64);
crate::add_impl!(Rem, rem, NonZeroNonNaN, StrictlyNegativeFinite, f64);
crate::add_impl!(Rem, rem, NonZeroNonNaN, StrictlyPositiveFinite, f64);

crate::add_impl!(Rem, rem, Positive, NonNaN, f64);
crate::add_impl!(Rem, rem, Positive, NonZeroNonNaN, f64);
crate::add_impl!(Rem, rem, Positive, Positive, f64);
crate::add_impl!(Rem, rem, Positive, Negative, f64);
crate::add_impl!(Rem, rem, Positive, StrictlyPositive, f64);
crate::add_impl!(Rem, rem, Positive, StrictlyNegative, f64);
crate::add_impl!(Rem, rem, Positive, NegativeFinite, f64);
crate::add_impl!(Rem, rem, Positive, PositiveFinite, f64);
crate::add_impl!(Rem, rem, Positive, StrictlyNegativeFinite, f64);
crate::add_impl!(Rem, rem, Positive, StrictlyPositiveFinite, f64);

crate::add_impl!(Rem, rem, Negative, NonNaN, f64);
crate::add_impl!(Rem, rem, Negative, NonZeroNonNaN, f64);
crate::add_impl!(Rem, rem, Negative, Positive, f64);
crate::add_impl!(Rem, rem, Negative, Negative, f64);
crate::add_impl!(Rem, rem, Negative, StrictlyPositive, f64);
crate::add_impl!(Rem, rem, Negative, StrictlyNegative, f64);
crate::add_impl!(Rem, rem, Negative, NegativeFinite, f64);
crate::add_impl!(Rem, rem, Negative, PositiveFinite, f64);
crate::add_impl!(Rem, rem, Negative, StrictlyNegativeFinite, f64);
crate::add_impl!(Rem, rem, Negative, StrictlyPositiveFinite, f64);

crate::add_impl!(Rem, rem, StrictlyPositive, NonNaN, f64);
crate::add_impl!(Rem, rem, StrictlyPositive, NonZeroNonNaN, f64);
crate::add_impl!(Rem, rem, StrictlyPositive, Positive, f64);
crate::add_impl!(Rem, rem, StrictlyPositive, Negative, f64);
crate::add_impl!(Rem, rem, StrictlyPositive, StrictlyPositive, f64);
crate::add_impl!(Rem, rem, StrictlyPositive, StrictlyNegative, f64);
crate::add_impl!(Rem, rem, StrictlyPositive, NegativeFinite, f64);
crate::add_impl!(Rem, rem, StrictlyPositive, PositiveFinite, f64);
crate::add_impl!(Rem, rem, StrictlyPositive, StrictlyNegativeFinite, f64);
crate::add_impl!(Rem, rem, StrictlyPositive, StrictlyPositiveFinite, f64);

crate::add_impl!(Rem, rem, StrictlyNegative, NonNaN, f64);
crate::add_impl!(Rem, rem, StrictlyNegative, NonZeroNonNaN, f64);
crate::add_impl!(Rem, rem, StrictlyNegative, Positive, f64);
crate::add_impl!(Rem, rem, StrictlyNegative, Negative, f64);
crate::add_impl!(Rem, rem, StrictlyNegative, StrictlyPositive, f64);
crate::add_impl!(Rem, rem, StrictlyNegative, StrictlyNegative, f64);
crate::add_impl!(Rem, rem, StrictlyNegative, NegativeFinite, f64);
crate::add_impl!(Rem, rem, StrictlyNegative, PositiveFinite, f64);
crate::add_impl!(Rem, rem, StrictlyNegative, StrictlyNegativeFinite, f64);
crate::add_impl!(Rem, rem, StrictlyNegative, StrictlyPositiveFinite, f64);

crate::add_impl!(Rem, rem, NegativeFinite, NonNaN, f64);
crate::add_impl!(Rem, rem, NegativeFinite, NonZeroNonNaN, f64);
crate::add_impl!(Rem, rem, NegativeFinite, Positive, f64);
crate::add_impl!(Rem, rem, NegativeFinite, Negative, f64);
crate::add_impl!(Rem, rem, NegativeFinite, StrictlyPositive, f64);
crate::add_impl!(Rem, rem, NegativeFinite, StrictlyNegative, f64);
crate::add_impl!(Rem, rem, NegativeFinite, NegativeFinite, f64);
crate::add_impl!(Rem, rem, NegativeFinite, PositiveFinite, f64);
crate::add_impl!(Rem, rem, NegativeFinite, StrictlyNegativeFinite, f64);
crate::add_impl!(Rem, rem, NegativeFinite, StrictlyPositiveFinite, f64);

crate::add_impl!(Rem, rem, PositiveFinite, NonNaN, f64);
crate::add_impl!(Rem, rem, PositiveFinite, NonZeroNonNaN, f64);
crate::add_impl!(Rem, rem, PositiveFinite, Positive, f64);
crate::add_impl!(Rem, rem, PositiveFinite, Negative, f64);
crate::add_impl!(Rem, rem, PositiveFinite, StrictlyPositive, f64);
crate::add_impl!(Rem, rem, PositiveFinite, StrictlyNegative, f64);
crate::add_impl!(Rem, rem, PositiveFinite, NegativeFinite, f64);
crate::add_impl!(Rem, rem, PositiveFinite, PositiveFinite, f64);
crate::add_impl!(Rem, rem, PositiveFinite, StrictlyNegativeFinite, f64);
crate::add_impl!(Rem, rem, PositiveFinite, StrictlyPositiveFinite, f64);

crate::add_impl!(Rem, rem, StrictlyNegativeFinite, NonNaN, f64);
crate::add_impl!(Rem, rem, StrictlyNegativeFinite, NonZeroNonNaN, f64);
crate::add_impl!(Rem, rem, StrictlyNegativeFinite, Positive, f64);
crate::add_impl!(Rem, rem, StrictlyNegativeFinite, Negative, f64);
crate::add_impl!(Rem, rem, StrictlyNegativeFinite, StrictlyPositive, f64);
crate::add_impl!(Rem, rem, StrictlyNegativeFinite, StrictlyNegative, f64);
crate::add_impl!(Rem, rem, StrictlyNegativeFinite, NegativeFinite, f64);
crate::add_impl!(Rem, rem, StrictlyNegativeFinite, PositiveFinite, f64);
crate::add_impl!(
    Rem,
    rem,
    StrictlyNegativeFinite,
    StrictlyNegativeFinite,
    f64
);
crate::add_impl!(
    Rem,
    rem,
    StrictlyNegativeFinite,
    StrictlyPositiveFinite,
    f64
);

crate::add_impl!(Rem, rem, StrictlyPositiveFinite, NonNaN, f64);
crate::add_impl!(Rem, rem, StrictlyPositiveFinite, NonZeroNonNaN, f64);
crate::add_impl!(Rem, rem, StrictlyPositiveFinite, Positive, f64);
crate::add_impl!(Rem, rem, StrictlyPositiveFinite, Negative, f64);
crate::add_impl!(Rem, rem, StrictlyPositiveFinite, StrictlyPositive, f64);
crate::add_impl!(Rem, rem, StrictlyPositiveFinite, StrictlyNegative, f64);
crate::add_impl!(Rem, rem, StrictlyPositiveFinite, NegativeFinite, f64);
crate::add_impl!(Rem, rem, StrictlyPositiveFinite, PositiveFinite, f64);
crate::add_impl!(
    Rem,
    rem,
    StrictlyPositiveFinite,
    StrictlyNegativeFinite,
    f64
);
crate::add_impl!(
    Rem,
    rem,
    StrictlyPositiveFinite,
    StrictlyPositiveFinite,
    f64
);
