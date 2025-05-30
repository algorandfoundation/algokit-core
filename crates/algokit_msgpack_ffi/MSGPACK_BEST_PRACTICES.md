# MessagePack Encoding Best Practices for Algorand

## Overview

This document outlines the proper approach to msgpack encoding and decoding for Algorand transactions, specifically addressing how to handle base64-encoded signed transactions to avoid double serialization.

## Key Principles

### 1. Canonical MessagePack Encoding Rules

Algorand follows strict canonical msgpack encoding rules:

- **Integers**: Must be encoded to the smallest type possible (0-255→8bit, 256-65535→16bit, etc.)
- **Field names**: Must be sorted alphabetically
- **Empty fields**: All empty and 0 fields should be omitted (handled by `skip_serializing_if`)
- **Positive numbers**: Must be encoded as uint (handled by rmp-serde)
- **Binary data**: Should use binary blob format, strings for text data

### 2. Transaction Encoding Flow

The correct flow for transaction encoding is:

```
SignedTransaction (Rust object) 
    → msgpack_encode() 
    → base64_encode() 
    → String (for API transmission)
```

**Never double-encode** by treating base64 strings as Rust objects during subsequent encoding.

## Implementation Patterns

### ✅ CORRECT: Using Vec<String> for Pre-encoded Transactions

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimulateRequestTransactionGroup {
    #[serde(rename = "txns")]
    pub txns: Vec<String>, // ✅ Base64-encoded msgpack strings
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DryrunRequest {
    #[serde(rename = "txns")]
    pub txns: Vec<String>, // ✅ Base64-encoded msgpack strings
}
```

### ❌ INCORRECT: Using Vec<SignedTransaction> for API Models

```rust
// DON'T DO THIS for API request models
pub struct SimulateRequestTransactionGroup {
    pub txns: Vec<SignedTransaction>, // ❌ Will cause double serialization
}
```

### Helper Functions

```rust
/// Convert a SignedTransaction to base64-encoded string for API requests
pub fn encode_signed_transaction_to_base64(
    signed_txn: &SignedTransaction
) -> Result<String, MsgpackError> {
    let msgpack_bytes = encode_msgpack_canonical(signed_txn)?;
    Ok(base64::encode(msgpack_bytes))
}

/// Canonical msgpack encoding following Algorand's rules
pub fn encode_msgpack_canonical<T: MsgpackEncodable>(
    model: &T
) -> Result<Vec<u8>, MsgpackError> {
    let mut buf = Vec::new();
    let mut serializer = rmp_serde::Serializer::new(&mut buf).with_struct_map();
    model.serialize(&mut serializer)?;
    Ok(buf)
}
```

## API Compatibility

### Algorand API Specification

According to the Algorand API specification:

> "Each transaction must be provided as a base64-encoded, canonically encoded SignedTransaction object"

This means:
1. Transaction objects are **already** msgpack-encoded
2. Then **base64-encoded** for safe JSON transport
3. Passed as **strings** in API requests

### Cross-Platform Consistency

**TypeScript Implementation:**
```typescript
export class SimulateRequestTransactionGroup {
    'txns': Array<string>; // ✅ Correct
}
```

**Rust Implementation (Fixed):**
```rust
pub struct SimulateRequestTransactionGroup {
    pub txns: Vec<String>, // ✅ Now matches TypeScript
}
```

## Usage Examples

### Creating a Simulate Request

```rust
// 1. Create and sign your transactions normally
let signed_txn = SignedTransaction::new(transaction, signature);

// 2. Encode to base64 string
let base64_txn = encode_signed_transaction_to_base64(&signed_txn)?;

// 3. Use in API request
let group = SimulateRequestTransactionGroup::new(vec![base64_txn]);
let simulate_request = SimulateRequest::new(vec![group]);

// 4. Encode the entire request for transmission
let encoded_request = encode_msgpack_canonical(&simulate_request)?;
```

### Working with Pre-encoded Transactions

```rust
// When you receive base64-encoded transactions from a client
let encoded_transactions = vec![
    "gqRsc2lngqNhcmeRxAhwcmVpbWFnZaFsxJc...".to_string(),
    "gqRsc2lngqNhcmeRxAhwcmVpbWFnZaFsxJc...".to_string(),
];

// Use directly without decoding/re-encoding
let group = SimulateRequestTransactionGroup::new(encoded_transactions);
```

## Testing Guidelines

Always test both encoding and decoding paths:

```rust
#[test]
fn test_round_trip_encoding() {
    let request = create_test_simulate_request();
    
    // Encode
    let encoded = encode_simulate_request(request.clone()).unwrap();
    
    // Decode
    let decoded: SimulateRequest = decode_msgpack(&encoded).unwrap();
    
    // Verify integrity
    assert_eq!(request, decoded);
}
```

## Common Pitfalls

1. **Double Serialization**: Using `Vec<SignedTransaction>` in API models
2. **Incorrect Field Types**: Not matching the API specification exactly
3. **Missing Canonical Rules**: Not following Algorand's msgpack requirements
4. **Cross-Platform Inconsistency**: Different types in different language implementations

## Benefits of This Approach

1. **Performance**: Avoids unnecessary decode/re-encode cycles
2. **Correctness**: Matches API specification exactly
3. **Consistency**: Same pattern across all languages and models
4. **Security**: Preserves exact transaction signatures without modification 
