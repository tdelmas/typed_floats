use core::ops::Mul;

use crate::types::{
    Negative, NegativeFinite, NonNaN, NonZeroNonNaN, Positive, PositiveFinite, StrictlyNegative,
    StrictlyNegativeFinite, StrictlyPositive, StrictlyPositiveFinite,
};

crate::add_impl!(Mul, mul, NonNaN, NonNaN, f64);
crate::add_impl!(Mul, mul, NonNaN, NonZeroNonNaN, f64);
crate::add_impl!(Mul, mul, NonNaN, Positive, f64);
crate::add_impl!(Mul, mul, NonNaN, Negative, f64);
crate::add_impl!(Mul, mul, NonNaN, StrictlyPositive, f64);
crate::add_impl!(Mul, mul, NonNaN, StrictlyNegative, f64);
crate::add_impl!(Mul, mul, NonNaN, NegativeFinite, f64);
crate::add_impl!(Mul, mul, NonNaN, PositiveFinite, f64);
crate::add_impl!(Mul, mul, NonNaN, StrictlyNegativeFinite, f64);
crate::add_impl!(Mul, mul, NonNaN, StrictlyPositiveFinite, f64);

crate::add_impl!(Mul, mul, NonZeroNonNaN, NonNaN, f64);
crate::add_impl!(Mul, mul, NonZeroNonNaN, NonZeroNonNaN, f64);
crate::add_impl!(Mul, mul, NonZeroNonNaN, Positive, f64);
crate::add_impl!(Mul, mul, NonZeroNonNaN, Negative, f64);
crate::add_impl!(Mul, mul, NonZeroNonNaN, StrictlyPositive, f64);
crate::add_impl!(Mul, mul, NonZeroNonNaN, StrictlyNegative, f64);
crate::add_impl!(Mul, mul, NonZeroNonNaN, NegativeFinite, f64);
crate::add_impl!(Mul, mul, NonZeroNonNaN, PositiveFinite, f64);
crate::add_impl!(Mul, mul, NonZeroNonNaN, StrictlyNegativeFinite, f64);
crate::add_impl!(Mul, mul, NonZeroNonNaN, StrictlyPositiveFinite, f64);

crate::add_impl!(Mul, mul, Positive, NonNaN, f64);
crate::add_impl!(Mul, mul, Positive, NonZeroNonNaN, f64);
crate::add_impl!(Mul, mul, Positive, Positive, f64);
crate::add_impl!(Mul, mul, Positive, Negative, f64);
crate::add_impl!(Mul, mul, Positive, StrictlyPositive, f64);
crate::add_impl!(Mul, mul, Positive, StrictlyNegative, f64);
crate::add_impl!(Mul, mul, Positive, NegativeFinite, f64);
crate::add_impl!(Mul, mul, Positive, PositiveFinite, f64);
crate::add_impl!(Mul, mul, Positive, StrictlyNegativeFinite, f64);
crate::add_impl!(Mul, mul, Positive, StrictlyPositiveFinite, f64);

crate::add_impl!(Mul, mul, Negative, NonNaN, f64);
crate::add_impl!(Mul, mul, Negative, NonZeroNonNaN, f64);
crate::add_impl!(Mul, mul, Negative, Positive, f64);
crate::add_impl!(Mul, mul, Negative, Negative, f64);
crate::add_impl!(Mul, mul, Negative, StrictlyPositive, f64);
crate::add_impl!(Mul, mul, Negative, StrictlyNegative, f64);
crate::add_impl!(Mul, mul, Negative, NegativeFinite, f64);
crate::add_impl!(Mul, mul, Negative, PositiveFinite, f64);
crate::add_impl!(Mul, mul, Negative, StrictlyNegativeFinite, f64);
crate::add_impl!(Mul, mul, Negative, StrictlyPositiveFinite, f64);

crate::add_impl!(Mul, mul, StrictlyPositive, NonNaN, f64);
crate::add_impl!(Mul, mul, StrictlyPositive, NonZeroNonNaN, f64);
crate::add_impl!(Mul, mul, StrictlyPositive, Positive, f64);
crate::add_impl!(Mul, mul, StrictlyPositive, Negative, f64);
crate::add_impl!(Mul, mul, StrictlyPositive, StrictlyPositive, f64);
crate::add_impl!(Mul, mul, StrictlyPositive, StrictlyNegative, f64);
crate::add_impl!(Mul, mul, StrictlyPositive, NegativeFinite, f64);
crate::add_impl!(Mul, mul, StrictlyPositive, PositiveFinite, f64);
crate::add_impl!(Mul, mul, StrictlyPositive, StrictlyNegativeFinite, f64);
crate::add_impl!(Mul, mul, StrictlyPositive, StrictlyPositiveFinite, f64);

