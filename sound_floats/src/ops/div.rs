use core::ops::Div;

use crate::types::{
    Negative, NegativeFinite, NonNaN, NonZeroNonNaN, Positive, PositiveFinite, StrictlyNegative,
    StrictlyNegativeFinite, StrictlyPositive, StrictlyPositiveFinite,
};

crate::add_impl!(Div, div, NonNaN, NonNaN, f64);
crate::add_impl!(Div, div, NonNaN, NonZeroNonNaN, f64);
crate::add_impl!(Div, div, NonNaN, Positive, f64);
crate::add_impl!(Div, div, NonNaN, Negative, f64);
crate::add_impl!(Div, div, NonNaN, StrictlyPositive, f64);
crate::add_impl!(Div, div, NonNaN, StrictlyNegative, f64);
crate::add_impl!(Div, div, NonNaN, NegativeFinite, f64);
crate::add_impl!(Div, div, NonNaN, PositiveFinite, f64);
crate::add_impl!(Div, div, NonNaN, StrictlyNegativeFinite, f64);
crate::add_impl!(Div, div, NonNaN, StrictlyPositiveFinite, f64);

crate::add_impl!(Div, div, NonZeroNonNaN, NonNaN, f64);
crate::add_impl!(Div, div, NonZeroNonNaN, NonZeroNonNaN, f64);
crate::add_impl!(Div, div, NonZeroNonNaN, Positive, f64);
crate::add_impl!(Div, div, NonZeroNonNaN, Negative, f64);
crate::add_impl!(Div, div, NonZeroNonNaN, StrictlyPositive, f64);
crate::add_impl!(Div, div, NonZeroNonNaN, StrictlyNegative, f64);
crate::add_impl!(Div, div, NonZeroNonNaN, NegativeFinite, f64);
crate::add_impl!(Div, div, NonZeroNonNaN, PositiveFinite, f64);
crate::add_impl!(Div, div, NonZeroNonNaN, StrictlyNegativeFinite, f64);
crate::add_impl!(Div, div, NonZeroNonNaN, StrictlyPositiveFinite, f64);

crate::add_impl!(Div, div, Positive, NonNaN, f64);
crate::add_impl!(Div, div, Positive, NonZeroNonNaN, f64);
crate::add_impl!(Div, div, Positive, Positive, f64);
crate::add_impl!(Div, div, Positive, Negative, f64);
crate::add_impl!(Div, div, Positive, StrictlyPositive, f64);
crate::add_impl!(Div, div, Positive, StrictlyNegative, f64);
crate::add_impl!(Div, div, Positive, NegativeFinite, f64);
crate::add_impl!(Div, div, Positive, PositiveFinite, f64);
crate::add_impl!(Div, div, Positive, StrictlyNegativeFinite, f64);
crate::add_impl!(Div, div, Positive, StrictlyPositiveFinite, f64);

crate::add_impl!(Div, div, Negative, NonNaN, f64);
crate::add_impl!(Div, div, Negative, NonZeroNonNaN, f64);
crate::add_impl!(Div, div, Negative, Positive, f64);
crate::add_impl!(Div, div, Negative, Negative, f64);
crate::add_impl!(Div, div, Negative, StrictlyPositive, f64);
crate::add_impl!(Div, div, Negative, StrictlyNegative, f64);
crate::add_impl!(Div, div, Negative, NegativeFinite, f64);
crate::add_impl!(Div, div, Negative, PositiveFinite, f64);
crate::add_impl!(Div, div, Negative, StrictlyNegativeFinite, f64);
crate::add_impl!(Div, div, Negative, StrictlyPositiveFinite, f64);

crate::add_impl!(Div, div, StrictlyPositive, NonNaN, f64);
crate::add_impl!(Div, div, StrictlyPositive, NonZeroNonNaN, f64);
crate::add_impl!(Div, div, StrictlyPositive, Positive, f64);
crate::add_impl!(Div, div, StrictlyPositive, Negative, f64);
crate::add_impl!(Div, div, StrictlyPositive, StrictlyPositive, f64);
crate::add_impl!(Div, div, StrictlyPositive, StrictlyNegative, f64);
crate::add_impl!(Div, div, StrictlyPositive, NegativeFinite, f64);
crate::add_impl!(Div, div, StrictlyPositive, PositiveFinite, f64);
crate::add_impl!(Div, div, StrictlyPositive, StrictlyNegativeFinite, f64);
crate::add_impl!(Div, div, StrictlyPositive, StrictlyPositiveFinite, f64);

