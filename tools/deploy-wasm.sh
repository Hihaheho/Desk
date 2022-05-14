#! /usr/bin/env bash

# fail fast
set -euo pipefail
shopt -s inherit_errexit

tools/build-wasm.sh
npx wrangler pages publish --project-name desk --branch production --commit-dirty=true crates/apps/desk-x/public
rm -rf node_modules
