#! /usr/bin/env bash

# fail fast
set -euo pipefail
shopt -s inherit_errexit

tools/build-wasm.sh
cat <<EOF > firebase.json
{
  "hosting": {
    "public": "crates/apps/desk-x/public"
  }
}
EOF
firebase --project hihaheho-e58a7 deploy --only hosting
rm firebase.json
