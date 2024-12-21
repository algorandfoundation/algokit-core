# Contributing Guide

## Principles

See the core principles in the repository's [README](../../README.md)

## Rust crates vs FFI libraries

The implementation of the rust crate should be completely seperate from the foreign interfaces. For example, [algo_models](../crates/algo_models/) does not depend on UniFFI or wasm-bindgen. Instead, there's a seperate crate [algo_models_ffi](../crates/algo_models_ffi/) that provides the foreign interfaces.
