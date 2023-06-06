use core::ops::Add;

use crate::types::{
    Negative, NegativeFinite, NonNaN, NonZeroNonNaN, Positive, PositiveFinite, StrictlyNegative,
    StrictlyNegativeFinite, StrictlyPositive, StrictlyPositiveFinite,
};

/*
#[macro_export]
macro_rules! impl_adds! {
    (
        ($namea: ident, $typea:ty, $nana:expr,true, $zeroa:expr, true, $negativea:expr),
        ($nameb: ident, $typeb:ty, $nanb:expr, true, $zerob:expr, $positiveb:expr, true)
    ) => {
        // Caviats:
        // Inf - Inf = NaN
        crate::add_impl!(Add, add, $namea, $nameb, f64);
    };
    (
        ($namea: ident, $typea:ty, $nana:expr,true, $zeroa:expr, $positivea:expr,true),
        ($nameb: ident, $typeb:ty, $nanb:expr, true, $zerob:expr, true, $negativeb:expr)
    ) => {
        // Caviats:
        // Inf - Inf = NaN
        crate::add_impl!(Add, add, $namea, $nameb, f64);
    };
    (
        ($namea: ident, $typea:ty, $nana:expr, $infa:expr, $zeroa:expr, $positivea:expr, $negativea:expr),
        ($nameb: ident, $typeb:ty, $nanb:expr, $infb:expr, $zerob:expr, $positiveb:expr, $negativeb:expr)
    ) => {
        crate::add_impl!(Add, add, $namea, $nameb, NonNaN);
    };
    (
        ($namea: ident, $typea:ty, $nana:expr, $infa:expr, $zeroa:expr, $positivea:expr, $negativea:expr),
        ($nameb: ident, $typeb:ty, $nanb:expr, $infb:expr, $zerob:expr, $positiveb:expr, $negativeb:expr)
    ) => {
    };
    (
        ($namea: ident, $typea:ty, $nana:expr, $infa:expr, $zeroa:expr, $positivea:expr, $negativea:expr),
        ($nameb: ident, $typeb:ty, $nanb:expr, $infb:expr, $zerob:expr, $positiveb:expr, $negativeb:expr)
    ) => {
    };
}*/

crate::add_impl!(Add, add, NonNaN, NonNaN, f64);
crate::add_impl!(Add, add, NonNaN, NonZeroNonNaN, f64);
crate::add_impl!(Add, add, NonNaN, Positive, f64);
crate::add_impl!(Add, add, NonNaN, Negative, f64);
crate::add_impl!(Add, add, NonNaN, StrictlyPositive, f64);
crate::add_impl!(Add, add, NonNaN, StrictlyNegative, f64);
crate::add_impl!(Add, add, NonNaN, NegativeFinite, NonNaN);
crate::add_impl!(Add, add, NonNaN, PositiveFinite, NonNaN);
crate::add_impl!(Add, add, NonNaN, StrictlyNegativeFinite, NonNaN);
crate::add_impl!(Add, add, NonNaN, StrictlyPositiveFinite, NonNaN);

crate::add_impl!(Add, add, NonZeroNonNaN, NonNaN, f64);
crate::add_impl!(Add, add, NonZeroNonNaN, NonZeroNonNaN, f64);
crate::add_impl!(Add, add, NonZeroNonNaN, Positive, f64);
crate::add_impl!(Add, add, NonZeroNonNaN, Negative, f64);
crate::add_impl!(Add, add, NonZeroNonNaN, StrictlyPositive, f64);
crate::add_impl!(Add, add, NonZeroNonNaN, StrictlyNegative, f64);
crate::add_impl!(Add, add, NonZeroNonNaN, NegativeFinite, NonNaN);
crate::add_impl!(Add, add, NonZeroNonNaN, PositiveFinite, NonNaN);
crate::add_impl!(Add, add, NonZeroNonNaN, StrictlyNegativeFinite, NonNaN);
crate::add_impl!(Add, add, NonZeroNonNaN, StrictlyPositiveFinite, NonNaN);

crate::add_impl!(Add, add, Positive, NonNaN, f64);
crate::add_impl!(Add, add, Positive, NonZeroNonNaN, NonZeroNonNaN);
crate::add_impl!(Add, add, Positive, Positive, Positive);
crate::add_impl!(Add, add, Positive, Negative, f64);
crate::add_impl!(Add, add, Positive, StrictlyPositive, StrictlyPositive);
crate::add_impl!(Add, add, Positive, StrictlyNegative, f64);
crate::add_impl!(Add, add, Positive, NegativeFinite, NonNaN);
crate::add_impl!(Add, add, Positive, PositiveFinite, Positive);
crate::add_impl!(Add, add, Positive, StrictlyNegativeFinite, NonNaN);
crate::add_impl!(Add, add, Positive, StrictlyPositiveFinite, StrictlyPositive);

crate::add_impl!(Add, add, Negative, NonNaN, f64);
crate::add_impl!(Add, add, Negative, NonZeroNonNaN, f64);
crate::add_impl!(Add, add, Negative, Positive, f64);
crate::add_impl!(Add, add, Negative, Negative, f64);
crate::add_impl!(Add, add, Negative, StrictlyPositive, f64);
crate::add_impl!(Add, add, Negative, StrictlyNegative, f64);
crate::add_impl!(Add, add, Negative, NegativeFinite, f64);
crate::add_impl!(Add, add, Negative, PositiveFinite, f64);
crate::add_impl!(Add, add, Negative, StrictlyNegativeFinite, f64);
crate::add_impl!(Add, add, Negative, StrictlyPositiveFinite, f64);

