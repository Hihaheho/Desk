#! /usr/bin/env bash

# fail fast
set -euo pipefail
shopt -s inherit_errexit

cargo build --bin desk-x --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir envs/firebase/public --target web target/wasm32-unknown-unknown/release/desk-x.wasm
