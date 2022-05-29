# Contribution Guide for Desk

## Getting started

1. Fork [Hihaheho/Desk](https://github.com/Hihaheho/Desk)
2. Clone the repository
3. [Setup Rust](https://www.rust-lang.org/tools/install)
4. [Install Bevy dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)
5. Run `cargo run -p desk-x`
6. Run `tools/check-ci.sh`

### Fast compile (optional)

1. Enable nightly: `rustup default nightly`
2. Copy [this](https://github.com/bevyengine/bevy/blob/main/.cargo/config_fast_builds) to `~/.cargo/config.toml` or `desk_dir/.cargo/config.toml`.
3. Install the fast linker for your environment.
    - `zld` for Mac
    - `lld` for Linux

## Terminology

- **Desk App** -
  Any application using Desk.
- **Desk X** -
  An official hosting of a Desk app
- **Desk** -
  The entire system consisting of desk-lang, dkernel, and a set of dplugins.
- **Desk-lang** -
  Desk-lang (Desk language) is a minimal functional language with statical typing.
- **Dkernel** -
  Dkernel (Desk Kernel) is the central dogma of dplugins.
- **Dplugin** -
  A dplugin (Desk plug-in) is a plug-in of Desk X that implements UI and integration with outer world of Desk.
- **ECS** -
  A framework to build an application with:
  - **Entities** - unique objects that has zero or more components
  - **Components** - describes the entity
  - **Systems** - implements behavior of application by using components
- **Bevy** -
  A game engine with the most powerful ECS. It's used for:
  - Application lifecycle
  - ECS with events
  - Mesh rendering
  - Input management
- **egui** -
  Immediate-mode GUI library for any backend.

## Directory structure

- `crates/`
  - `apps/` Executable crates for Desk
    - `desk-x/` is the entrypoint for Desk X.
  - `plugins/` provides a feature to application by using systems
  - `systems/` defines functionality with components
  - `components/` defines data and its behavior for the application
  - `adapters/` handles external resources for systems
  - `tests/` Integration tests
- `docs/` Documents
- `envs/` Deployment things
- `configs/` Config files
- `tools/` Command line tools for ease of development

- See [visualized codebase](https://mango-dune-07a8b7110.1.azurestaticapps.net/?repo=Hihaheho%2FDesk).

## The big picture of Desk

### Desk-lang

The most core thing of Desk. Using Desk is ultimately decomposed into manipulating and interacting with the language.

### Dkernel

Dkernel is a API set for developing Dplugins.
Dkernel is like file system in OS
Data store
Permission management
Collaboration support

### Dplugin

Each Dplugin adds a unique interactive surface to Desk-lang by communicating with Dkernel.

## Dplugin guide

### 4 of states

- Node attribute
- AST
- Kernel State
- Bevy Component

Flowchart

## Desk-lang guide

### Compliler paths

1. Lexer
2. Parser
3. HIR generator
4. Type inference
5. Typed HIR generator 
6. Abstract MIR generator
7. Concretizer (Concrere MIR generator)

### Execution

Currently there is only a interpreter called MIR Evaluator (mireval)
Mireval executes a concrere MIR by statement by statement   

## Dkernel guide

## Coding guidelines

### General

- [**Simple made easy**](https://www.infoq.com/presentations/Simple-Made-Easy/)
- **Naming is the matter**
  - Describe it vividly.
  - Use metaphor.
  - If you cannot name it, you should not work on it.
    Feel free to ask someone for naming in [Discussion](https://github.com/Hihaheho/Desk/discussions) or Discord.
- **Perfect is better than done**
  - Perfect means "If I would rewrite the code tomorrow, I'll write the same code."
  - Perfect does not mean complex or complete.
  - If you don't think your architecture perfect, you should not write any code.
    Feel free to discuss in [Discussion](https://github.com/Hihaheho/Desk/discussions) or Discord.
- **Delete over maintain**
  - Any code should be deleted rather than maintaining it unless you think it's absolutely necessary.
- **Don't trust human checks**
  - Use automations instead
- **Leveragable over feature-rich**
  - Make things leveragable rather than adding features.
- **Immediate over retained**
  - Avoid global retained state. It's not simple.
  - Avoid on-demand call by event propagation. It's difficult to trace.
  - Avoid to check needed or not before calling a function, just call it everytime. No human check.
- **Incremental over batching**
  - You are encouraged to use [salsa](https://github.com/salsa-rs/salsa) as data store.
