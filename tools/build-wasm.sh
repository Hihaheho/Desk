#! /usr/bin/env bash

# fail fast
set -euo pipefail
shopt -s inherit_errexit

cargo build --bin desk-x --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir crates/apps/desk-x/public --target web target/wasm32-unknown-unknown/release/desk-x.wasm
