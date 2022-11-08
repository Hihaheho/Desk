#! /usr/bin/env bash

# fail fast
set -euo pipefail
shopt -s inherit_errexit

VERSION=0.0.0

skip_until=${1:-""}
skip="yes"
if [ -z $skip_until ]; then
    skip="no"
fi

tools/list-crates.sh | while read crate; do
    if [ $skip = "yes" ]; then
        if [ $crate = $skip_until ]; then
            skip="no"
        else
            continue
        fi
    fi
    if cargo search ${crate} | grep --quiet "${crate} = \"${VERSION}\""; then
        echo "Skipping ${crate} because it is already published"
        sleep 5
    else
        echo "Publishing ${crate}"
        cargo publish -p ${crate} --no-verify
        sleep 20
    fi
done
