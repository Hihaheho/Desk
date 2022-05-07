#! /usr/bin/env bash

# fail fast
set -euo pipefail
shopt -s inherit_errexit

cargo fmt
cargo deny check --config configs/deny.toml
cargo clippy --all-targets --all-features -- -D warnings -W clippy::all -W clippy::dbg_macro
cargo check --all-features
cargo test --all-features
