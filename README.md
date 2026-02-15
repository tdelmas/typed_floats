
[![Build Status](https://circleci.com/gh/tdelmas/typed_floats.svg?style=shield)](https://circleci.com/gh/tdelmas/typed_floats)
[![Tests on GitHub CI](https://github.com/tdelmas/typed_floats/actions/workflows/tests.yml/badge.svg)](https://github.com/tdelmas/typed_floats/actions/workflows/tests.yml)
[![Version](https://img.shields.io/crates/v/typed_floats.svg)](https://crates.io/crates/typed_floats)
[![Documentation](https://docs.rs/typed_floats/badge.svg)](https://docs.rs/typed_floats)
[![License](https://img.shields.io/crates/l/typed_floats.svg)](https://github.com/tdelmas/typed_floats/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/tdelmas/typed_floats/status.svg)](https://deps.rs/repo/github/tdelmas/typed_floats)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.70-blue)](https://github.com/rust-lang/rust/releases/tag/1.70.0)
[![Miri](https://github.com/tdelmas/typed_floats/actions/workflows/miri.yml/badge.svg)](https://github.com/tdelmas/typed_floats/actions/workflows/miri.yml)
[![Changelog](https://img.shields.io/badge/CHANGELOG.md--555.svg)](https://github.com/tdelmas/typed_floats/blob/main/CHANGELOG.md)

This crate helps you to ensure the kind of floats you are using, without `panic!` (except if the `unsafe` function `new_unchecked` is used in an unsound way).

zero overhead: everything is checked at compile time.
(only `new` and `try_from` adds a little overhead at runtime)

`NaN` is rejected by all types.

# TL;DR

This crate is for you if:

- If you want to know at compile time if a float can be negative, positive, zero, finite and ensure it is not `NaN`, without `panic!`.

- If you need [`core::cmp::Ord`], [`core::cmp::Eq`] or [`core::hash::Hash`] on (non-`NaN`) floats.

# The 12 types provided by this crate

- [`NonNaN`], [`NonNaNFinite`], [`NonZeroNonNaN`], [`NonZeroNonNaNFinite`]

And their positive and negative counterparts:

- [`Positive`],[`PositiveFinite`], [`StrictlyPositive`], [`StrictlyPositiveFinite`]
- [`Negative`],[`NegativeFinite`], [`StrictlyNegative`], [`StrictlyNegativeFinite`]

| Type | `-∞` | `]-∞; -0.0[` | `-0.0` | `+0.0` | `]+0.0; +∞[` | `+∞` | `NaN` |
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

Most constants are also available, with the most appropriate typed float type (except `NAN` for obvious reasons) in the [`tf64`] and [`tf32`] modules (in [`tf64::consts`] and [`tf32::consts`] respectively when the constant comes from [`core::f64::consts`] or [`core::f32::consts`]). Those modules are named that way to avoid conflicts or confusion with the primitives [`f32`] and [`f64`].

⚠️ Like for primitives [`f32`] and [`f64`],`-0.0 == +0.0` is `true` for all types of this crate.
To facilitate comparisons, the methods `is_positive_zero` and `is_negative_zero` are added.

# Traits implemented

## Conversions: [`core::convert::From`] / [`core::convert::TryFrom`]

- Between all the types of this crate (of the same kind, [`f32`] or [`f64`])
- From [`f32`] and [`f64`]
- From integers types (except [`u128`] and [`i128`])
- From `NonZero*` ([`core::num::NonZeroU8`], [`core::num::NonZeroU16`], [`core::num::NonZeroU32`], [`core::num::NonZeroU64`], [`core::num::NonZeroI8`], [`core::num::NonZeroI16`], [`core::num::NonZeroI32`], [`core::num::NonZeroI64`])

(The traits `From` and `TryFrom` are implemented depending on the situation)

## Comparisons: [`core::cmp::PartialOrd`] and [`core::cmp::PartialEq`]
| 🗘 | `f32`/`f64` | [`NonNaN`] | [`NonNaNFinite`] | [`NonZeroNonNaN`] | [`NonZeroNonNaNFinite`] | [`Positive`] | [`PositiveFinite`] | [`StrictlyPositive`] | [`StrictlyPositiveFinite`] | [`Negative`] | [`NegativeFinite`] | [`StrictlyNegative`] | [`StrictlyNegativeFinite`]
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `f32`/`f64` | N/A |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ | 
| [`NonNaN`] | ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ | 
| [`NonNaNFinite`] | ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ | 
| [`NonZeroNonNaN`] | ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ | 
| [`NonZeroNonNaNFinite`] | ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ | 
| [`Positive`] | ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ | 
| [`PositiveFinite`] | ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ | 
| [`StrictlyPositive`] | ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ | 
| [`StrictlyPositiveFinite`] | ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ | 
| [`Negative`] | ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ | 
| [`NegativeFinite`] | ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ | 
| [`StrictlyNegative`] | ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ | 
| [`StrictlyNegativeFinite`] | ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ |  ✔️ | 

## Traits without generic parameters

| Trait | [`NonNaN`] | [`NonNaNFinite`] | [`NonZeroNonNaN`] | [`NonZeroNonNaNFinite`] | [`Positive`] | [`PositiveFinite`] | [`StrictlyPositive`] | [`StrictlyPositiveFinite`] | [`Negative`] | [`NegativeFinite`] | [`StrictlyNegative`] | [`StrictlyNegativeFinite`] |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| [`core::cmp::Eq`] | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| [`core::cmp::Ord`] | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| [`core::hash::Hash`] | ✔️¹ | ✔️¹ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| [`core::default::Default`] | `0.0` | `0.0` | ❌ | ❌  | `0.0` | `0.0` | ❌ | ❌ | `-0.0` | `-0.0` | ❌ | ❌ |

¹: there is a (small) overhead because they accept `0.0` and `-0.0` (which are equal) so they must `core::hash::Hash` to the same value.

# Methods implemented

All 12 types implement the methods available on [`f32`] and [`f64`] **except**:

- deprecated and nightly-only methods
- `total_cmp(&self, other: &f64) -> Ordering`
- `sin_cos(self) -> (f64, f64)`
- `mul_add(self, a: f64, b: f64) -> f64`
- `clamp(self, min: f64, max: f64) -> f64`
- `LowerExp`
- `UpperExp`
- `Product`
- `Sum`
- `to_int_unchecked`
- `to*_bits`
- `from*_bits`

## Panics

The only method that can `panic!` is the `unsafe` method `new_unchecked` when used in an invalid way.

A `panic!` triggered in any other way is considered a security bug and should be reported.

## Minimal overhead and optimizations

This crate is designed to have a minimal overhead at runtime, in terms of memory, speed and binary size.

It can even be faster than using primitives [`f32`] and [`f64`] directly, as it may avoids some checks by using compiler hints and can use some faster implementations in some cases.

### Overhead

The only methods that adds a little overhead are `try_from` because of the checks they do at runtime, compared to the `unsafe` method `new_unchecked`.

In debug mode, a little overhead is present, both to check the validity of the values and because `inline` may not be respected.

Any other overhead is considered a bug and should be reported.

### Compiler optimizations

The compiler hints are enabled by default to enable compiler optimization when possible.

Also, some methods are faster than the default implementation. For example:

- When possible `Eq` is implemented by comparing the bits of the two floats instead of the slower default implementation, that had special cases for `NaN` (to handle `NaN != NaN`) and `-0.0` (to handle `-0.0 == 0.0`). (8% faster)
- `Ord` is implemented by directly comparing the bits of the two floats instead of the slower default implementation for negatives and positives types. (4% faster)
- `signum` doesn't needs to check for `NaN`. (35% faster)
- types others than [`NonNaN`] and [`NonNaNFinite`] use a faster implementation of `Eq`, opening the door to Jump Threading optimisations.

# Features

- `std`: enabled by default, gives all `f32` and `f64` methods.
- `serde`: implements `Serialize` and `Deserialize` for all 12 types.
- `libm`: use the `Float` trait from `num-traits` and `libm` to implement the missing methods when the `std` feature is disabled. When both `std` and `libm` features are enabled, the `std` implementation is used.
- `compiler_hints`: enabled by default, will add `core::hint::unreachable_unchecked` after all `debug_assert`. 
- `ensure_no_undefined_behavior`:  Will `panic!` in release mode instead of risking undefined behavior. This will override the `compiler_hints` feature, and adds a little overhead to `new_unchecked`.
- `f32` and `f64`: enabled by default, allow to only build the types for `f32` or `f64`. Enableing only one of those decrease the code generated by ~50% and compile time by ~10%.


## How it works

For each operation, at compile time crate determine the most strict type possible for the result.

For example, if you multiply a [`PositiveFinite`] and a [`StrictlyNegativeFinite`], the result will be a [`Negative`].

Methods that takes another float as parameter will also return the most strict type possible depending on the both types. For the methods where a trait is not available to specify the return type depending on the parameter type, a new trait is created: 
[`Hypot`], [`Min`], [`Max`], [`Copysign`], [`DivEuclid`] and [`Atan2`].

## Main limitations

- Doesn't fix the floating point quirks such as `0.0 == -0.0`
- Doesn't fix the odd methods such as:
  - `sqrt(-0.0)` returning `-0.0` instead of `NaN`
  - `min(-0.0, 0.0)` possibly returning `0.0` instead of `-0.0` (same for `max`)
  - `frac(-0.0)` returning `0.0` instead of `-0.0`

Because that would introduce a runtime overhead and may introduce some incompatibilities with existing code.

## Rust version

This crate is tested when a new version is release with:
- Rust beta
- Rust stable
- Rust 1.70.0

Also, tests on `nightly`, `beta` and `stable` are run monthly on [GitHub actions](https://github.com/tdelmas/typed_floats/actions/workflows/exaustive-tests.yml).

The minimum supported Rust version (MSRV) is 1.70.0 because of the use of `dep:` in `Cargo.toml`.
A change in the MSRV will be treated as a breaking change.

## Testing

Tests are run on different architectures on [GitHub actions](https://github.com/tdelmas/typed_floats/actions/workflows/tests.yml) and [CircleCI](https://circleci.com/gh/tdelmas/typed_floats).

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
[`core::cmp::PartialOrd`]: https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html "`PartialOrd`"
[`core::cmp::PartialEq`]: https://doc.rust-lang.org/core/cmp/trait.PartialEq.html "`PartialEq`"
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
