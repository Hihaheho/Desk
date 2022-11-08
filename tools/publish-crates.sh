#! /usr/bin/env bash

VERSION=0.0.0

tools/list-crates.sh | while read crate; do
    if cargo search ${crate} | grep --quiet "${crate} = \"${VERSION}\""; then
        echo "Skipping ${crate} because it is already published"
    else
        echo "Publishing ${crate}" &&
        cargo publish -p ${crate} --no-verify && sleep 20
    fi
done
