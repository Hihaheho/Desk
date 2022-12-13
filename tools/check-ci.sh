#! /usr/bin/env bash

# fail fast
set -eo pipefail
shopt -s inherit_errexit

for OPT in "$@"
do
    case $OPT in
        --no-cargo-deny)
	    NO_CARGO_DENY=1
            ;;
    esac
done

tools/diff-crates.sh
cargo fmt --all -- --check
cargo check --tests --all-features
cargo clippy --all-targets --all-features -- -D warnings -W clippy::all -W clippy::dbg_macro
cargo test --no-run --locked --all-features
cargo test --all-features
[[ -z ${NO_CARGO_DENY} ]] && cargo deny check --config configs/deny.toml
