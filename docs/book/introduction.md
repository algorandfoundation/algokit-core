# AlgoKit Core Documentation

Welcome to the comprehensive documentation for AlgoKit Core, a collection of Rust crates that provide core functionality for Algorand blockchain development.

## Overview

AlgoKit Core consists of several interconnected crates:

- **[algokit_transact](./crates/algokit_transact.md)** - Core transaction building and signing functionality
- **[algokit_transact_ffi](./crates/algokit_transact_ffi.md)** - Foreign Function Interface bindings for multiple languages
- **[ffi_macros](./crates/ffi_macros.md)** - Procedural macros for FFI code generation
- **[uniffi_bindgen](./crates/uniffi_bindgen.md)** - Custom UniFFI binding generation tools

## API Documentation

📚 **[Complete API Documentation](./api/index.html)**

Browse the full Rust API documentation with detailed type information, function signatures, and code examples for all crates.

## Architecture

This project follows a layered architecture:

1. **Core Layer** (`algokit_transact`) - Pure Rust implementations of Algorand transaction logic
2. **FFI Layer** (`algokit_transact_ffi`) - Language bindings and foreign function interfaces
3. **Tooling Layer** (`ffi_macros`, `uniffi_bindgen`) - Development and build-time utilities

## Getting Started

Each crate has its own documentation with examples and API references. Start with the [algokit_transact](./crates/algokit_transact.md) crate to understand the core functionality, then explore the FFI bindings for your target language.

## Research and Decisions

This documentation also includes our research findings and architectural decision records to help you understand the reasoning behind our design choices:

- **[Research](./research/README.md)** - Technical research and analysis
- **[Architecture Decisions](./decisions/README.md)** - Documented decisions and their rationale
- **[Contributing](./contributing/README.md)** - Guidelines for contributors

## Building Documentation

To build this documentation locally:

```bash
cargo run --bin build-docs --manifest-path docs/Cargo.toml
```

The generated documentation will be available in the `target/docs` directory.
