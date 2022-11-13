#! /usr/bin/env bash

cat configs/crates.txt | grep -v "^#" | sed 's/^!//' | awk NF
