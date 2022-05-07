#! /usr/bin/env bash

# fail fast
set -euo pipefail
shopt -s inherit_errexit

tools/build-wasm.sh
cd crates/apps/desk-x/
firebase deploy --only hosting
cd -
