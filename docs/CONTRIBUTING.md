# Contribution Guide for Desk

## Flowchart

1. If you have a question -> [Create a new discussion if not exists](https://github.com/Hihaheho/Desk/discussions/categories/q-a)
2. If you have a feature request or a bug you found -> [Create a new issue if not exists](https://github.com/Hihaheho/Desk/issues)
3. If you have a nice code to merge -> [Open a pull request](https://github.com/Hihaheho/Desk/compare)

## Getting started

1. Enable Git LFS
2. Fork [Hihaheho/Desk](https://github.com/Hihaheho/Desk)
3. Clone the repository
4. [Setup Rust](https://www.rust-lang.org/tools/install)
5. [Install Bevy dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)
6. Run `cargo run -p desk-x`
7. Play with the code.
7. Run `tools/check-ci.sh`.

### Fast compile (optional)

1. Enable nightly: `rustup default nightly`
2. Copy [this](https://github.com/bevyengine/bevy/blob/main/.cargo/config_fast_builds) to `~/.cargo/config.toml` or `$YOUR_DESK_HOME/.cargo/config.toml`.
3. Install the recommended faster linker for your environment.
    - `zld` for Mac
    - `mold` for Linux

## Terminology

- **Desk App** -
  Any application using Desk.
- **Desk X** -
  An official Desk app consists of Desk plugins.
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
- **component crates** - defines components, data structures and its behaviors, that depends nothing except pure data structure things.
- **system crates** - defines systems, the core logics of Desk, with a pure function way with no any platform-specific things.
- **adapter crates** - handles platform-specific things and encode their data into components to use it from systems.
- **plugin crates** - integrates systems with Bevy.

## Directory structure

- `crates/` contains all crates.
  - `apps/` contains all executable crates.
    - `desk-x/` The entrypoint for Desk X.
  - `plugins/` contains plugin crates.
  - `systems/` contains system crates.
  - `components/` contains component crates.
  - `adapters/` contains adapter crates.
  - `libs/` contains useful libraries.
  - `tests/` contains integration tests.
- `docs/` contains many markdown or text files including documents and blogs.
- `envs/` contains platform-specific files.
- `configs/` contains config files.
- `tools/` contains things like command line tools to support development.
- `assets/` contains any media file, which is mainly used for Desk X.

- See [visualized codebase](https://mango-dune-07a8b7110.1.azurestaticapps.net/?repo=Hihaheho%2FDesk).

## The big picture of Desk

- **Desk-lang**
  - The most core thing of Desk.
  - Using Desk is ultimately decomposed into manipulating and interacting with the language.
- **Dworkspace**
  - Dworkspace is like OS especially for file system.
    - An API set to develop Desk Apps/Plugins.
    - The only data store for Desk Apps/Plugins.
    - Permission management
    - Realtime collaboration support
- **Dplugin**
  - Each dplugin adds a unique feature for Desk app.

![Screen Shot 0004-05-29 at 11 33 08](https://user-images.githubusercontent.com/8780513/170849556-1fdb2246-a9fe-4753-80a7-b547cce2e486.png)

## Dplugin guide

### Application states

- ASTs in a dworkspace: Desk-lang's syntax trees mainly used for anything such as data or programs.
- Node attributes: Metadata for each node of the ASTs, e.g., its display position or colors, or whether to show type hints for it.
- Dworkspace States: for internal and not user-related states.
- Bevy Components: states that indicates the behavior of dplugin.

**Flowchart**

1. If it's a code written by a user. -> ASTs
2. If it's data that describes anything and is consistent in any way for example how it would be rendered. -> ASTs
3. If it's data related to an AST node but does describe extrinsic aspects of it. -> Node attributes
4. If you need a state to make it work your systems. -> Dworkspace states
5. otherwise -> Bevy Components

## Desk-lang guide

### Terminology

- ASTs: a syntax tree.
- A type conclusion: a data contains all types, effects, and cast strategies for every nodes.
- MIR: Mid-level intermediate representation of a code that can be directly executed in an interpreter and can be used to generate an efficient executable of the code.

### Complilation flow

1. Parsing
2. Type/effect inference
3. MIR generation

## Dkernel guide

### Terminolopy

- **event** - an enum data that describes how AST is constructed and modified.
- **snapshot** - a temporal state of an AST data that is constructed from events.
- **repository** - an object that stores a stream of events and snapshots of the aggregated state.
- **audit** - reject invalid or not granted events to keep the state always correct and secured

### Lifecycle of Desk Apps

1. `Dworkspace::process` is called.
2. It polls events from repository.
3. It audits these events and rest certified events for later steps.
4. It updates dworkspace's internal states with the certified events
5. It updates dworkspace States with the certified events.
6. It updates the snapshot in the dworkspace.

## Coding guidelines

### General

- [**Simple made easy**](https://www.infoq.com/presentations/Simple-Made-Easy/)
- **Naming is the matter**
  - Describe it vividly.
  - Use metaphor but no implicit
  - If you cannot name it, you should not work on it.
    Feel free to ask someone for naming in [Discussion](https://github.com/Hihaheho/Desk/discussions) or Discord.
- **Perfect is better than done**
  - Your code may be used for decades.
  - Perfect means "If you would rewrite the code later, you would write the same code."
  - Perfect does not mean complex or complete but simple and minimally viable.
  - Since all of us are not good at always writing a perfect code, we never to quit refactoring.
  - If you don't think your architecture perfect, you should not write any code.
    Feel free to discuss in [Discussion](https://github.com/Hihaheho/Desk/discussions) or Discord.
- **Delete it over maintain it**
  - Any code should be deleted rather than maintaining it unless we think it's an absolutely necessary code.
- **Trust automations rather than each of us.
  - Test-first programming is highly recommended.
  - Use automations and keep it simple and portable.
- **Leveragable over feature-rich**
  - Make things leverageable and aggressively delete features.
- **Immediate over retained**
  - Avoid global retained state. It's always not simple.
  - Avoid on-demand call like event propagation. It's difficult to trace a bug.
  - Avoid to check needed or not before calling a function, just call it everytime.
- **Incremental over batching or sequential**
  - Write idempotent and pure functions, and use a tool for incremental computing.