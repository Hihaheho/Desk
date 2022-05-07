#! /usr/bin/env bash

crates=(
    dson
    deskc-file
    deskc-ids
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
    deskc-cli
    deskc-language-server
    serde-dson
    dkernel-card
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
    cargo publish -p ${crate} --no-verify && sleep 20
done
