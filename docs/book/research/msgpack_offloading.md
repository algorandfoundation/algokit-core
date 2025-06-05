# Offloading MessagePack Encoding/Decoding to Rust for Algorand OpenAPI Clients

## Context

Various Algorand APIs expose endpoints that can exchange data in **[MessagePack](https://msgpack.org/)** – notably transaction simulation (`/v2/teal/simulate`) and the REST variants that support returning `application/msgpack` responses.  Historically each language-specific client (Go, JS, Python, …) re-implemented the same canonicalisation rules:

* alphabetical map key ordering
* removal of zero/empty values
* special-casing of byte slices vs. integer arrays

Keeping N implementations correct and in sync is very time-consuming and a consistent source of bugs.

## Approach

1. Model the exact request/response structures with **Serde** in a dedicated Rust crate (`algokit_msgpack`).
2. Implement a single, well-tested conversion pipeline:
   * JSON ➟ `serde_json::Value`
   * canonicalisation (sorting / zero-value stripping)
   * `rmp` encoding ➟ `Vec<u8>` (or Base-64 wrapper)
   * inverse pipeline for decoding
3. Expose the converter to other runtimes through `algokit_msgpack_ffi`:
   * **UniFFI**: generates bindings for Python, Swift, Kotlin, C#…
   * **wasm-bindgen** + `tsify-next`: WebAssembly module for TypeScript / browsers
4. Wire the generated OpenAPI clients so that whenever the spec marks an endpoint as `application/msgpack`, the request/response bodies are funnelled through the FFI helper instead of the language's native MsgPack library.  The client code remains 100 % idiomatic for each language; only the (de)serialisation step jumps into Rust.

### Data Flow

```ascii
TypeScript ↔ OpenAPI client
           │  (JSON)
           ▼
wasm-bindgen wrapper  ──▶ algokit_msgpack (Rust) ──▶ MessagePack bytes ──▶ algod
```

Python and other UniFFI targets follow a similar path through a native shared library rather than WebAssembly.

## Benefits

* **Single source of truth** – business logic lives in one place.
* **Portability** – leveraging uniffi, the same codebase can be used in any language that supports the target language runtime.

## Limitations & Future Work

* Only two models (`SimulateRequest`, `SimulateTransaction200Response`) are currently implemented – coverage will grow as the OpenAPI surface is expanded.
* Find a more sophisticated way to filter out and further automate model generation from algod spec that only includes models related to msgpack.
* Expose SignedTransaction via ffi, modify the autogeneration for rust algod models to reference that core abstraction.
* Remove or refine model registry mechanism, must be unified with AlgokitMessagePack trait once SignedTransaction is exposed and can be referenced in the algod spec.
