# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 1.0.7 - Unreleased

### Added

- 35% faster implementation of `signum` for all types
- 8% faster implementation of `Eq` for non-zero types
- 4% faster implementation of `Ord` for positives and negatives types

## 1.0.6 - 2025-05-24

### Added

- Add [`next_up`](https://doc.rust-lang.org/stable/std/primitive.f64.html#method.next_up) and [`next_down`](https://doc.rust-lang.org/stable/std/primitive.f64.html#method.next_down) methods (rust>=86)
- `PartialEq` optimization in most cases, faster than the default float `PartialEq` implementation (~8% faster)
- Add benchmarks in CI using `criterion`.
- More extaustive tests
- https://mutants.rs in CI

## 1.0.5 - 2025-03-24

### Added

- Add [`midpoint`](https://doc.rust-lang.org/core/primitive.f32.html#method.midpoint) method for (rust>=85)
- Method `get` is now `const`
- When possible, floats methods are const (rust>=1.85)

## 1.0.4 - 2025-03-11

### Security

- Invalid return type for `recip` and `to_radiants`[#200](https://github.com/tdelmas/typed_floats/pull/200)
⚠️ This is a breaking change, and exceptionally semver is *not* respected as it will only break code with the security issue.

## 1.0.3 - 2025-03-02

### Added

- When possible, floats methods are const (rust>=1.83)

## 1.0.1 - 2024-04-02

### Added

- `PartialEq` and `PartialOrd` implementations between all 12 types, and with `f32`/`f64`.

## 1.0.0 - 2024-03-09

### Added

- First version to commit to semantic versioning.
- Macros to create `const` values

## 0.7.4 - 2024-02-18

- `Default` implementation for types accepting `0.0` or `-0.0`
- Fix `Ord` implementation (Stack overflow) and add tests
- New macro `assert_float_eq` that differentiate `0.0` from `-0.0`.
- Improved documentation and tests

## 0.6.0 - 2023-11-15

- `Hash` implementation for all 12 types.

## 0.5.0 - 2023-11-12

- `libm` optional feature for `no_std`.

## 0.4.0 - 2023-11-05

### Added

- `no_std` support.

## 0.3.0 - 2023-11-04

### Added

- `serde` serialization and deserialization support.

## 0.2.3 - 2023-11-03

### Added

- `atan2` method.

## 0.2.1 - 2023-11-01

### Added

- `Hash` implementation for compatible types.
