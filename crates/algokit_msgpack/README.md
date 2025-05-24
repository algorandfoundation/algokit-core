# algokit_msgpack

Utilities for converting between Algorand OpenAPI JSON payloads and the compact [MessagePack](https://msgpack.org/) representation used by Algorand APIs.

## Why

Various Algorand API endpoints (e.g. `/v2/teal/simulate`) support **MessagePack** alongside plain JSON.  Re-implementing the encoding rules in every language is error-prone and hard to maintain. This crate centralises that logic in a small Rust library that is surfaced to other runtimes via [`algokit_msgpack_ffi`](../algokit_msgpack_ffi/).

## Highlights

- Encode JSON into canonical MessagePack bytes
- Decode MessagePack (or Base64-wrapped MessagePack) back to JSON
- Canonicalisation: keys are alphabetically sorted and zero / empty values are stripped to match algod behaviour
- Aimed at supporting all complex types used in API clients generated from the Algorand OpenAPI specs

## Quick Start

Add the dependency:

```bash
cargo add algokit_msgpack
```

Encoding a **SimulateRequest** body:

```rust
use algokit_msgpack::{ModelType, encode_json_to_msgpack};

let json = r#"{ "txn": { "snd": "AAAA…", "fee": 1000, "type": "pay", "amt": 123 } }"#;
let bytes = encode_json_to_msgpack(ModelType::SimulateRequest, json)?;
// `bytes` can now be POSTed to algod.
```

### Base64-encoded MessagePack

```rust
let b64 = algokit_msgpack::encode_json_to_base64_msgpack(ModelType::SimulateRequest, json)?;
```

## Extending

New models live in `src/models/`.  Implement the serde structs, then call `registry.register::<YourType>(ModelType::YourVariant)` inside `register_all_models()`.
