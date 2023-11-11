#!/bin/sh

# Stop at first error
set -e

# First check that it builds with all features combinations
cargo xtask pre-build
cargo build
cargo build --no-default-features
cargo build --no-default-features --features serde
cargo build --no-default-features --features libm
cargo build --no-default-features --features serde,libm
cargo build --no-default-features --features serde,std,libm


# Default tests (with std feature only)
cargo test

# no-std tests

## Test without any feature (no std)
cargo test --no-default-features
cargo test --test 'serde' --no-default-features --features serde
cargo test --no-default-features --features libm

# Test serde (with std)
cargo test --verbose --features serde

cargo build --no-default-features --features serde,std,libm

# Test in release mode

## Only check that it builds with all features combinations
cargo build --release
cargo build --release --no-default-features
cargo build --release --no-default-features --features serde
cargo build --release --no-default-features --features libm
cargo build --release --no-default-features --features serde,libm
cargo build --release --no-default-features --features serde,std,libm

cargo test --no-run --release
cargo test --no-run --release --no-default-features
cargo test --no-run --release --no-default-features --features serde
cargo test --no-run --release --no-default-features --features libm
cargo test --no-run --release --no-default-features --features serde,libm
cargo test --no-run --release --no-default-features --features serde,std,libm

## Run tests with the two main features: serde and std
cargo test --release --no-default-features --features serde,std

