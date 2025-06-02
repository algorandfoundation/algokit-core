# Ergonomic and Efficient (De)Serialization for Algod Models via FFI

---

## 1. Current Approach (Summary)
- **All models** are generated from the official Algorand OpenAPI spec and exported via Rust FFI (UniFFI/wasm-bindgen).
- **Per-model encode/decode functions** are generated in Rust using macros, due to FFI limitations with generics.
- **Python/TypeScript helpers** dynamically resolve the correct FFI function for each model using naming conventions.
- **JSON:** All models support encode/decode to/from JSON.
- **Msgpack:** Only a subset of models support encode/decode to/from msgpack, as required.
- **No vendor extension automation**; spec is kept in sync with the official Go repo.

---

## 2. Design Constraints
- **OpenAPI Generator (mustache) is required** for future multi-language support.
- **All models must be exported**; selective exports are not allowed.
- **No generics at the FFI boundary**; per-model methods are required.
- **Minimize spec drift**; no custom vendor extension automation.

---

## 3. Proposed Refinements

### A. Maintain Per-Model FFI Functions
- Continue using macros to generate per-model encode/decode functions for both JSON and msgpack.
- **Macro Note:** Ensure that generated methods are consistent and idiomatic in both Python and TypeScript:
  - Python: `teal_key_value_to_json`, `teal_key_value_from_json` (snake_case)
  - TypeScript: `tealKeyValueToJson`, `tealKeyValueFromJson` (camelCase)
- Strictly follow naming conventions for all models (e.g., `account_to_json`, `encode_account`).

### B. Centralize Model Resolution in Python/TS
- Keep the dynamic function resolution logic in helpers (as in `helpers.py`), with clear error handling and documentation.

### C. Explicit Subset for Msgpack
- In Rust, use a macro attribute or a config file to mark which models should have msgpack encode/decode generated.
- In Python/TS, expose only those models for msgpack encode/decode; raise clear errors for unsupported models.

### D. Testing and Validation
- Ensure property-based and roundtrip tests for all JSON (de)serialization.
- For msgpack, test only the subset of models with encode/decode support.

### E. Documentation
- Document the (de)serialization API, model support matrix, and error cases in the downstream packages.

---

## 4. Architecture Diagram

```mermaid
flowchart TD
    subgraph Rust FFI Crate
        A[OpenAPI Spec] --> B[Mustache Templates]
        B --> C[All Rust Models]
        C --> D[Per-Model JSON encode/decode]
        C --> E[Per-Model Msgpack encode/decode (subset)]
    end
    D & E --> F[UniFFI/wasm-bindgen]
    F --> G[Python/TS FFI Bindings]
    G --> H[helpers.py / helpers.ts]
    H --> I[User Code]
```

---

## 5. Actionable Steps

1. **Rust Macros**
   - Ensure macros generate per-model encode/decode for all models (JSON) and a configurable subset (msgpack).
   - Ensure method names are consistent and idiomatic in both Python (snake_case) and TypeScript (camelCase).
   - Document macro usage and naming conventions.

2. **Python/TS Helpers**
   - Maintain dynamic function resolution as in `helpers.py`.
   - Document error handling and supported models for msgpack.

3. **Testing**
   - Add/maintain roundtrip tests for all models (JSON).
   - Add/maintain roundtrip tests for msgpack subset.

4. **Documentation**
   - Update downstream package docs to clarify which models support msgpack and error handling for unsupported models.

---

This plan is tailored to your constraints: OpenAPI Generator, all models exported, no generics at FFI, minimal spec drift, and consistent idiomatic method naming in both Python and TypeScript.
