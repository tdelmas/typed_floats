This crate helps you to ensure the kind of floats you are using.

zero overhead: everything is checked at compile time.
(only `try_from` adds a little overhead at runtime)

`NaN` is rejected by all types.

The types provided by this crate are:
- [`NonNaN`],[`NonNaNFinite`], [`NonZeroNonNaN`], [`NonZeroNonNaNFinite`]

And their positive and negative counterparts:
- [`Positive`],[`PositiveFinite`], [`StrictlyPositive`], [`StrictlyPositiveFinite`]
- [`Negative`],[`NegativeFinite`], [`StrictlyNegative`], [`StrictlyNegativeFinite`]

By default all types are [`f64`] but all can use [`f32`] (e.g. like [`Positive<f32>`]).

The following conversions are implemented:
- Between all the types of this crate
- From [`f64`]
- From integers types (exept `u128` and `i128`)
- From `NonZero*`

[`f64`]: https://doc.rust-lang.org/core/primitive.f64.html