# Contribution Guide for Desk

## Flowchart

1. If you have a question -> [Create a new discussion if not exists](https://github.com/Hihaheho/Desk/discussions/categories/q-a)
2. If you have a feature request or a bug you found -> [Create a new issue if not exists](https://github.com/Hihaheho/Desk/issues)
3. If you have a nice code to merge -> [Open a pull request](https://github.com/Hihaheho/Desk/compare)

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
    - `mold` for Linux

## Terminology

- **Desk App** -
  Any application using Desk.
- **Desk X** -
  An official hosting of a collection of Desk apps
- **Desk** -
  The entire system consisting of desk-lang, dworkspace, and a set of dplugins.
- **Desk-lang** -
  Desk-lang (Desk programming language) is a minimal functional language with statical typing and effects.
- **Dworkspace** -
  Dworkspace (Desk Workspace) is the central dogma of dplugins.
- **Dplugin** -
  A dplugin (Desk plug-in) is a bevy plug-in of Desk X that implements UI and integration with the outer world of Desk.
- **ECS** -
  A framework to build an application with:
  - **Entities** - unique objects that has zero or more components
  - **Components** - describes the entity
  - **Systems** - implements behavior of application by using components
- **Bevy** -
  A game engine with stateof-the-art ECS. It's used for:
  - Application lifecycle
  - ECS with events
  - Mesh rendering
  - Input management
- **egui** -
  Immediate-mode GUI library for any backend.
- **Rust** - The most loved programming language in the world.
- **crate** - An unit of application/library containing source codes in Rust-lang.
- **component crates** - Defines components, data structures and its behaviors, that depends nothing except pure data structure things.
- **system crates** - Defines systems, the core logics of Desk, with a pure function way with no any platform-specific things.
- **adapter crates** - Handles platform-specific things and encode their data into components to use it from systems.

## Directory structure

- `crates/` Includes all crates.
  - `apps/` Executable crates for Desk
    - `desk-x/` the entrypoint for Desk X.
  - `plugins/` provides a feature to application with using systems
  - `systems/` defines functionality with pure function way. Depends only on platform-agnostic libraries.
  - `components/` defines data and its behavior for the application. Depends only on data-layer libralies.
  - `adapters/` handles external resources and encode their data to components for systems.
  - `tests/` Integration tests
- `docs/` Documents
- `envs/` Deployment things
- `configs/` Config files
- `tools/` Command line tools for ease of development

- See [visualized codebase](https://mango-dune-07a8b7110.1.azurestaticapps.net/?repo=Hihaheho%2FDesk).

## The big picture of Desk

- **Desk-lang**
  - The most core thing of Desk.
  - Using Desk is ultimately decomposed into manipulating and interacting with the language.
- **Dkernel**
  - Dkernel is like file system in OS.
    - An API set and data store for developing Dplugins.
    - Permission management
    - Collaboration support
- **Dplugin**
  - Each dplugin adds a unique feature for Desk app.

![Screen Shot 0004-05-29 at 11 33 08](https://user-images.githubusercontent.com/8780513/170849556-1fdb2246-a9fe-4753-80a7-b547cce2e486.png)

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

- Currently there is only a interpreter called MIR Evaluator (mireval).
- Mireval executes a concrere MIR by statement by statement.

## Dkernel guide

### Terminolopy

- **event** - an enum data that describes how AST is modified
- **repository** - an object that stores events
- **audit** - filters denied events
- **snapshot** - all AST data

### Lifecycle

1. `Kernel::process` is called
2. Poll events from repository
3. Audit events
4. Handle events for internal
5. Handle events for kernel states
6. Handle events for snapshot

## Coding guidelines

### General

- [**Simple made easy**](https://www.infoq.com/presentations/Simple-Made-Easy/)
- **Naming is the matter**
  - Describe it vividly.
  - Use metaphor.
  - If you cannot name it, you should not work on it.
    Feel free to ask someone for naming in [Discussion](https://github.com/Hihaheho/Desk/discussions) or Discord.
- **Perfect is better than done**
  - Your code may be used for decades.
  - Perfect means "If I would rewrite the code after years, I'll write the same code."
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
