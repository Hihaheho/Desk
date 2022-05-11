#! /usr/bin/env bash

# fail fast
set -euo pipefail
shopt -s inherit_errexit

tools/build-wasm.sh
cd envs/firebase
firebase deploy --only hosting
cd -
