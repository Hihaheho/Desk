# Desk

### 🔮 The application platform for your cyberpunk desk

[![Demo](https://img.shields.io/badge/Desk--X-Wasm+WebGL2-b236a6)](https://desk-x.com)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg?style=flat)](https://github.com/Hihaheho/Desk/blob/main/LICENSE)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/ryo33?color=ffc5cd&labelColor=2a4638)](https://github.com/sponsors/ryo33)
[![GitHub Repo stars](https://img.shields.io/github/stars/Hihaheho/Desk?style=social&color=yellow)](https://github.com/Hihaheho/Desk)

## Release soon!

Status: All plans are behind the schedule for several reasons.
- I'm building [query-flow](https://github.com/ryo33/query-flow) for `deskc` and `dworkspace`.
- I want to discard the current `mirgen` and design a new MIR intervening between Desk-lang and low-level IR to generate a GC-less binary, which is fast as Rust in theory (in my mind).
- I've switched to Ubuntu Desktop from M1 Macbook Pro mainly for financial reasons. As a subsequence of it, I can utilize the mold linker.
- I've returned to Neovim after several years of a temporary stay in the VSCode.
- I'm trying to replace my trackball mouse with self-built input device for using alongside of my build of the [Helix keyboard](https://shop.yushakobo.jp/en/products/2143) to operate the cursor while on the home position.

### Pre-release (planned for this month)

Pre-release includes:

- [x] incremental Desk-lang compiler
- [x] DeskVM with an official scheduler
- [x] a file system for Desk-lang
- [ ] Desk-lang visual editor
- [ ] A web demo like Rust Playground for Desk-lang

### The first release (planned for earlier 2023)

The first release includes:

- [ ] MVP of Desk Craft, a game engine
- [ ] a platformer game demo
- [ ] a space to publish created games
- [ ] real-time collaboration on Web
- [ ] paid plans for Desk X (official hosting)

[See the draft of the first release](/docs/blog/0001-introduce-desk.md)

## Goals and Philosophy

🎯 **Blur the line between living and coding**
🎯 **Make every software programmable**

- 🎮 **Intuitive** like games
- 🥼 **Pragmatic** like professional tools
- 🗺️️ **Versatile** like spreadsheets
- 💗 **Accessible** to everyone
- 🛹 **Minimalist** design

## Why Desk?

Desk apps are inherently:

- 🎼 **code-oriented** like data-oriented
- 🔒 **statically-typed** (data and UI)
- 🤖 **programmable** (extensible by code)
- 🧲 **interoperable** with other Desk apps
- 📱 running on **everywhere** (web, desktop, mobile)

## How does Desk work?

Desk is consist of:

- Desk Programming Language and Desk Compiler (deskc)
- Desk Workspace System (dworkspace)
- DeskVM (deskvm)
- Desk-plugins (dplugins)

## Desk Programming Language (Desk-lang)

Desk-lang is a programming language that has:

- minimalistic syntax and semantics
- type system with inference
- algebraic effects
- content-addressable by type and UUID
- incremental compilation

Most of the data and programs on Desk apps are finally evaluated as a snippet of Desk-lang.

## Desk Compiler (deskc)

Desk compiler is an incremental compiler for Desk-lang.

**Crates**

- [deskc](/crates/systems/deskc/src): the incremental compiler
- [deskc-lexer](/crates/systems/deskc-lexer/src) scans Desk-lang source code and generates tokens
- [deskc-parser](/crates/systems/deskc-parser/src) parses tokens and generates an AST
- [deskc-typeinfer](/crates/systems/deskc-typeinfer/src) infers types of expressions.
- [deskc-mirgen](/crates/systems/deskc-mirgen/src) generates [MIR](/crates/components/deskc-mir/src)

## Desk-workspace (dworkspace)

Desk-workspace is a platform-agnostic environment for editing Desk-lang.

Desk-workspace provides these features:

- file system for Desk-lang
- permission management system
- realtime collaboration support

**Crates**

- [dworkspace](/crates/systems/dworkspace/src): the implementation
- [dworkspace-codebase](/crates/components/dworkspace-codebase/src) defines structs for a codebase

## DeskVM (deskvm)

DeskVM is a runtime for Desk-lang influenced by Erlang VM.

**Features**

- platform-agnostic
- capable of running many programs as a d-process
- type-driven message passing and pub/sub
- interpreter-agnostic: DeskVM can run anything as a d-process
- preemptive scheduling
- custom scheduler support

**Crates**

- [deskvm](/crates/systems/deskvm/src): the implementation
- [deskvm-dprocess](/crates/components/deskvm-dprocess/src) defines structs of such as d-process

## Desk-plugins (dplugins)

There are many Desk-plugins. Each Desk-plugin implements a single feature as a Bevy Plugin.

- 🚧 **Desk Craft** for game development
- 🚧 **Desk Brain** for productivity
- 🚧 **Desk Verse** for communication
- 🚧 **Desk Robot** for automation
- 🚧 **Desk Board** for BI
- 🚧 **Desk Calendar** for scheduling
- 🚧 **Desk Pages** for hosting

🚧 not yet implemented

## Is it any good?

Yes.

## Resources

- [Contributing Guide](https://github.com/Hihaheho/Desk/blob/main/docs/CONTRIBUTING.md)

## Join our community

👉 [![Q&A Have a question?](https://img.shields.io/badge/Q%26A-Have%20a%20question%3F-yellowgreen?style=social&logo=github)](https://github.com/Hihaheho/Desk/discussions/new?category=q-a)

👉 [![GitHub Discussions](https://img.shields.io/github/discussions/Hihaheho/Desk?logo=GitHub&style=social)](https://github.com/Hihaheho/Desk/discussions)

👉 [![GitHub Repo stars](https://img.shields.io/github/stars/Hihaheho/Desk?style=social)](https://github.com/Hihaheho/Desk)

👉 [![Twitter Follow](https://img.shields.io/twitter/follow/HihahehoStudio?style=social)](https://twitter.com/HihahehoStudio)

👉 [![Discord](https://img.shields.io/discord/808315755460165683?color=6A7EC2&label=&logo=discord&logoColor=ffffff&labelColor=4e5af0&style=for-the-badge)](https://discord.gg/egTTeg7DRp)