crate::add_impl!(Div, div, StrictlyNegative, NonNaN, f64);
crate::add_impl!(Div, div, StrictlyNegative, NonZeroNonNaN, f64);
crate::add_impl!(Div, div, StrictlyNegative, Positive, f64);
crate::add_impl!(Div, div, StrictlyNegative, Negative, f64);
crate::add_impl!(Div, div, StrictlyNegative, StrictlyPositive, f64);
crate::add_impl!(Div, div, StrictlyNegative, StrictlyNegative, f64);
crate::add_impl!(Div, div, StrictlyNegative, NegativeFinite, f64);
crate::add_impl!(Div, div, StrictlyNegative, PositiveFinite, f64);
crate::add_impl!(Div, div, StrictlyNegative, StrictlyNegativeFinite, f64);
crate::add_impl!(Div, div, StrictlyNegative, StrictlyPositiveFinite, f64);

crate::add_impl!(Div, div, NegativeFinite, NonNaN, f64);
crate::add_impl!(Div, div, NegativeFinite, NonZeroNonNaN, f64);
crate::add_impl!(Div, div, NegativeFinite, Positive, f64);
crate::add_impl!(Div, div, NegativeFinite, Negative, f64);
crate::add_impl!(Div, div, NegativeFinite, StrictlyPositive, f64);
crate::add_impl!(Div, div, NegativeFinite, StrictlyNegative, f64);
crate::add_impl!(Div, div, NegativeFinite, NegativeFinite, f64);
crate::add_impl!(Div, div, NegativeFinite, PositiveFinite, f64);
crate::add_impl!(Div, div, NegativeFinite, StrictlyNegativeFinite, f64);
crate::add_impl!(Div, div, NegativeFinite, StrictlyPositiveFinite, f64);

crate::add_impl!(Div, div, PositiveFinite, NonNaN, f64);
crate::add_impl!(Div, div, PositiveFinite, NonZeroNonNaN, f64);
crate::add_impl!(Div, div, PositiveFinite, Positive, f64);
crate::add_impl!(Div, div, PositiveFinite, Negative, f64);
crate::add_impl!(Div, div, PositiveFinite, StrictlyPositive, f64);
crate::add_impl!(Div, div, PositiveFinite, StrictlyNegative, f64);
crate::add_impl!(Div, div, PositiveFinite, NegativeFinite, f64);
crate::add_impl!(Div, div, PositiveFinite, PositiveFinite, f64);
crate::add_impl!(Div, div, PositiveFinite, StrictlyNegativeFinite, f64);
crate::add_impl!(Div, div, PositiveFinite, StrictlyPositiveFinite, f64);

crate::add_impl!(Div, div, StrictlyNegativeFinite, NonNaN, f64);
crate::add_impl!(Div, div, StrictlyNegativeFinite, NonZeroNonNaN, f64);
crate::add_impl!(Div, div, StrictlyNegativeFinite, Positive, f64);
crate::add_impl!(Div, div, StrictlyNegativeFinite, Negative, f64);
crate::add_impl!(Div, div, StrictlyNegativeFinite, StrictlyPositive, f64);
crate::add_impl!(Div, div, StrictlyNegativeFinite, StrictlyNegative, f64);
crate::add_impl!(Div, div, StrictlyNegativeFinite, NegativeFinite, f64);
crate::add_impl!(Div, div, StrictlyNegativeFinite, PositiveFinite, f64);
crate::add_impl!(
    Div,
    div,
    StrictlyNegativeFinite,
    StrictlyNegativeFinite,
    f64
);
crate::add_impl!(
    Div,
    div,
    StrictlyNegativeFinite,
    StrictlyPositiveFinite,
    f64
);

crate::add_impl!(Div, div, StrictlyPositiveFinite, NonNaN, f64);
crate::add_impl!(Div, div, StrictlyPositiveFinite, NonZeroNonNaN, f64);
crate::add_impl!(Div, div, StrictlyPositiveFinite, Positive, f64);
crate::add_impl!(Div, div, StrictlyPositiveFinite, Negative, f64);
crate::add_impl!(Div, div, StrictlyPositiveFinite, StrictlyPositive, f64);
crate::add_impl!(Div, div, StrictlyPositiveFinite, StrictlyNegative, f64);
crate::add_impl!(Div, div, StrictlyPositiveFinite, NegativeFinite, f64);
crate::add_impl!(Div, div, StrictlyPositiveFinite, PositiveFinite, f64);
crate::add_impl!(
    Div,
    div,
    StrictlyPositiveFinite,
    StrictlyNegativeFinite,
    f64
);
crate::add_impl!(
    Div,
    div,
    StrictlyPositiveFinite,
    StrictlyPositiveFinite,
    f64
);
