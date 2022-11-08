#! /usr/bin/env bash

VERSION=0.0.0

tools/list-crates.sh | while read crate; do
    cargo search dworkspace | grep -q -v "${crate} = \"${VERSION}\"" &&
        echo "Publishing ${crate}" &&
        cargo publish -p ${crate} --no-verify && sleep 20
done
