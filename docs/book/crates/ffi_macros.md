# FFI Macros

Procedural macros for generating Foreign Function Interface (FFI) bindings in AlgoKit Core.

## Overview

`ffi_macros` provides procedural macros that automatically generate the necessary attributes and code for creating FFI bindings across multiple platforms and languages. These macros streamline the process of making Rust code accessible from WebAssembly, UniFFI, and other FFI contexts.

## Purpose

This crate provides three key procedural macros:

- **`#[ffi_func]`** - Decorates functions for FFI export
- **`#[ffi_record]`** - Decorates structs for cross-language serialization
- **`#[ffi_enum]`** - Decorates enums for cross-language serialization

## Macros

### `#[ffi_func]`

Automatically applies the necessary attributes for function export:

- Adds `uniffi::export` attributes for UniFFI bindings

### `#[ffi_record]`

Prepares structs for FFI serialization by adding:

- Serde serialization/deserialization derives
- WebAssembly-compatible type generation with Tsify
- UniFFI Record derivation
- Proper handling of Option fields
- Automatic camelCase field renaming for JavaScript

### `#[ffi_enum]`

Prepares enums for FFI serialization with:

- Serde serialization/deserialization derives
- WebAssembly-compatible type generation
- UniFFI Enum derivation
- Automatic camelCase variant renaming

## Features

The macros automatically handle:

- **Multi-target Support** - Conditional compilation for different FFI targets
- **Type Conversion** - Automatic handling of Rust types to FFI-compatible types
- **Naming Conventions** - Automatic conversion to target language conventions
- **Optional Fields** - Special handling for Option types in different FFI contexts

## Key Dependencies

- `syn` - Rust code parsing and manipulation
- `quote` - Code generation utilities
- `convert_case` - String case conversion for naming conventions

## Usage Examples

```rust
use ffi_macros::{ffi_func, ffi_record, ffi_enum};

#[ffi_func]
pub fn create_transaction() -> Transaction {
    // Function automatically exported for FFI
}

#[ffi_record]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: Option<u64>,
}

#[ffi_enum]
pub enum TransactionType {
    Payment,
    AssetTransfer,
    AppCall,
}
```

## Crate Type

This is a procedural macro crate (`proc-macro = true`), used at compile time to generate code.

## API Documentation

📚 **[View Full API Documentation](../api/ffi_macros/index.html)**

The complete API documentation with all macro definitions and usage examples.

## Development

When modifying the macros:

1. Test with all FFI targets (WebAssembly, UniFFI)
1. Verify generated code compiles correctly
1. Update tests for new functionality
1. Ensure backward compatibility

## Testing

```bash
cargo test --package ffi_macros
```

## Integration

This crate is primarily used by `algokit_transact_ffi` and should not be used directly by end users. The macros are automatically applied during the FFI binding generation process.