crate::add_impl!(Mul, mul, StrictlyNegative, NonNaN, f64);
crate::add_impl!(Mul, mul, StrictlyNegative, NonZeroNonNaN, f64);
crate::add_impl!(Mul, mul, StrictlyNegative, Positive, f64);
crate::add_impl!(Mul, mul, StrictlyNegative, Negative, f64);
crate::add_impl!(Mul, mul, StrictlyNegative, StrictlyPositive, f64);
crate::add_impl!(Mul, mul, StrictlyNegative, StrictlyNegative, f64);
crate::add_impl!(Mul, mul, StrictlyNegative, NegativeFinite, f64);
crate::add_impl!(Mul, mul, StrictlyNegative, PositiveFinite, f64);
crate::add_impl!(Mul, mul, StrictlyNegative, StrictlyNegativeFinite, f64);
crate::add_impl!(Mul, mul, StrictlyNegative, StrictlyPositiveFinite, f64);

crate::add_impl!(Mul, mul, NegativeFinite, NonNaN, f64);
crate::add_impl!(Mul, mul, NegativeFinite, NonZeroNonNaN, f64);
crate::add_impl!(Mul, mul, NegativeFinite, Positive, f64);
crate::add_impl!(Mul, mul, NegativeFinite, Negative, f64);
crate::add_impl!(Mul, mul, NegativeFinite, StrictlyPositive, f64);
crate::add_impl!(Mul, mul, NegativeFinite, StrictlyNegative, f64);
crate::add_impl!(Mul, mul, NegativeFinite, NegativeFinite, f64);
crate::add_impl!(Mul, mul, NegativeFinite, PositiveFinite, f64);
crate::add_impl!(Mul, mul, NegativeFinite, StrictlyNegativeFinite, f64);
crate::add_impl!(Mul, mul, NegativeFinite, StrictlyPositiveFinite, f64);

crate::add_impl!(Mul, mul, PositiveFinite, NonNaN, f64);
crate::add_impl!(Mul, mul, PositiveFinite, NonZeroNonNaN, f64);
crate::add_impl!(Mul, mul, PositiveFinite, Positive, f64);
crate::add_impl!(Mul, mul, PositiveFinite, Negative, f64);
crate::add_impl!(Mul, mul, PositiveFinite, StrictlyPositive, f64);
crate::add_impl!(Mul, mul, PositiveFinite, StrictlyNegative, f64);
crate::add_impl!(Mul, mul, PositiveFinite, NegativeFinite, f64);
crate::add_impl!(Mul, mul, PositiveFinite, PositiveFinite, f64);
crate::add_impl!(Mul, mul, PositiveFinite, StrictlyNegativeFinite, f64);
crate::add_impl!(Mul, mul, PositiveFinite, StrictlyPositiveFinite, f64);

crate::add_impl!(Mul, mul, StrictlyNegativeFinite, NonNaN, f64);
crate::add_impl!(Mul, mul, StrictlyNegativeFinite, NonZeroNonNaN, f64);
crate::add_impl!(Mul, mul, StrictlyNegativeFinite, Positive, f64);
crate::add_impl!(Mul, mul, StrictlyNegativeFinite, Negative, f64);
crate::add_impl!(Mul, mul, StrictlyNegativeFinite, StrictlyPositive, f64);
crate::add_impl!(Mul, mul, StrictlyNegativeFinite, StrictlyNegative, f64);
crate::add_impl!(Mul, mul, StrictlyNegativeFinite, NegativeFinite, f64);
crate::add_impl!(Mul, mul, StrictlyNegativeFinite, PositiveFinite, f64);
crate::add_impl!(
    Mul,
    mul,
    StrictlyNegativeFinite,
    StrictlyNegativeFinite,
    f64
);
crate::add_impl!(
    Mul,
    mul,
    StrictlyNegativeFinite,
    StrictlyPositiveFinite,
    f64
);

crate::add_impl!(Mul, mul, StrictlyPositiveFinite, NonNaN, f64);
crate::add_impl!(Mul, mul, StrictlyPositiveFinite, NonZeroNonNaN, f64);
crate::add_impl!(Mul, mul, StrictlyPositiveFinite, Positive, f64);
crate::add_impl!(Mul, mul, StrictlyPositiveFinite, Negative, f64);
crate::add_impl!(Mul, mul, StrictlyPositiveFinite, StrictlyPositive, f64);
crate::add_impl!(Mul, mul, StrictlyPositiveFinite, StrictlyNegative, f64);
crate::add_impl!(Mul, mul, StrictlyPositiveFinite, NegativeFinite, f64);
crate::add_impl!(Mul, mul, StrictlyPositiveFinite, PositiveFinite, f64);
crate::add_impl!(
    Mul,
    mul,
    StrictlyPositiveFinite,
    StrictlyNegativeFinite,
    f64
);
crate::add_impl!(
    Mul,
    mul,
    StrictlyPositiveFinite,
    StrictlyPositiveFinite,
    f64
);
