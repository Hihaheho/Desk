#! /bin/bash

tools/build-wasm.sh &&
    # cargo install basic-http-server
    basic-http-server -a 0.0.0.0:4000 crates/apps/desk-x/public
