# AlgoKit ABI

A library for encoding and decoding Algorand ABI types, as defined in [ARC-4](https://arc.algorand.foundation/ARCs/arc-0004).

## Features

- **Full ARC-4 compatibility**: Supports all ABI types, including `uint<N>`, `ufixed<N>x<M>`, `bool`, `byte`, `address`, `string`, arrays, and tuples.
- **Encoding and decoding**: Provides a simple and intuitive API for encoding and decoding ABI values.
- **Method selector generation**: Includes utilities for generating ABI method selectors.
- **Type parsing**: Can parse ABI type strings into structured `ABIType` objects.

## Type String Representations

The library supports parsing of all ARC-4 type strings:

| Type              | Description                                                                                                | Example                 |
| ----------------- | ---------------------------------------------------------------------------------------------------------- | ----------------------- |
| `uint<N>`         | An `N`-bit unsigned integer (`8 <= N <= 512`, `N % 8 = 0`).                                                  | `uint64`                |
| `ufixed<N>x<M>`   | An `N`-bit unsigned fixed-point number with `M` decimal places (`8 <= N <= 512`, `N % 8 = 0`, `0 < M <= 160`). | `ufixed128x10`          |
| `bool`            | A boolean.                                                                                                 | `bool`                  |
| `byte`            | An 8-bit unsigned integer. Alias for `uint8`.                                                              | `byte`                  |
| `address`         | A 32-byte Algorand address. Alias for `byte[32]`.                                                          | `address`               |
| `string`          | A dynamic-length UTF-8 encoded string. Alias for `byte[]`.                                                 | `string`                |
| `<type>[<N>]`     | A fixed-length array of `N` elements of `<type>`.                                                          | `uint32[10]`            |
| `<type>[]`        | A dynamic-length array of elements of `<type>`.                                                            | `bool[]`                |
| `(T1,T2,...)`     | A tuple of types.                                                                                          | `(uint64,string,bool)` |

**Note:** Reference types (`account`, `asset`, `application`) and transaction types (`txn`, `pay`, etc.) are not supported as standalone types to be encoded/decoded, as they relate to transaction-level properties rather than ABI-level values.

## Example

```rust
use algokit_abi::{ABIMethod, ABIType, ABIValue};
use num_bigint::BigUint;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an ABI type from a string
    let abi_type = ABIType::from_str("(uint64,string)")?;
    // Create an ABI value
    let abi_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(123u64)),
        ABIValue::String("hello".to_string()),
    ]);
    // Encode the value
    let encoded = abi_type.encode(&abi_value)?;
    // Decode the value
    let decoded = abi_type.decode(&encoded)?;
    assert_eq!(abi_value, decoded);
    // Get the method selector for a given method signature
    let method = ABIMethod::from_str("my_method(uint64,string)void")?;
    let selector = method.selector()?;
    assert_eq!(selector, vec![40, 29, 20, 227]);

    Ok(())
}
```

## Testing

This crate uses [insta](https://insta.rs/) for snapshot testing of ARC56 contract parsing and serialization. If tests fail due to snapshot changes:

1. Review changes carefully to ensure they're intentional
2. Run `cargo insta review` to interactively approve/reject snapshot updates
3. Commit updated `.snap` files with your changes

For more information, see the [contributing guide](../../docs/book/contributing/contributing_guide.md#snapshot-testing-abi-crate).
