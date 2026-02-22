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

cargo test --lib --bins --tests --no-default-features --features f32
cargo test --lib --bins --tests --no-default-features --features f64
cargo test --lib --bins --tests --no-default-features --features f32,std
cargo test --lib --bins --tests --no-default-features --features f32,serde
cargo test --lib --bins --tests --no-default-features --features f32,libm
cargo test --lib --bins --tests --no-default-features --features f32,serde,std
cargo test --lib --bins --tests --no-default-features --features f32,serde,libm
cargo test --lib --bins --tests --no-default-features --features f32,std,libm
cargo test --lib --bins --tests --no-default-features --features f32,serde,std,libm

cargo test --lib --bins --tests --release --no-default-features --features f32
cargo test --lib --bins --tests --release --no-default-features --features f64
cargo test --lib --bins --tests --release --no-default-features --features f32,std
cargo test --lib --bins --tests --release --no-default-features --features f32,serde
cargo test --lib --bins --tests --release --no-default-features --features f32,libm
cargo test --lib --bins --tests --release --no-default-features --features f32,serde,std
cargo test --lib --bins --tests --release --no-default-features --features f32,serde,libm
cargo test --lib --bins --tests --release --no-default-features --features f32,std,libm
cargo test --lib --bins --tests --release --no-default-features --features f32,serde,std,libm
