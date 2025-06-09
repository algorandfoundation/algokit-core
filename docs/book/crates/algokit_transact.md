# AlgoKit Transact

The core crate providing transaction building and signing functionality for the Algorand blockchain.

## Overview

`algokit_transact` is the foundational crate that implements:

- Transaction building and serialization

## Features

- **Transaction Types**: Support for all Algorand transaction types
- **Serialization**: MessagePack encoding/decoding
- **Test Utilities**: Optional testing helpers (feature: `test_utils`)

## Crate Type

This crate is built as both:

- `cdylib` - For dynamic linking in FFI scenarios
- `rlib` - For standard Rust library usage

## Key Dependencies

- `serde` - Serialization framework
- `rmp-serde` - MessagePack serialization
- `sha2` - SHA-256 hashing
- `ed25519-dalek` - Ed25519 signatures (optional, test_utils feature)

## Usage Examples

```rust
use algokit_transact::*;

// Example usage would go here
// This would typically include transaction building examples
```

## API Documentation

📚 **[View Full API Documentation](../api/algokit_transact/index.html)**

The complete API documentation with all transaction types, signing utilities, and core functionality.

## Testing

The crate includes comprehensive tests. Run them with:

```bash
cargo test --package algokit_transact
```

For tests that require the test utilities:

```bash
cargo test --package algokit_transact --features test_utils
```
