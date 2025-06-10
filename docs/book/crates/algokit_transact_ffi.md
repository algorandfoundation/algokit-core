# AlgoKit Transact FFI

Foreign Function Interface bindings for `algokit_transact`, enabling usage from multiple programming languages.

## Overview

`algokit_transact_ffi` provides language bindings for the core `algokit_transact` functionality, supporting:

- **UniFFI Bindings** - For Python, Swift, Kotlin, and other languages
- **WebAssembly Bindings** - For JavaScript/TypeScript usage
- **C-compatible ABI** - For integration with C/C++ and other systems languages

## Features

- `ffi_uniffi` (default) - UniFFI-based bindings
- `ffi_wasm` - WebAssembly/JavaScript bindings

## Crate Types

Built as both:

- `cdylib` - Dynamic library for FFI
- `staticlib` - Static library for embedding

## Architecture

The FFI layer uses several approaches:

1. **UniFFI** - Generates bindings automatically from Rust definitions
2. **wasm-bindgen** - Provides seamless WebAssembly integration
3. **Custom macros** - Uses `ffi_macros` for specialized binding generation

## Key Dependencies

- `algokit_transact` - Core functionality
- `ffi_macros` - Procedural macros for FFI code generation
- `uniffi` - Multi-language binding generation
- `wasm-bindgen` - WebAssembly bindings
- `tsify-next` - TypeScript type generation

## API Documentation

📚 **[View Full API Documentation](../api/algokit_transact_ffi/index.html)**

The complete API documentation with all FFI types, functions, and binding examples.

## Building

### UniFFI Bindings

```bash
cargo build --package algokit_transact_ffi --features ffi_uniffi
```

### WebAssembly Bindings

```bash
cargo build --package algokit_transact_ffi --features ffi_wasm
```

## Language Support

### Python

```python
# Example Python usage would go here
```

### JavaScript/TypeScript

```javascript
// Example JS/TS usage would go here
```

### Swift

```swift
// Example Swift usage would go here
```

## Testing

```bash
cargo test --package algokit_transact_ffi
```

For WASM-specific tests:

```bash
wasm-pack test --node
```
