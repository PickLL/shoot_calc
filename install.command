#!/bin/sh
rustup update
cargo update
cargo clean
cargo build --release
cp target/release/shoot_calc ./
