#!/bin/sh

# Stop at first error
set -e

# Display executed commands
set -x

cd $(dirname $0)

# Default tests (with std feature only)
cargo test

## Test without any feature (no std)
cargo test --no-default-features
cargo test --test 'serde' --no-default-features --features serde
cargo test --no-default-features --features libm

# Test serde (with std)
cargo test --verbose --features serde

## Run tests with the two main features: serde and std
cargo test --release --no-default-features --features serde,std

