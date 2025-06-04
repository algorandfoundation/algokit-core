# UniFFI Bindgen

Custom UniFFI binding generation tools and utilities.

## Overview

`uniffi_bindgen` is a customized version of the UniFFI binding generator, tailored for AlgoKit Core's specific requirements.

## Purpose

This crate provides:

- Custom binding generation logic
- AlgoKit-specific type mappings
- Enhanced error handling for generated bindings
- Optimized binding templates

## Relationship to Standard UniFFI

While based on the standard UniFFI framework, this custom implementation includes:

- Specialized handling for Algorand-specific types
- Optimized serialization for blockchain data structures
- Custom error types and handling patterns
- AlgoKit-specific naming conventions

## Usage

This crate is primarily used as a build tool and is not intended for direct usage by end users. It's automatically invoked during the build process of `algokit_transact_ffi`.

## API Documentation

📚 **[View Full API Documentation](../api/uniffi-bindgen/index.html)**

The complete API documentation with all binding generation tools and utilities.

## Customizations

The key customizations include:

- **Type Mappings** - Custom handling for Algorand addresses, hashes, and signatures
- **Error Handling** - Specialized error propagation for blockchain operations
- **Performance** - Optimized serialization paths for high-frequency operations

## Building

```bash
cargo build --package uniffi-bindgen
```

## Development

When modifying the binding generator:

1. Test with all target languages
2. Verify generated bindings compile correctly
3. Run integration tests with `algokit_transact_ffi`
4. Update documentation for any new features

## Testing

```bash
cargo test --package uniffi-bindgen
```
