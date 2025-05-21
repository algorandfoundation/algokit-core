# algokit_msgpack_ffi

Foreign-function interface bindings for the core [`algokit_msgpack`](../algokit_msgpack/) crate.

The wrappers expose the high-performance MessagePack encoder/decoder to other runtimes via either **UniFFI** or **WebAssembly**.

## Feature Flags

| Feature | Default? | Purpose |
|---------|----------|---------|
| `ffi_uniffi` | ✅ | Generate UniFFI bindings (C header + Swift / Kotlin / Python, etc.) |
| `ffi_wasm`   | ❌ | Build WebAssembly+JS bindings through `wasm-bindgen` and `tsify-next` |

Enable the desired target when compiling, e.g.

```bash
# WebAssembly build
cargo build -p algokit_msgpack_ffi --features ffi_wasm --target wasm32-unknown-unknown
```

## Key Files

- `src/lib.rs` – thin wrappers that translate between public FFI types and the internal API
- `uniffi.toml` & `build.rs` – scaffolding generation when building with `ffi_uniffi`

## Usage Examples

### TypeScript (wasm-bindgen)

```ts
import { encodeJsonToBase64Msgpack, ModelType } from "algokit_msgpack";

const b64 = encodeJsonToBase64Msgpack(
  ModelType.SimulateRequest,
  JSON.stringify({ txn: {/* … */} })
);
```

### Python (UniFFI)

```py
from algokit_msgpack import encode_json_to_msgpack, ModelType

payload = encode_json_to_msgpack(
    ModelType.SimulateRequest,
    '{"txn": {...}}'
)
```

---
See the [core crate](../algokit_msgpack/) for implementation details; this crate only handles binding generation.
