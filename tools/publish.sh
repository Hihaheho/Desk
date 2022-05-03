#! /bin/bash

crates=(
    deskc-dson
    deskc-file
    deskc-link
    deskc-types
    deskc-tokens
    deskc-ast
    deskc-hir
    deskc-thir
    deskc-amir
    deskc-mir
    deskc-textual-diagnostics
    deskc-lexer
    deskc-parser
    deskc-hirgen
    deskc-typeinfer
    deskc-thirgen
    deskc-amirgen
    deskc-concretizer
    deskc-evalmir
    deskc-fmt
    deskc-thir2dson
    deskc
    deskc-language-server
    serde-dson
    dkernel-node
    dkernel-diff
    dkernel
    dkernel-firestore
    dkernel-in-memory
    desk-x-theme
    desk-x-egui-plugin
    desk-x
)

for crate in "${crates[@]}"
do
    echo "Publishing ${crate}"
    cargo publish -p ${crate} --no-verify
    sleep 10
done
