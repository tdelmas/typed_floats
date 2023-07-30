This crate helps you to ensure the kind of floats you are using.

zero overhead: everything is checked at compile time.
(only `try_from` adds a little overhead at runtime)

`NaN` is rejected by all types.

The types provided by this crate are:

- [`NonNaN`],[`NonNaNFinite`], [`NonZeroNonNaN`], [`NonZeroNonNaNFinite`]

And their positive and negative counterparts:

- [`Positive`],[`PositiveFinite`], [`StrictlyPositive`], [`StrictlyPositiveFinite`]
- [`Negative`],[`NegativeFinite`], [`StrictlyNegative`], [`StrictlyNegativeFinite`]

By default all types are [`core::f64`] but all can use [`core::f32`] (e.g. like [`Positive<f32>`]).

The following conversions are implemented:

- Between all the types of this crate (of the same kind, [`core::f32`] or [`core::f64`])
- From [`core::f32`] and [`core::f64`]
- From integers types (exept [`core::u128`] and [`core::i128`])
- From `NonZero*` ([`NonZeroU8`], [`NonZeroU16`], [`NonZeroU32`], [`NonZeroU64`], [`NonZeroI8`], [`NonZeroI16`], [`NonZeroI32`], [`NonZeroI64`])

(The trait `From` or `TryFrom` is implemented depending on the situation. Impossible conversions are not implemented.)

# When to use it

## When handling floats

When you handle floats, this crate can help you to ensure that you are not using `NaN` or `Infinity` by mistake. Methods and functions implemented returns a type as strict as possible, so you know when you really have to check for `NaN` or `Infinity`.

## When writing a library

Using one of the type provided by this crate in your public API can help your users to avoid mistakes and limits the checks your functions have to do.

It also helps to make API simpler as you don't have to handle and document all the possible cases with `NaN` and `Infinity` for example.

E.g. the following function:

```rust
fn fast_inv_sqrt(x: StrictlyPositiveFinite) -> StrictlyPositive;
```

Ensures:
- For the implementer: the parameter `x` is neither `NaN` nor `Infinity`, and is strictly positive
- For the user: the result is not `NaN` and is strictly positive but may be `Infinity`

It that example:
- the implementer doesn't have to check for `NaN`, `Infinity`, or `<= 0` for the parameter `x`
- the user only have to check the result for `Infinity` if they want to handle it differently and can't call the function with an invalid parameter.

# Full documentation

Is on [docs.rs](https://docs.rs/typed_floats).

[`core::f32`]: https://doc.rust-lang.org/core/primitive.f32.html "`f32`"
[`core::f64`]: https://doc.rust-lang.org/core/primitive.f64.html "`f64`"
[`core::u128`]: https://doc.rust-lang.org/core/primitive.u128.html "`u128`"
[`core::i128`]: https://doc.rust-lang.org/core/primitive.i128.html "`i128`"
[`NonNaN`]: https://docs.rs/typed_floats/latest/typed_floats/struct.NonNaN.html
[`NonNaNFinite`]: https://docs.rs/typed_floats/latest/typed_floats/struct.NonNaNFinite.html
[`NonZeroNonNaN`]: https://docs.rs/typed_floats/latest/typed_floats/struct.NonZeroNonNaN.html
[`NonZeroNonNaNFinite`]: https://docs.rs/typed_floats/latest/typed_floats/struct.NonZeroNonNaNFinite.html
[`Positive`]: https://docs.rs/typed_floats/latest/typed_floats/struct.Positive.html
[`PositiveFinite`]: https://docs.rs/typed_floats/latest/typed_floats/struct.PositiveFinite.html
[`StrictlyPositive`]: https://docs.rs/typed_floats/latest/typed_floats/struct.StrictlyPositive.html
[`StrictlyPositiveFinite`]: https://docs.rs/typed_floats/latest/typed_floats/struct.StrictlyPositiveFinite.html
[`Negative`]: https://docs.rs/typed_floats/latest/typed_floats/struct.Negative.html
[`NegativeFinite`]: https://docs.rs/typed_floats/latest/typed_floats/struct.NegativeFinite.html
[`StrictlyNegative`]: https://docs.rs/typed_floats/latest/typed_floats/struct.StrictlyNegative.html
[`StrictlyNegativeFinite`]: https://docs.rs/typed_floats/latest/typed_floats/struct.StrictlyNegativeFinite.html
[`Positive<f32>`]: https://docs.rs/typed_floats/latest/typed_floats/type.Positive.html
[`NonZeroU8`]: https://doc.rust-lang.org/core/num/struct.NonZeroU8.html
[`NonZeroU16`]: https://doc.rust-lang.org/core/num/struct.NonZeroU16.html
[`NonZeroU32`]: https://doc.rust-lang.org/core/num/struct.NonZeroU32.html
[`NonZeroU64`]: https://doc.rust-lang.org/core/num/struct.NonZeroU64.html
[`NonZeroI8`]: https://doc.rust-lang.org/core/num/struct.NonZeroI8.html
[`NonZeroI16`]: https://doc.rust-lang.org/core/num/struct.NonZeroI16.html
[`NonZeroI32`]: https://doc.rust-lang.org/core/num/struct.NonZeroI32.html
[`NonZeroI64`]: https://doc.rust-lang.org/core/num/struct.NonZeroI64.html
