#! /usr/bin/env bash

cat configs/crates.txt | grep -v "^#" | awk NF
