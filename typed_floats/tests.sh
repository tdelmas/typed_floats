#!/bin/sh

# Stop at first error
set -e

# Display executed commands
set -x

cd $(dirname $0)

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
