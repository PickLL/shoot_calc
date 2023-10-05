#!/bin/sh
git pull origin
rustup update
cargo update
cargo clean
cargo build --release
cp target/release/shoot_calc ./
