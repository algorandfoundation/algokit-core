# Algorand-Rust-FFIs

This is a cargo workspace for Rust implementations of core Algorand functionality that is exposed to other languages via FFIs.

## Principles

### Idiomatic Rust

The core library should be idiomatic Rust. This is essential for maintainability and ease of use. This may result in some extra wrappers around the Rust library for the FFI bindings because the foreign languages and/or their binding generators might not support all the Rust features.

### Foreign Interface Consistency

All the foreign bindings should have consistent interfaces. The way things are done in Python should be the same as the way things are done in JavaScript. Finding the lowest common denominator between languages is the goal, which comes with the trade-off that we might not be fully leveraging foreign language features.

### Purely Functional Foreign Interfaces

The exported interfaces should be purely functional without any state owned by Rust. This is essential for maintaining consistency between the foreign bindings and memory safety. Some foreign languages, such as `kotlin` or `go` do not support integrating Rust-owned objects into garbage collection. If Rust owned stateful objects, users of these languages would need to manually manage the lifetimes of the objects or potentially leak memory. See [this document](/docs/research/ffi_garbage_collection.md) for more information on garbage collection.

## Crates

- [algokit_transact](./crates/algokit_transact) - Currently a spike to determine feasibility of using UniFFI and wasm-pack. Handles msgpack encoding and decoding of Algorand transactions and allows attaching signatures to transactions.

## ADRs

[./docs/decisions](./docs/decisions) contains the ADRs for this project. Specific implementations might have their own ADRs in their respective directories.

## Research

[./docs/research](./docs/research/) contains various Markdown documents summarize research related to this project

## Contributing

### Learning Resources

If you are new to Rust or UniFFI, check out the [learning resources document](./docs/contributing/learning_resources.md)

### Contributing Guide

See the [contributing guide](./docs/contributing/contributing_guide.md) for recommended editors settings and other guidelines for contribution.
