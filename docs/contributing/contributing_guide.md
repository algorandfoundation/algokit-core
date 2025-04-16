# Contributing Guide

## Principles

See the core principles in the repository's [README](../../README.md)

## Rust crates vs FFI libraries

The implementation of the rust crate should be completely seperate from the foreign interfaces. For example, [algokit_transact](../crates/algokit_transact/) does not depend on UniFFI or wasm-bindgen. Instead, there's a seperate crate [algokit_transact_ffi](../crates/algokit_transact_ffi/) that provides the foreign interfaces.
