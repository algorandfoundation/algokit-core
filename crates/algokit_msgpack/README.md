# algokit_msgpack

Utilities for converting between Algorand OpenAPI JSON payloads and the compact [MessagePack](https://msgpack.org/) representation used by Algorand APIs.

## Why

Various Algorand API endpoints (e.g. `/v2/transactions/simulate`) support **MessagePack** alongside plain JSON.  Re-implementing the encoding rules in every language is error-prone and hard to maintain. This crate centralises that logic in a small Rust library that is surfaced to other runtimes via [`algokit_msgpack_ffi`](../algokit_msgpack_ffi/).

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

let json = r#"{
  "txn-groups": [{
    "txns": ["gqNzaWfEQC0RQ1E6Y+/iS6luFP6Q9c6Veo838jRIABcV+jSzetx61nlrmasonRDbxN02mbCESJw98o7IfKgQvSMvk9kE0gqjdHhuiaNhbXTOAA9CQKNmZWXNA+iiZnYzo2dlbqxkb2NrZXJuZXQtdjGiZ2jEIEeJCm8ejvOqNCXVH+4GP95TdhioDiMH0wMRTIiwAmAUomx2zQQbo3JjdsQg/x0nrFM+VxALq2Buu1UscgDBy0OKIY2MGnDzg8xkNaOjc25kxCD/HSesUz5XEAurYG67VSxyAMHLQ4ohjYwacPODzGQ1o6R0eXBlo3BheQ=="]
  }],
  "allow-empty-signatures": true,
  "allow-more-logging": true,
  "allow-unnamed-resources": true,
  "exec-trace-config": {
    "enable": true,
    "stack-change": true,
    "scratch-change": true,
    "state-change": true
  }
}"#;
let bytes = encode_json_to_msgpack(ModelType::SimulateRequest, json)?;
// `bytes` can now be POSTed to algod's /v2/transactions/simulate endpoint.
```

### Base64-encoded MessagePack

```rust
let b64 = algokit_msgpack::encode_json_to_base64_msgpack(ModelType::SimulateRequest, json)?;
```

## Extending

New models live in `src/models/`.  Implement the serde structs, then call `registry.register::<YourType>(ModelType::YourVariant)` inside `register_all_models()`.
