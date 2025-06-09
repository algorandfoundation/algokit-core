# Contributing Guide

## Principles

See the core principles in the repository's [README](../../README.md)

## Rust crates vs FFI libraries

The implementation of the rust crate should be completely seperate from the foreign interfaces. For example, [algokit_transact](../crates/algokit_transact/) does not depend on UniFFI or wasm-bindgen. Instead, there's a seperate crate [algokit_transact_ffi](../crates/algokit_transact_ffi/) that provides the foreign interfaces.

## Debugging Rust Code is VS Code

### Prerequisites

Install the [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) extension for Visual Studio Code to debug Rust code.

### Debug Configurations

The project includes pre-configured debug configurations in `.vscode/launch.json`:

- **Debug unit tests in algokit_transact**: Debug tests for the core transaction functionality
- **Debug unit tests in algokit_transact_ffi**: Debug tests for the FFI bindings

### How to Debug

1. Set breakpoints by clicking in the gutter next to line numbers
2. Go to the Debug view (`Ctrl+Shift+D` or `Cmd+Shift+D`) and select a configuration for the crate you want to debug
3. Press `F5` to start debugging
4. Use the debug toolbar to step through code (`F10` for step over, `F11` for step into)
