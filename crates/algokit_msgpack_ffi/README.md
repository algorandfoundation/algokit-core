# AlgoKit FFI MessagePack Bindings

FFI bindings for Algorand API operations with **canonical MessagePack encoding support**.

## Features

This crate provides:

- **Canonical MessagePack encoding** that conforms to [Algorand's canonical encoding rules](https://developer.algorand.org/docs/get-details/encoding/)
- FFI bindings for WebAssembly (WASM) and UniFFI 
- Generated models from Algorand's OpenAPI specification
- Encoding/decoding functions for key API request/response types

## Algorand Canonical MessagePack Encoding

This implementation follows Algorand's canonical encoding rules:

### 1. **Lexicographic Key Sorting**
- Map keys are serialized in lexicographic (alphabetical) order
- Implemented using `rmp_serde::Serializer::with_struct_map()`

### 2. **Empty Value Omission** 
- Empty/zero-value fields are omitted from the encoding
- Handled by serde's `skip_serializing_if = "Option::is_none"` attributes on optional fields
- Zero values (0, false, empty strings/arrays) are excluded

### 3. **Efficient Integer Encoding**
- Positive integers are encoded as unsigned values when possible
- Uses the smallest possible integer representation (8-bit, 16-bit, 32-bit, 64-bit)
- Handled automatically by `rmp-serde`

### 4. **Binary Format Support**
- Binary arrays use the modern MessagePack "bin" format family
- Byte arrays are properly distinguished from strings
- Use `#[serde(with = "serde_bytes")]` for optimal binary encoding

### 5. **Deterministic Encoding**
- Same input always produces identical output
- Essential for cryptographic signatures and hash verification

## API Usage

### Encoding Models to MessagePack

```rust
use algokit_msgpack_ffi::*;

// Create an account model
let account = Account::new(
    "AAAA...".to_string(),
    1000000,
    100000,
    900000,
    // ... other fields
);

// Encode using canonical rules
let msgpack_bytes = encode_account(account)?;
```

### Decoding MessagePack to Models

```rust
// Decode account from msgpack bytes
let account = decode_account(msgpack_bytes)?;
```

### Supported Operations

#### Encoding (Request Bodies)
- `encode_simulate_request(SimulateRequest)` - POST /v2/transactions/simulate
- `encode_dryrun_request(DryrunRequest)` - POST /v2/teal/dryrun  

#### Decoding (Response Bodies)  
- `decode_account(Vec<u8>)` - GET /v2/accounts/{address}
- `decode_error_response(Vec<u8>)` - Error responses
- `decode_pending_transaction_response(Vec<u8>)` - GET /v2/transactions/pending/{txid}
- `decode_ledger_state_delta_for_transaction_group(Vec<u8>)` - GET /v2/deltas/{round}/txn/group

#### Bidirectional (Request & Response)
- `encode_account(Account)` / `decode_account(Vec<u8>)` - Account information

## Compliance Verification

The implementation includes comprehensive tests that verify:

- ✅ **Key sorting**: Struct fields are serialized in alphabetical order
- ✅ **Empty omission**: Optional None fields are excluded from encoding  
- ✅ **Integer compliance**: Positive integers use unsigned encoding
- ✅ **Binary support**: Byte arrays use proper MessagePack binary format
- ✅ **Round-trip consistency**: Encode → Decode preserves data integrity

## Comparison with Reference Implementations

### JavaScript SDK
This implementation follows the same canonical rules as [`js-algorand-sdk`](https://github.com/algorand/js-algorand-sdk/blob/develop/src/encoding/encoding.ts):
- Uses `sortKeys: true` equivalent behavior
- Omits empty values like the JS SDK's `omitEmpty` functionality  

### Python SDK
Aligns with [`py-algorand-sdk`](https://github.com/algorand/py-algorand-sdk/blob/master/algosdk/encoding.py):
- Implements recursive key sorting like `_sort_dict()`
- Uses `use_bin_type=True` equivalent for binary data

## Development

### Running Tests

```bash
cargo test
```

The test suite includes:
- Canonical encoding verification
- Round-trip encoding/decoding tests  
- Compliance tests against Algorand's rules
- Binary data handling validation

### Building

```bash
# Standard build
cargo build

# With WASM support
cargo build --features ffi_wasm

# With UniFFI support  
cargo build --features ffi_uniffi
```

## References

- [Algorand Canonical Encoding Specification](https://developer.algorand.org/docs/get-details/encoding/)
- [MessagePack Specification](https://msgpack.org/)
- [ARC-4 ABI Specification](https://github.com/algorandfoundation/ARCs/blob/main/ARCs/arc-0004.md)

## API

### Encoding Functions

- `encode_teal_value(model: TealValue) -> Result<Vec<u8>, MsgpackError>`
- `encode_account(model: Account) -> Result<Vec<u8>, MsgpackError>`
- `encode_simulate_request(model: SimulateRequest) -> Result<Vec<u8>, MsgpackError>`

### Decoding Functions

- `decode_teal_value(data: Vec<u8>) -> Result<TealValue, MsgpackError>`
- `decode_account(data: Vec<u8>) -> Result<Account, MsgpackError>`
- `decode_error_response(data: Vec<u8>) -> Result<ErrorResponse, MsgpackError>`

### Generic Function (for convenience)

- `encode_msgpack_generic(model_json: String) -> Result<Vec<u8>, MsgpackError>` - Accepts JSON and auto-detects the model type

## Usage Examples

### Rust

```rust
use algokit_msgpack_ffi::*;

// Create a TealValue model
let teal_value = TealValue {
    r#type: 1,
    bytes: "test".to_string(),
    uint: 42,
};

// Encode directly with the typed model
let encoded = encode_teal_value(teal_value)?;

// Decode back to the typed model
let decoded = decode_teal_value(encoded)?;
println!("Decoded: {:?}", decoded);

// Create an Account model
let account = Account::new(
    "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_string(),
    1000000,
    100000,
    900000,
    0, 0, 0, 0, 0, 0,
    1000,
    "Online".to_string(),
);

let encoded_account = encode_account(account)?;
let decoded_account = decode_account(encoded_account)?;
```

### JavaScript (via wasm-bindgen)

```javascript
import { 
  TealValue, Account, SimulateRequest,
  encode_teal_value, decode_teal_value,
  encode_account, decode_account,
  encode_simulate_request
} from './pkg/algokit_msgpack_ffi.js';

// Create a TealValue object directly
const tealValue = new TealValue(1, "test", 42);

// Encode the typed object
const encoded = encode_teal_value(tealValue);

// Decode back to typed object
const decoded = decode_teal_value(encoded);
console.log('Decoded:', decoded);

// Create an Account object
const account = new Account(
  "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
  1000000,
  100000,
  900000,
  0, 0, 0, 0, 0, 0,
  1000,
  "Online"
);

const encodedAccount = encode_account(account);
const decodedAccount = decode_account(encodedAccount);
```

### Python (via UniFFI)

```python
from algokit_msgpack import (
    TealValue, Account, SimulateRequest,
    encode_teal_value, decode_teal_value,
    encode_account, decode_account,
    encode_simulate_request,
    MsgpackError
)

# Create a TealValue object directly
teal_value = TealValue(type=1, bytes="test", uint=42)

try:
    # Encode the typed object
    encoded = encode_teal_value(teal_value)
    
    # Decode back to typed object
    decoded = decode_teal_value(encoded)
    print(f"Decoded: {decoded}")
    
    # Create an Account object
    account = Account(
        address="AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        amount=1000000,
        min_balance=100000,
        amount_without_pending_rewards=900000,
        total_apps_opted_in=0,
        total_assets_opted_in=0,
        total_created_apps=0,
        total_created_assets=0,
        pending_rewards=0,
        rewards=0,
        round=1000,
        status="Online"
    )
    
    encoded_account = encode_account(account)
    decoded_account = decode_account(encoded_account)
    
except MsgpackError.EncodingNotSupported:
    print("This model type does not support encoding")
except MsgpackError.DecodingNotSupported:
    print("This model type does not support decoding")
```

## Error Handling

The API uses a single `MsgpackError` enum with the following variants:

- `EncodingNotSupported`: The model type doesn't support encoding
- `DecodingNotSupported`: The model type doesn't support decoding  
- `SerializationError(String)`: Error during msgpack encoding
- `DeserializationError(String)`: Error during msgpack decoding

## Benefits of This Approach

1. **Type Safety**: Work directly with typed model objects instead of JSON strings
2. **Performance**: No JSON serialization/deserialization overhead
3. **IDE Support**: Full autocomplete and type checking in your IDE
4. **Consistency**: Same API across Rust, JavaScript, and Python
5. **Error Prevention**: Compile-time checks prevent passing wrong model types

## Building

### For UniFFI (Python, Swift, etc.)

```bash
cargo build --features ffi_uniffi
```

### For wasm-bindgen (JavaScript)

```bash
cargo build --features ffi_wasm
```

### For both

```bash
cargo build --features ffi_uniffi,ffi_wasm
```

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.0.1
- Package version: 0.1.0
- Generator version: 7.12.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `algokit_msgpack_ffi` and add the following to `Cargo.toml` under `[dependencies]`:

```
algokit_msgpack_ffi = { path = "./algokit_msgpack_ffi" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------


## Documentation For Models

 - [AbortCatchup200Response](docs/AbortCatchup200Response.md)
 - [Account](docs/Account.md)
 - [AccountApplicationInformation200Response](docs/AccountApplicationInformation200Response.md)
 - [AccountAssetHolding](docs/AccountAssetHolding.md)
 - [AccountAssetInformation200Response](docs/AccountAssetInformation200Response.md)
 - [AccountAssetsInformation200Response](docs/AccountAssetsInformation200Response.md)
 - [AccountParticipation](docs/AccountParticipation.md)
 - [AccountStateDelta](docs/AccountStateDelta.md)
 - [AddParticipationKey200Response](docs/AddParticipationKey200Response.md)
 - [AppCallLogs](docs/AppCallLogs.md)
 - [Application](docs/Application.md)
 - [ApplicationInitialStates](docs/ApplicationInitialStates.md)
 - [ApplicationKvStorage](docs/ApplicationKvStorage.md)
 - [ApplicationLocalReference](docs/ApplicationLocalReference.md)
 - [ApplicationLocalState](docs/ApplicationLocalState.md)
 - [ApplicationParams](docs/ApplicationParams.md)
 - [ApplicationStateOperation](docs/ApplicationStateOperation.md)
 - [ApplicationStateSchema](docs/ApplicationStateSchema.md)
 - [Asset](docs/Asset.md)
 - [AssetHolding](docs/AssetHolding.md)
 - [AssetHoldingReference](docs/AssetHoldingReference.md)
 - [AssetParams](docs/AssetParams.md)
 - [AvmKeyValue](docs/AvmKeyValue.md)
 - [AvmValue](docs/AvmValue.md)
 - [Box](docs/Box.md)
 - [BoxReference](docs/BoxReference.md)
 - [BuildVersion](docs/BuildVersion.md)
 - [DebugSettingsProf](docs/DebugSettingsProf.md)
 - [DryrunRequest](docs/DryrunRequest.md)
 - [DryrunSource](docs/DryrunSource.md)
 - [DryrunState](docs/DryrunState.md)
 - [DryrunTxnResult](docs/DryrunTxnResult.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [EvalDelta](docs/EvalDelta.md)
 - [EvalDeltaKeyValue](docs/EvalDeltaKeyValue.md)
 - [GetApplicationBoxes200Response](docs/GetApplicationBoxes200Response.md)
 - [GetBlock200Response](docs/GetBlock200Response.md)
 - [GetBlockHash200Response](docs/GetBlockHash200Response.md)
 - [GetBlockLogs200Response](docs/GetBlockLogs200Response.md)
 - [GetBlockTimeStampOffset200Response](docs/GetBlockTimeStampOffset200Response.md)
 - [GetBlockTxids200Response](docs/GetBlockTxids200Response.md)
 - [GetPendingTransactionsByAddress200Response](docs/GetPendingTransactionsByAddress200Response.md)
 - [GetStatus200Response](docs/GetStatus200Response.md)
 - [GetSupply200Response](docs/GetSupply200Response.md)
 - [GetSyncRound200Response](docs/GetSyncRound200Response.md)
 - [GetTransactionGroupLedgerStateDeltasForRound200Response](docs/GetTransactionGroupLedgerStateDeltasForRound200Response.md)
 - [GetTransactionProof200Response](docs/GetTransactionProof200Response.md)
 - [LedgerStateDeltaForTransactionGroup](docs/LedgerStateDeltaForTransactionGroup.md)
 - [LightBlockHeaderProof](docs/LightBlockHeaderProof.md)
 - [ParticipationKey](docs/ParticipationKey.md)
 - [PendingTransactionResponse](docs/PendingTransactionResponse.md)
 - [RawTransaction200Response](docs/RawTransaction200Response.md)
 - [ScratchChange](docs/ScratchChange.md)
 - [SimulateInitialStates](docs/SimulateInitialStates.md)
 - [SimulateRequest](docs/SimulateRequest.md)
 - [SimulateRequestTransactionGroup](docs/SimulateRequestTransactionGroup.md)
 - [SimulateTraceConfig](docs/SimulateTraceConfig.md)
 - [SimulateTransaction200Response](docs/SimulateTransaction200Response.md)
 - [SimulateTransactionGroupResult](docs/SimulateTransactionGroupResult.md)
 - [SimulateTransactionResult](docs/SimulateTransactionResult.md)
 - [SimulateUnnamedResourcesAccessed](docs/SimulateUnnamedResourcesAccessed.md)
 - [SimulationEvalOverrides](docs/SimulationEvalOverrides.md)
 - [SimulationOpcodeTraceUnit](docs/SimulationOpcodeTraceUnit.md)
 - [SimulationTransactionExecTrace](docs/SimulationTransactionExecTrace.md)
 - [StartCatchup200Response](docs/StartCatchup200Response.md)
 - [StateProof](docs/StateProof.md)
 - [StateProofMessage](docs/StateProofMessage.md)
 - [TealCompile200Response](docs/TealCompile200Response.md)
 - [TealDisassemble200Response](docs/TealDisassemble200Response.md)
 - [TealDryrun200Response](docs/TealDryrun200Response.md)
 - [TealKeyValue](docs/TealKeyValue.md)
 - [TealValue](docs/TealValue.md)
 - [TransactionParams200Response](docs/TransactionParams200Response.md)
 - [Version](docs/Version.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


