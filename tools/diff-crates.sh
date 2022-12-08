#! /usr/bin/env bash

diff <(grep name -r crates/*/*/Cargo.toml | awk '{print $3}' | sed 's/"//g' | grep -v test | sort) <(tools/list-crates.sh | sort)
exit $?
