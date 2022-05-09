#! /usr/bin/env bash

# fail fast
set -euo pipefail
shopt -s inherit_errexit

cargo deny check --config configs/deny.toml
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings -W clippy::all -W clippy::dbg_macro
cargo check --all-features
cargo test --no-run --locked --all-features
cargo test --all-features