crate::add_impl!(Add, add, StrictlyPositive, NonNaN, f64);
crate::add_impl!(Add, add, StrictlyPositive, NonZeroNonNaN, f64);
crate::add_impl!(Add, add, StrictlyPositive, Positive, StrictlyPositive);
crate::add_impl!(Add, add, StrictlyPositive, Negative, f64);
crate::add_impl!(Add, add, StrictlyPositive, StrictlyPositive, f64);
crate::add_impl!(Add, add, StrictlyPositive, StrictlyNegative, f64);
crate::add_impl!(Add, add, StrictlyPositive, NegativeFinite, f64);
crate::add_impl!(Add, add, StrictlyPositive, PositiveFinite, f64);
crate::add_impl!(Add, add, StrictlyPositive, StrictlyNegativeFinite, f64);
crate::add_impl!(Add, add, StrictlyPositive, StrictlyPositiveFinite, f64);

crate::add_impl!(Add, add, StrictlyNegative, NonNaN, f64);
crate::add_impl!(Add, add, StrictlyNegative, NonZeroNonNaN, f64);
crate::add_impl!(Add, add, StrictlyNegative, Positive, f64);
crate::add_impl!(Add, add, StrictlyNegative, Negative, f64);
crate::add_impl!(Add, add, StrictlyNegative, StrictlyPositive, f64);
crate::add_impl!(Add, add, StrictlyNegative, StrictlyNegative, f64);
crate::add_impl!(Add, add, StrictlyNegative, NegativeFinite, f64);
crate::add_impl!(Add, add, StrictlyNegative, PositiveFinite, f64);
crate::add_impl!(Add, add, StrictlyNegative, StrictlyNegativeFinite, f64);
crate::add_impl!(Add, add, StrictlyNegative, StrictlyPositiveFinite, f64);

crate::add_impl!(Add, add, NegativeFinite, NonNaN, NonNaN);
crate::add_impl!(Add, add, NegativeFinite, NonZeroNonNaN, NonNaN);
crate::add_impl!(Add, add, NegativeFinite, Positive, NonNaN);
crate::add_impl!(Add, add, NegativeFinite, Negative, f64);
crate::add_impl!(Add, add, NegativeFinite, StrictlyPositive, f64);
crate::add_impl!(Add, add, NegativeFinite, StrictlyNegative, f64);
crate::add_impl!(Add, add, NegativeFinite, NegativeFinite, f64);
crate::add_impl!(Add, add, NegativeFinite, PositiveFinite, f64);
crate::add_impl!(Add, add, NegativeFinite, StrictlyNegativeFinite, f64);
crate::add_impl!(Add, add, NegativeFinite, StrictlyPositiveFinite, f64);

crate::add_impl!(Add, add, PositiveFinite, NonNaN, NonNaN);
crate::add_impl!(Add, add, PositiveFinite, NonZeroNonNaN, NonNaN);
crate::add_impl!(Add, add, PositiveFinite, Positive, Positive);
crate::add_impl!(Add, add, PositiveFinite, Negative, f64);
crate::add_impl!(Add, add, PositiveFinite, StrictlyPositive, f64);
crate::add_impl!(Add, add, PositiveFinite, StrictlyNegative, f64);
crate::add_impl!(Add, add, PositiveFinite, NegativeFinite, f64);
crate::add_impl!(Add, add, PositiveFinite, PositiveFinite, f64);
crate::add_impl!(Add, add, PositiveFinite, StrictlyNegativeFinite, f64);
crate::add_impl!(Add, add, PositiveFinite, StrictlyPositiveFinite, f64);

crate::add_impl!(Add, add, StrictlyNegativeFinite, NonNaN, NonNaN);
crate::add_impl!(Add, add, StrictlyNegativeFinite, NonZeroNonNaN, NonNaN);
crate::add_impl!(Add, add, StrictlyNegativeFinite, Positive, NonNaN);
crate::add_impl!(Add, add, StrictlyNegativeFinite, Negative, f64);
crate::add_impl!(Add, add, StrictlyNegativeFinite, StrictlyPositive, f64);
crate::add_impl!(Add, add, StrictlyNegativeFinite, StrictlyNegative, f64);
crate::add_impl!(Add, add, StrictlyNegativeFinite, NegativeFinite, f64);
crate::add_impl!(Add, add, StrictlyNegativeFinite, PositiveFinite, f64);
crate::add_impl!(
    Add,
    add,
    StrictlyNegativeFinite,
    StrictlyNegativeFinite,
    f64
);
crate::add_impl!(
    Add,
    add,
    StrictlyNegativeFinite,
    StrictlyPositiveFinite,
    f64
);

crate::add_impl!(Add, add, StrictlyPositiveFinite, NonNaN, NonNaN);
crate::add_impl!(Add, add, StrictlyPositiveFinite, NonZeroNonNaN, NonNaN);
crate::add_impl!(Add, add, StrictlyPositiveFinite, Positive, StrictlyPositive);
crate::add_impl!(Add, add, StrictlyPositiveFinite, Negative, f64);
crate::add_impl!(Add, add, StrictlyPositiveFinite, StrictlyPositive, f64);
crate::add_impl!(Add, add, StrictlyPositiveFinite, StrictlyNegative, f64);
crate::add_impl!(Add, add, StrictlyPositiveFinite, NegativeFinite, f64);
crate::add_impl!(Add, add, StrictlyPositiveFinite, PositiveFinite, f64);
crate::add_impl!(
    Add,
    add,
    StrictlyPositiveFinite,
    StrictlyNegativeFinite,
    f64
);
crate::add_impl!(
    Add,
    add,
    StrictlyPositiveFinite,
    StrictlyPositiveFinite,
    f64
);
