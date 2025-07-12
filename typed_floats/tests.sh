#!/bin/sh

# Stop at first error
set -e

# Display executed commands
set -x

cd $(dirname $0)

# First check that it builds with all features combinations
cargo build --no-default-features
cargo build --no-default-features --features std
cargo build --no-default-features --features serde
cargo build --no-default-features --features libm
cargo build --no-default-features --features serde,std
cargo build --no-default-features --features serde,libm
cargo build --no-default-features --features std,libm
cargo build --no-default-features --features serde,std,libm

cargo test --no-default-features
cargo test --no-default-features --features std
cargo test --no-default-features --features serde
cargo test --no-default-features --features libm
cargo test --no-default-features --features serde,std
cargo test --no-default-features --features serde,libm
cargo test --no-default-features --features std,libm
cargo test --no-default-features --features serde,std,libm

cargo test --release --no-default-features
cargo test --release --no-default-features --features std
cargo test --release --no-default-features --features serde
cargo test --release --no-default-features --features libm
cargo test --release --no-default-features --features serde,std
cargo test --release --no-default-features --features serde,libm
cargo test --release --no-default-features --features std,libm
cargo test --release --no-default-features --features serde,std,libm
