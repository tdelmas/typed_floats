# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
