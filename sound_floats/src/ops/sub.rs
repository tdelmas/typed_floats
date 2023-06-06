use core::ops::Sub;

use crate::types::{
    Negative, NegativeFinite, NonNaN, NonZeroNonNaN, Positive, PositiveFinite, StrictlyNegative,
    StrictlyNegativeFinite, StrictlyPositive, StrictlyPositiveFinite,
};

crate::add_impl!(Sub, sub, NonNaN, NonNaN, f64);
crate::add_impl!(Sub, sub, NonNaN, NonZeroNonNaN, f64);
crate::add_impl!(Sub, sub, NonNaN, Positive, f64);
crate::add_impl!(Sub, sub, NonNaN, Negative, f64);
crate::add_impl!(Sub, sub, NonNaN, StrictlyPositive, f64);
crate::add_impl!(Sub, sub, NonNaN, StrictlyNegative, f64);
crate::add_impl!(Sub, sub, NonNaN, NegativeFinite, f64);
crate::add_impl!(Sub, sub, NonNaN, PositiveFinite, f64);
crate::add_impl!(Sub, sub, NonNaN, StrictlyNegativeFinite, f64);
crate::add_impl!(Sub, sub, NonNaN, StrictlyPositiveFinite, f64);

crate::add_impl!(Sub, sub, NonZeroNonNaN, NonNaN, f64);
crate::add_impl!(Sub, sub, NonZeroNonNaN, NonZeroNonNaN, f64);
crate::add_impl!(Sub, sub, NonZeroNonNaN, Positive, f64);
crate::add_impl!(Sub, sub, NonZeroNonNaN, Negative, f64);
crate::add_impl!(Sub, sub, NonZeroNonNaN, StrictlyPositive, f64);
crate::add_impl!(Sub, sub, NonZeroNonNaN, StrictlyNegative, f64);
crate::add_impl!(Sub, sub, NonZeroNonNaN, NegativeFinite, f64);
crate::add_impl!(Sub, sub, NonZeroNonNaN, PositiveFinite, f64);
crate::add_impl!(Sub, sub, NonZeroNonNaN, StrictlyNegativeFinite, f64);
crate::add_impl!(Sub, sub, NonZeroNonNaN, StrictlyPositiveFinite, f64);

crate::add_impl!(Sub, sub, Positive, NonNaN, f64);
crate::add_impl!(Sub, sub, Positive, NonZeroNonNaN, f64);
crate::add_impl!(Sub, sub, Positive, Positive, f64);
crate::add_impl!(Sub, sub, Positive, Negative, f64);
crate::add_impl!(Sub, sub, Positive, StrictlyPositive, f64);
crate::add_impl!(Sub, sub, Positive, StrictlyNegative, f64);
crate::add_impl!(Sub, sub, Positive, NegativeFinite, f64);
crate::add_impl!(Sub, sub, Positive, PositiveFinite, f64);
crate::add_impl!(Sub, sub, Positive, StrictlyNegativeFinite, f64);
crate::add_impl!(Sub, sub, Positive, StrictlyPositiveFinite, f64);

crate::add_impl!(Sub, sub, Negative, NonNaN, f64);
crate::add_impl!(Sub, sub, Negative, NonZeroNonNaN, f64);
crate::add_impl!(Sub, sub, Negative, Positive, f64);
crate::add_impl!(Sub, sub, Negative, Negative, f64);
crate::add_impl!(Sub, sub, Negative, StrictlyPositive, f64);
crate::add_impl!(Sub, sub, Negative, StrictlyNegative, f64);
crate::add_impl!(Sub, sub, Negative, NegativeFinite, f64);
crate::add_impl!(Sub, sub, Negative, PositiveFinite, f64);
crate::add_impl!(Sub, sub, Negative, StrictlyNegativeFinite, f64);
crate::add_impl!(Sub, sub, Negative, StrictlyPositiveFinite, f64);

crate::add_impl!(Sub, sub, StrictlyPositive, NonNaN, f64);
crate::add_impl!(Sub, sub, StrictlyPositive, NonZeroNonNaN, f64);
crate::add_impl!(Sub, sub, StrictlyPositive, Positive, f64);
crate::add_impl!(Sub, sub, StrictlyPositive, Negative, f64);
crate::add_impl!(Sub, sub, StrictlyPositive, StrictlyPositive, f64);
crate::add_impl!(Sub, sub, StrictlyPositive, StrictlyNegative, f64);
crate::add_impl!(Sub, sub, StrictlyPositive, NegativeFinite, f64);
crate::add_impl!(Sub, sub, StrictlyPositive, PositiveFinite, f64);
crate::add_impl!(Sub, sub, StrictlyPositive, StrictlyNegativeFinite, f64);
crate::add_impl!(Sub, sub, StrictlyPositive, StrictlyPositiveFinite, f64);

