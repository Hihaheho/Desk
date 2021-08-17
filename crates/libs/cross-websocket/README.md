# cross-websocket

[![GitHub](https://img.shields.io/badge/GitHub-Hihaheho/Desk-222222)](https://github.com/Hihaheho/Desk)
![Wasm](https://img.shields.io/badge/available-Wasm/Native-pink)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/cross-websocket)](https://crates.io/crates/cross-websocket)
[![docs.rs](https://img.shields.io/docsrs/cross-websocket)](https://docs.rs/cross-websocket)

A cross-platform websocket client library for Rust.

## Example

```rust
let (tx, rx) = cross_websocket::connect("ws://localhost:4000").await?.split();
```
