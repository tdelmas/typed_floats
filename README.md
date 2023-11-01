
[![Build Status](https://circleci.com/gh/tdelmas/typed_floats.svg?style=shield)](https://circleci.com/gh/tdelmas/typed_floats)
[![Version](https://img.shields.io/crates/v/typed_floats.svg)](https://crates.io/crates/typed_floats)
[![Documentation](https://docs.rs/typed_floats/badge.svg)](https://docs.rs/typed_floats)
[![License](https://img.shields.io/crates/l/typed_floats.svg)](https://github.com/tdelmas/typed_floats/blob/main/LICENSE)

This crate helps you to ensure the kind of floats you are using.

zero overhead: everything is checked at compile time.
(only `try_from` adds a little overhead at runtime)

`NaN` is rejected by all types.

The types provided by this crate are:

- [`NonNaN`], [`NonNaNFinite`], [`NonZeroNonNaN`], [`NonZeroNonNaNFinite`]

And their positive and negative counterparts:

- [`Positive`],[`PositiveFinite`], [`StrictlyPositive`], [`StrictlyPositiveFinite`]
- [`Negative`],[`NegativeFinite`], [`StrictlyNegative`], [`StrictlyNegativeFinite`]

(Negatives types reject `+0.0` and positives types reject `-0.0`)

To avoid specifying the kind of float (e.g. like [`Positive<f32>`]), you can use the modules [`tf64`] and [`tf32`] which expose aliases.

The following conversions are implemented:

- Between all the types of this crate (of the same kind, [`f32`] or [`f64`])
- From [`f32`] and [`f64`]
- From integers types (except [`u128`] and [`i128`])
- From `NonZero*` ([`NonZeroU8`], [`NonZeroU16`], [`NonZeroU32`], [`NonZeroU64`], [`NonZeroI8`], [`NonZeroI16`], [`NonZeroI32`], [`NonZeroI64`])

(The trait `From` or `TryFrom` is implemented depending on the situation. Impossible conversions - for example between [`Positive`] and [`Negative`] - are not implemented.)

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
- For the person implementing the API: the parameter `x` is neither `NaN` nor `Infinity`, and is strictly positive
- For the user: the result is not `NaN` and is strictly positive but may be `Infinity`

It that example:
- the person implementing the API doesn't have to check for `NaN`, `Infinity`, or `<= 0` for the parameter `x`
- the user only have to check the result for `Infinity` if they want to handle it differently and can't call the function with an invalid parameter.

# API

Most methods and traits available on the underlying type are available on the types of this crate.

Most constants are also available, with the most appropriate `TypedFloat` type (except `NAN` for obvious reasons) in the [`tf64`] and [`tf32`] modules (in [`tf64::consts`] and [`tf32::consts`] respectively when the constant comes from [`core::f64::consts`] or [`core::f32::consts`]). Those modules are named that way to avoid conflicts or confusion with the primitives [`f32`] and [`f64`].

As none of the types of this crate can be `NaN`, the traits [`core::cmp::Ord`] and [`core::cmp::Eq`] are implemented for all of them.

⚠️ Like for primitives [`f32`] and [`f64`],`-0.0 == +0.0` is `true` for all types of this crate. For that reason, [`core::hash::Hash`] is not implemented.
To facilitate comparisons, the methods `is_positive_zero` and `is_negative_zero` are added.

# Methods implemented

All 12 types implement the methods available on [`f32`] and [`f64`] except:

- deprecated and nightly-only methods
- atan2(self, other: f64) -> f64
- sin_cos(self) -> (f64, f64)
- mul_add(self, a: f64, b: f64) -> f64
- powi(self, n: i32) -> f64
- powf(self, n: f64) -> f64
- clamp(self, min: f64, max: f64) -> f64
- LowerExp
- UpperExp
- Product
- Sum
- `to_int_unchecked`
- `to*_bits`
- `from*_bits`

In addition, `Hash` is implemented for all types expect [`NonNaN`] and [`NonNaNFinite`] (because they both accept `0.0` and `-0.0`, which are equal, and that is incompatible with `Hash`).

## Panics

The only method that can `panic!` is the `unsafe` method `new_unchecked` when used in an invalid way.

A `panic!` triggered in any other way is considered a security bug and should be reported.

## Overhead

This crate is designed to have a minimal overhead at runtime, in terms of memory, speed and binary size.

It may even be faster than using primitives [`f32`] and [`f64`] directly, as it may avoids some checks by using compiler hints.

The only methods that adds a little overhead are `try_from` because of the checks they do at runtime, compared to the `unsafe` method `new_unchecked`.

In debug mode, a little overhead is present, both to check the validity of the values and because `inline` may not be respected.

Any other overhead is considered a bug and should be reported.

## How it works

For each operation, at compile time crate determine the most strict type possible for the result.

For example, if you multiply a [`PositiveFinite`] and a [`StrictlyNegativeFinite`], the result will be a [`Negative`].

## Main limitations

- Doesn't work with `no_std` (for now)
- Doesn't implement `serde` serialization/deserialization (yet)
- Doesn't fix the floating point quirks such as `0.0 == -0.0` (because that would introduce a runtime overhead)

## Testing

Tests are run on different architectures on [GitHub actions](https://github.com/tdelmas/typed_floats/actions/workflows/tests.yml) and [CircleCI](https://circleci.com/gh/tdelmas/typed_floats).

A separate crate is handling exaustive testing: `typed_floats_test`. They are not included in the published crates because they use >1GB of disk and take >10 minutes to run.

To run all tests:

```bash
git clone https://github.com/tdelmas/typed_floats
cd typed_floats
cargo test --all
```

# Similar crates

- [checked-float](https://crates.io/crates/checked-float) A crate for making invariant-enforcing floating point wrappers
- [decorum](https://crates.io/crates/decorum) Decorum is a Rust library that provides total ordering, equivalence, hashing, and constraints for floating-point representations. Decorum does not require std.
- [eq-float](https://crates.io/crates/eq-float) Float wrappers with a total order (by setting NAN == NAN).
- [fix_float](https://crates.io/crates/fix_float) Fixed floating types that allows useful trait implementations and datastructures on float numbers
- [float-derive](https://crates.io/crates/float-derive) A crate that allows deriving Eq and Hash for types that contain floating points.
- [float-ord](https://crates.io/crates/float-ord) A total ordering for floating-point numbers.
- [nanbox](https://crates.io/crates/nanbox) NaN boxing implementation.
- [noisy_float](https://crates.io/crates/noisy_float) Contains floating point types that panic if they are set to an illegal value, such as NaN.
- [num-order](https://crates.io/crates/num-order) Numerically consistent `Eq`, `Ord` and `Hash` implementations for various `num` types (`u32`, `f64`, `num_bigint::BigInt`, etc.)
- [ordered-float](https://crates.io/crates/ordered-float) Provides several wrapper types for Ord and Eq implementations on f64 and friends.
- [partial-min-max](https://crates.io/crates/partial-min-max) `min` and `max` functions that work with `PartialOrd`.
- [real_float](https://crates.io/crates/real_float) Floating point types that check for correctness and implement total ordering.
- [result_float](https://crates.io/crates/result_float) Floating point type that cannot store NaN.
- [totally-ordered](https://crates.io/crates/totally-ordered) No dependency, no-std totally ordered f32/f64
- [unsigned-f64](https://crates.io/crates/unsigned-f64) A wrapper around f64 that guarantees that the value is always non-negative on the type level.

Features provided/checked by those crates:

✔️: provided, ❌: not provided, ❓: unknown

(you may need to scroll to the right to see all the columns : "Production ready", "Avoid `panic!`", "Minimal overhead", "Eq/Ord", "Hash", "NaN", "Inf", "Zero", "Positive", "Negative")

| Crates           | Production ready | Avoid `panic!` | Minimal overhead | Eq/Ord | Hash | NaN | Inf | Zero | Positive | Negative |
|------------------|------------------|----------------|------------------|--------|------|-----|-----|------|----------|----------|
|**`typed_floats`**| ✔️              | ✔️             | ✔️              | ✔️    | ✔️¹  | ✔️ | ✔️  | ✔️   | ✔️      | ✔️       |
| `checked-float`  | ✔️              | ✔️             | ❌              | ✔️    | ❌   | ✔️²| ✔️² | ✔️²  | ✔️²     | ✔️²      |
| `decorum`        | ✔️              | ❌             | ❌              | ❌    | ❌   | ✔️²| ✔️² | ✔️²  | ✔️²     | ✔️²      |
| `eq-float`       | ❌              | ✔️             | ✔️              | ✔️    | ❌   | ❌ | ❌  | ❌   | ❌      | ❌       |
| `fix_float`      | ✔️              | ✔️             | ✔️              | ✔️    | ✔️   | ✔️ | ✔️  | ❌   | ❌      | ❌       |
| `float-derive`   | ❌              | ❓             | ❓              | ✔️    | ✔️   | ❌ | ❌  | ❌   | ❌      | ❌       |
| `float-ord`      | ✔️              | ✔️             | ❌              | ✔️    | ❌   | ❌ | ❌  | ❌   | ❌      | ❌       |
| `nanbox`         | ❌              | ❓             | ❓              | ❌    | ❌   | ✔️ | ❌  | ❌   | ❌      | ❌       |
| `noisy_float`    | ✔️              | ❌             | ❌              | ❌    | ❌   | ✔️ | ✔️  | ❌   | ❌      | ❌       |
| `num-order`      | ✔️              | ✔️             | ❌              | ✔️    | ✔️   | ❌ | ❌  | ❌   | ❌      | ❌       |
| `ordered-float`  | ✔️              | ❌             | ❌              | ✔️    | ❌   | ✔️ | ❌  | ❌   | ❌      | ❌       |
| `partial-min-max`| ❌              | ✔️             | ✔️              | ❌    | ❌   | ❌ | ❌  | ❌   | ❌      | ❌       |
| `real_float`     | ✔️              | ❌             | ❌              | ✔️    | ❌   | ✔️ | ✔️  | ❌   | ✔️      | ❌       |
| `result_float`   | ✔️              | ✔️             | ❌              | ✔️    | ❌   | ✔️ | ❌  | ❌   | ❌      | ❌       |
| `totally-ordered`| ✔️              | ✔️             | ❌              | ✔️    | ✔️   | ❌ | ❌  | ❌   | ❌      | ❌       |
| `unsigned-f64`   | ❌              | ✔️             | ✔️              | ❌    | ❌   | ❌ | ❌  | ❌   | ✔️      | ❌       |

(N.B. "Production ready" is a subjective measure)

¹: `Hash` is implemented for all types expect [`NonNaN`] and [`NonNaNFinite`] (because they both accept `0.0` and `-0.0`)

²: Can be manually checked

# Full documentation

Is on [docs.rs](https://docs.rs/typed_floats).

[`f32`]: https://doc.rust-lang.org/core/primitive.f32.html
[`f64`]: https://doc.rust-lang.org/core/primitive.f64.html
[`u128`]: https://doc.rust-lang.org/core/primitive.u128.html
[`i128`]: https://doc.rust-lang.org/core/primitive.i128.html
[`core::f32::consts`]: https://doc.rust-lang.org/core/f32/consts/index.html
[`core::f64::consts`]: https://doc.rust-lang.org/core/f64/consts/index.html
[`core::cmp::Ord`]: https://doc.rust-lang.org/core/cmp/trait.Ord.html "`Ord`"
[`core::cmp::Eq`]: https://doc.rust-lang.org/core/cmp/trait.Eq.html "`Eq`"
[`core::hash::Hash`]: https://doc.rust-lang.org/core/hash/trait.Hash.html "`Hash`"
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
[`tf64`]: https://docs.rs/typed_floats/latest/typed_floats/tf64/index.html
[`tf32`]: https://docs.rs/typed_floats/latest/typed_floats/tf32/index.html
[`tf64::consts`]: https://docs.rs/typed_floats/latest/typed_floats/tf64/consts/index.html
[`tf32::consts`]: https://docs.rs/typed_floats/latest/typed_floats/tf32/consts/index.html
[`NonZeroU8`]: https://doc.rust-lang.org/core/num/struct.NonZeroU8.html
[`NonZeroU16`]: https://doc.rust-lang.org/core/num/struct.NonZeroU16.html
[`NonZeroU32`]: https://doc.rust-lang.org/core/num/struct.NonZeroU32.html
[`NonZeroU64`]: https://doc.rust-lang.org/core/num/struct.NonZeroU64.html
[`NonZeroI8`]: https://doc.rust-lang.org/core/num/struct.NonZeroI8.html
[`NonZeroI16`]: https://doc.rust-lang.org/core/num/struct.NonZeroI16.html
[`NonZeroI32`]: https://doc.rust-lang.org/core/num/struct.NonZeroI32.html
[`NonZeroI64`]: https://doc.rust-lang.org/core/num/struct.NonZeroI64.html
