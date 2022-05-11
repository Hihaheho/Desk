#! /usr/bin/env bash

tools/list-crates.sh | while read crate; do
    echo "Publishing ${crate}"
    cargo publish -p ${crate} --no-verify && sleep 20
done
