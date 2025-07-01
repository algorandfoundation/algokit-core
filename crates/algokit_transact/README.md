# algokit_transact

Rust library for encoding and decoding Algorand transactions. Primary goal is to enable transaction encoding/decoding for creating transactions and attaching signatures (or program).

See [algokit_transact_ffi](../algokit_transact_ffi/) for foreign interfaces.

## Features

### Encoding and Decoding Support

- [x] Payment transactions
- [x] Asset transfer transactions
- [ ] Asset freeze transactions
- [x] Asset configuration transactions
- [x] Application call transactions
- [ ] Key registration transactions
- [ ] State proof transactions
- [ ] Heartbeat transactions
- [x] Signed transactions (one signer)
- [ ] Signed multi-sig transactions
- [ ] Logic signature transactions

### Out of Scope

- Encoding/decoding of transactions in blocks (i.e. transactions with `ApplyData`)