crate::add_impl!(Sub, sub, StrictlyNegative, NonNaN, f64);
crate::add_impl!(Sub, sub, StrictlyNegative, NonZeroNonNaN, f64);
crate::add_impl!(Sub, sub, StrictlyNegative, Positive, f64);
crate::add_impl!(Sub, sub, StrictlyNegative, Negative, f64);
crate::add_impl!(Sub, sub, StrictlyNegative, StrictlyPositive, f64);
crate::add_impl!(Sub, sub, StrictlyNegative, StrictlyNegative, f64);
crate::add_impl!(Sub, sub, StrictlyNegative, NegativeFinite, f64);
crate::add_impl!(Sub, sub, StrictlyNegative, PositiveFinite, f64);
crate::add_impl!(Sub, sub, StrictlyNegative, StrictlyNegativeFinite, f64);
crate::add_impl!(Sub, sub, StrictlyNegative, StrictlyPositiveFinite, f64);

crate::add_impl!(Sub, sub, NegativeFinite, NonNaN, f64);
crate::add_impl!(Sub, sub, NegativeFinite, NonZeroNonNaN, f64);
crate::add_impl!(Sub, sub, NegativeFinite, Positive, f64);
crate::add_impl!(Sub, sub, NegativeFinite, Negative, f64);
crate::add_impl!(Sub, sub, NegativeFinite, StrictlyPositive, f64);
crate::add_impl!(Sub, sub, NegativeFinite, StrictlyNegative, f64);
crate::add_impl!(Sub, sub, NegativeFinite, NegativeFinite, f64);
crate::add_impl!(Sub, sub, NegativeFinite, PositiveFinite, f64);
crate::add_impl!(Sub, sub, NegativeFinite, StrictlyNegativeFinite, f64);
crate::add_impl!(Sub, sub, NegativeFinite, StrictlyPositiveFinite, f64);

crate::add_impl!(Sub, sub, PositiveFinite, NonNaN, f64);
crate::add_impl!(Sub, sub, PositiveFinite, NonZeroNonNaN, f64);
crate::add_impl!(Sub, sub, PositiveFinite, Positive, f64);
crate::add_impl!(Sub, sub, PositiveFinite, Negative, f64);
crate::add_impl!(Sub, sub, PositiveFinite, StrictlyPositive, f64);
crate::add_impl!(Sub, sub, PositiveFinite, StrictlyNegative, f64);
crate::add_impl!(Sub, sub, PositiveFinite, NegativeFinite, f64);
crate::add_impl!(Sub, sub, PositiveFinite, PositiveFinite, f64);
crate::add_impl!(Sub, sub, PositiveFinite, StrictlyNegativeFinite, f64);
crate::add_impl!(Sub, sub, PositiveFinite, StrictlyPositiveFinite, f64);

crate::add_impl!(Sub, sub, StrictlyNegativeFinite, NonNaN, f64);
crate::add_impl!(Sub, sub, StrictlyNegativeFinite, NonZeroNonNaN, f64);
crate::add_impl!(Sub, sub, StrictlyNegativeFinite, Positive, f64);
crate::add_impl!(Sub, sub, StrictlyNegativeFinite, Negative, f64);
crate::add_impl!(Sub, sub, StrictlyNegativeFinite, StrictlyPositive, f64);
crate::add_impl!(Sub, sub, StrictlyNegativeFinite, StrictlyNegative, f64);
crate::add_impl!(Sub, sub, StrictlyNegativeFinite, NegativeFinite, f64);
crate::add_impl!(Sub, sub, StrictlyNegativeFinite, PositiveFinite, f64);
crate::add_impl!(
    Sub,
    sub,
    StrictlyNegativeFinite,
    StrictlyNegativeFinite,
    f64
);
crate::add_impl!(
    Sub,
    sub,
    StrictlyNegativeFinite,
    StrictlyPositiveFinite,
    f64
);

crate::add_impl!(Sub, sub, StrictlyPositiveFinite, NonNaN, f64);
crate::add_impl!(Sub, sub, StrictlyPositiveFinite, NonZeroNonNaN, f64);
crate::add_impl!(Sub, sub, StrictlyPositiveFinite, Positive, f64);
crate::add_impl!(Sub, sub, StrictlyPositiveFinite, Negative, f64);
crate::add_impl!(Sub, sub, StrictlyPositiveFinite, StrictlyPositive, f64);
crate::add_impl!(Sub, sub, StrictlyPositiveFinite, StrictlyNegative, f64);
crate::add_impl!(Sub, sub, StrictlyPositiveFinite, NegativeFinite, f64);
crate::add_impl!(Sub, sub, StrictlyPositiveFinite, PositiveFinite, f64);
crate::add_impl!(
    Sub,
    sub,
    StrictlyPositiveFinite,
    StrictlyNegativeFinite,
    f64
);
crate::add_impl!(
    Sub,
    sub,
    StrictlyPositiveFinite,
    StrictlyPositiveFinite,
    f64
);
