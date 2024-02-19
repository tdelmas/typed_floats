
[![Build Status](https://circleci.com/gh/tdelmas/typed_floats.svg?style=shield)](https://circleci.com/gh/tdelmas/typed_floats)
[![Tests on GitHub CI](https://github.com/tdelmas/typed_floats/workflows/Tests/badge.svg?branch=main)](https://github.com/tdelmas/typed_floats/actions/workflows/tests.yml)
[![Version](https://img.shields.io/crates/v/typed_floats.svg)](https://crates.io/crates/typed_floats)
[![Documentation](https://docs.rs/typed_floats/badge.svg)](https://docs.rs/typed_floats)
[![License](https://img.shields.io/crates/l/typed_floats.svg)](https://github.com/tdelmas/typed_floats/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/tdelmas/typed_floats/status.svg)](https://deps.rs/repo/github/tdelmas/typed_floats)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.70-blue)](https://github.com/rust-lang/rust/releases/tag/1.70.0)
[![Miri](https://github.com/tdelmas/typed_floats/actions/workflows/miri.yml/badge.svg)](https://github.com/tdelmas/typed_floats/actions/workflows/miri.yml)

This crate helps you to ensure the kind of floats you are using, without `panic!` (except if the `unsafe` function `new_unchecked` is used in an unsound way).

zero overhead: everything is checked at compile time.
(only `try_from` adds a little overhead at runtime)

`NaN` is rejected by all types.

The 12 types provided by this crate are:

- [`NonNaN`], [`NonNaNFinite`], [`NonZeroNonNaN`], [`NonZeroNonNaNFinite`]

And their positive and negative counterparts:

- [`Positive`],[`PositiveFinite`], [`StrictlyPositive`], [`StrictlyPositiveFinite`]
- [`Negative`],[`NegativeFinite`], [`StrictlyNegative`], [`StrictlyNegativeFinite`]

(Negatives types reject `+0.0` and positives types reject `-0.0`)

| Type | -∞ | ]-∞; -0.0[ | -0.0 | +0.0 | ]+0.0; +∞[ | +∞ | `NaN` |
|---|---|---|---|---|---|---|---|
| [`NonNaN`] | ✔️ | ✔️ | ✔️ | ✔️  | ✔️ | ✔️ | ❌ |
| [`NonNaNFinite`] | ❌ | ✔️ | ✔️ | ✔️ | ✔️ | ❌ |  ❌ |
| [`NonZeroNonNaN`] | ✔️ | ✔️ | ❌ | ❌ | ✔️ | ✔️ |  ❌ |
| [`NonZeroNonNaNFinite`] | ❌ | ✔️ | ❌ | ❌ | ✔️ | ❌ | ❌ |
| [`Positive`] | ❌ | ❌ | ❌ | ✔️ | ✔️ | ✔️ | ❌ |
| [`PositiveFinite`] | ❌ | ❌ | ❌ | ✔️ | ✔️ | ❌ | ❌ |
| [`StrictlyPositive`] | ❌ | ❌ | ❌ | ❌ | ✔️ | ✔️ | ❌ |
| [`StrictlyPositiveFinite`] | ❌ | ❌ | ❌ | ❌ | ✔️ | ❌ | ❌ |
| [`Negative`] | ✔️ | ✔️ | ✔️ | ❌ | ❌ | ❌ | ❌ |
| [`NegativeFinite`] | ❌ | ✔️ | ✔️ | ❌ | ❌ | ❌ | ❌ |
| [`StrictlyNegative`] | ✔️ | ✔️ | ❌ | ❌ | ❌ | ❌ | ❌ |
| [`StrictlyNegativeFinite`] | ❌ | ✔️ | ❌ | ❌ | ❌ | ❌ | ❌ |

To avoid specifying the kind of float (e.g. like [`Positive<f32>`]), you can use the modules [`tf64`] and [`tf32`] which expose aliases.

The following conversions are implemented:

- Between all the types of this crate (of the same kind, [`f32`] or [`f64`])
- From [`f32`] and [`f64`]
- From integers types (except [`u128`] and [`i128`])
- From `NonZero*` ([`core::num::NonZeroU8`], [`core::num::NonZeroU16`], [`core::num::NonZeroU32`], [`core::num::NonZeroU64`], [`core::num::NonZeroI8`], [`core::num::NonZeroI16`], [`core::num::NonZeroI32`], [`core::num::NonZeroI64`])

(The traits `From` and `TryFrom` are implemented depending on the situation. Impossible conversions - for example between [`Positive`] and [`Negative`] - are not implemented.)

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

It ensures:
- For the person implementing the API: the parameter `x` is neither `NaN` nor `Infinity`, and is strictly positive
- For the user: the result is not `NaN` and is strictly positive but may be `Infinity`

In that example:
- the person implementing the API doesn't have to check for `NaN`, `Infinity`, or `<= 0` for the parameter `x`
- the user only have to check the result for `Infinity` if they want to handle it differently and can't call the function with an invalid parameter.

# API

Most methods and traits available on the underlying type are available on the types of this crate.

Most constants are also available, with the most appropriate `TypedFloat` type (except `NAN` for obvious reasons) in the [`tf64`] and [`tf32`] modules (in [`tf64::consts`] and [`tf32::consts`] respectively when the constant comes from [`core::f64::consts`] or [`core::f32::consts`]). Those modules are named that way to avoid conflicts or confusion with the primitives [`f32`] and [`f64`].

⚠️ Like for primitives [`f32`] and [`f64`],`-0.0 == +0.0` is `true` for all types of this crate.
To facilitate comparisons, the methods `is_positive_zero` and `is_negative_zero` are added.

# Traits implemented

## On all 12 types

As none of the types of this crate can be `NaN`, the following traits are implemented on all types:

- [`core::cmp::Ord`]
- [`core::cmp::Eq`]
- [`core::hash::Hash`] 

Note: for [`core::hash::Hash`] on [`NonNaN`] and [`NonNaNFinite`] there is a (small) overhead because they both accept `0.0` and `-0.0`, which are equal so they must `core::hash::Hash` to the same value.

## Only on some types

- [`core::default::Default`]:
  - with the value `0.0` for [`NonNaN`], [`NonNaNFinite`], [`Positive`], and [`PositiveFinite`].
  - with the value `-0.0` for  [`Negative`], and [`NegativeFinite`].

# Methods implemented

All 12 types implement the methods available on [`f32`] and [`f64`] **except**:

- deprecated and nightly-only methods
- total_cmp(&self, other: &f64) -> Ordering
- sin_cos(self) -> (f64, f64)
- mul_add(self, a: f64, b: f64) -> f64
- clamp(self, min: f64, max: f64) -> f64
- LowerExp
- UpperExp
- Product
- Sum
- `to_int_unchecked`
- `to*_bits`
- `from*_bits`

## Panics

The only method that can `panic!` is the `unsafe` method `new_unchecked` when used in an invalid way.

A `panic!` triggered in any other way is considered a security bug and should be reported.

## Minimal overhead

This crate is designed to have a minimal overhead at runtime, in terms of memory, speed and binary size.

It may even be faster than using primitives [`f32`] and [`f64`] directly, as it may avoids some checks by using compiler hints.

The only methods that adds a little overhead are `try_from` because of the checks they do at runtime, compared to the `unsafe` method `new_unchecked`.

In debug mode, a little overhead is present, both to check the validity of the values and because `inline` may not be respected.

Any other overhead is considered a bug and should be reported.

# Features

- `std`: enabled by default, gives all `f32` and `f64` methods.
- `serde`: implements `Serialize` and `Deserialize` for all `NonNaN` types.
- `libm`: use the `Float` trait from `num-traits` and `libm` to implement the missing methods when the `std` feature is disabled. Side effect: `f32` and `f64` will implement `Float` from `num-traits`. When both `std` and `libm` features are enabled, the `std` implementation is used.

## How it works

For each operation, at compile time crate determine the most strict type possible for the result.

For example, if you multiply a [`PositiveFinite`] and a [`StrictlyNegativeFinite`], the result will be a [`Negative`].

Methods that takes another float as parameter will also return the most strict type possible depending on the both types. For the methods where a trait is not available to specify the return type depending on the parameter type, a new trait is created: 
[`Hypot`], [`Min`], [`Max`], [`Copysign`], [`DivEuclid`] and [`Atan2`].

## Main limitations

- Doesn't fix the floating point quirks such as `0.0 == -0.0`
- Doesn't fix the odd methods such as:
  - `sqrt(-0.0)` returning `-0.0` instead of `NaN`
  - `min(-0.0, 0.0)` returning `-0.0` instead of `0.0` (same for `max`)
  - `frac(-0.0)` returning `0.0` instead of `-0.0`

Because that would introduce a runtime overhead and may introduce some incompatibilities with existing code.

## Rust version

This crate is tested when a new version is release with:
- Rust beta
- Rust stable
- Rust 1.70.0

Also, tests on `nightly`, `beta` and `stable` are run weekly on [GitHub actions](https://github.com/tdelmas/typed_floats/actions/workflows/weekly-tests.yml).

The minimum supported Rust version is 1.70.0 because of the use of `dep:` in `Cargo.toml`.

## Testing

Tests are run on different architectures on [GitHub actions](https://github.com/tdelmas/typed_floats/actions/workflows/tests.yml) and [CircleCI](https://circleci.com/gh/tdelmas/typed_floats).

To run all tests:

```bash
git clone https://github.com/tdelmas/typed_floats

# generate the published documentation, including some tests
cargo xtask pre-build

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
- [num-order](https://crates.io/crates/num-order) Numerically consistent `core::cmp::Eq`, `core::cmp::Ord` and `core::hash::Hash` implementations for various `num` types (`u32`, `f64`, `num_bigint::BigInt`, etc.)
- [ordered-float](https://crates.io/crates/ordered-float) Provides several wrapper types for Ord and Eq implementations on f64 and friends.
- [partial-min-max](https://crates.io/crates/partial-min-max) `min` and `max` functions that work with `PartialOrd`.
- [real_float](https://crates.io/crates/real_float) Floating point types that check for correctness and implement total ordering.
- [result_float](https://crates.io/crates/result_float) Floating point type that cannot store NaN.
- [totally-ordered](https://crates.io/crates/totally-ordered) No dependency, no-std totally ordered f32/f64
- [unsigned-f64](https://crates.io/crates/unsigned-f64) A wrapper around f64 that guarantees that the value is always non-negative on the type level.

Features provided/checked by those crates:

✔️: provided, ❌: not provided, ❓: unknown

(you may need to scroll to the right to see all the columns: "Production ready", "Avoid `panic!`", "Minimal overhead", "Eq/Ord", "Hash", "NaN", "Inf", "Zero", "Positive", "Negative")

| Crates           | Production ready | Avoid `panic!` | Minimal overhead | Eq/Ord | Hash | NaN | Inf | Zero | Positive | Negative |
|------------------|------------------|----------------|------------------|--------|------|-----|-----|------|----------|----------|
|**`typed_floats`**| ✔️              | ✔️             | ✔️              | ✔️    | ✔️  | ✔️ | ✔️  | ✔️   | ✔️      | ✔️       |
| `checked-float`  | ✔️              | ✔️             | ❌              | ✔️    | ❌   | ✔️¹| ✔️¹ | ✔️¹  | ✔️¹     | ✔️¹      |
| `decorum`        | ✔️              | ❌             | ❌              | ❌    | ❌   | ✔️¹| ✔️¹ | ✔️¹  | ✔️¹     | ✔️¹      |
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

¹: Can be manually checked

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
[`core::default::Default`]: https://doc.rust-lang.org/core/core/default/trait.Default.html "`Default`"
[`core::hash::Hash`]: https://doc.rust-lang.org/core/hash/trait.Hash.html "`Hash`"
[`core::convert::From`]: https://doc.rust-lang.org/core/convert/trait.From.html "`From`"
[`core::convert::TryFrom`]: https://doc.rust-lang.org/core/convert/trait.TryFrom.html "`TryFrom`"
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
[`core::num::NonZeroU8`]: https://doc.rust-lang.org/core/num/struct.NonZeroU8.html "`NonZeroU8`"
[`core::num::NonZeroU16`]: https://doc.rust-lang.org/core/num/struct.NonZeroU16.html "`NonZeroU16`"
[`core::num::NonZeroU32`]: https://doc.rust-lang.org/core/num/struct.NonZeroU32.html "`NonZeroU32`"
[`core::num::NonZeroU64`]: https://doc.rust-lang.org/core/num/struct.NonZeroU64.html "`NonZeroU64`"
[`core::num::NonZeroI8`]: https://doc.rust-lang.org/core/num/struct.NonZeroI8.html "`NonZeroI8`"
[`core::num::NonZeroI16`]: https://doc.rust-lang.org/core/num/struct.NonZeroI16.html "`NonZeroI16`"
[`core::num::NonZeroI32`]: https://doc.rust-lang.org/core/num/struct.NonZeroI32.html "`NonZeroI32`"
[`core::num::NonZeroI64`]: https://doc.rust-lang.org/core/num/struct.NonZeroI64.html "`NonZeroI64`"
[`Hypot`]: https://docs.rs/typed_floats/latest/typed_floats/trait.Hypot.html
[`Min`]: https://docs.rs/typed_floats/latest/typed_floats/trait.Min.html
[`Max`]: https://docs.rs/typed_floats/latest/typed_floats/trait.Max.html
[`Copysign`]: https://docs.rs/typed_floats/latest/typed_floats/trait.Copysign.html
[`DivEuclid`]: https://docs.rs/typed_floats/latest/typed_floats/trait.DivEuclid.html
[`Atan2`]: https://docs.rs/typed_floats/latest/typed_floats/trait.Atan2.html
