#! /bin/bash

cargo fmt
cargo deny check --config configs/deny.toml &&
    cargo clippy --all-targets --all-features -- -D warnings -W clippy::all -W clippy::dbg_macro &&
    cargo check --all-features &&
    cargo test --all-features &&
