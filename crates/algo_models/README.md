# algo_models

Rust library for encoding and decoding Algorand transactions. Primary goal is to enable transaction encoding/decoding for creating transactions and attaching signatures (or program).

See [algo_models_ffi](../algo_models_ffi/) for foreign interfaces.

## Features

### Encoding and Decoding Support

- [x] Payment transactions
- [x] Asset transfer transactions
- [ ] Asset freeze transactions
- [ ] Asset configuration transactions
- [ ] Application call transactions
- [ ] Key registration transactions
- [ ] State proof transactions
- [ ] Heartbeat transactions
- [x] Signed transactions (one signer)
- [ ] Signed multi-sig transactions
- [ ] Logic signature transactions

### Out of Scope

- Encoding/decoding of transactions in blocks (i.e. transactions with `ApplyData`)
