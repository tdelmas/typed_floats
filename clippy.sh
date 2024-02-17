#!/bin/sh

# Stop at first error
set -e

# Display executed commands
set -x

cd $(dirname $0)

cargo +nightly clippy --no-default-features
cargo +nightly clippy --no-default-features --features serde
cargo +nightly clippy --no-default-features --features libm
cargo +nightly clippy --no-default-features --features serde,libm
cargo +nightly clippy --no-default-features --features serde,std,libm

