# algokit_transact_ffi

Provides interfaces for [UniFFI](https://github.com/mozilla/uniffi-rs) and [wasm-pack](https://github.com/rustwasm/wasm-pack) for the [algokit_transact](../algokit_transact/) crate.

## Key Files

- [src/lib.rs](src/lib.rs): Wrappers around the Rust library for the WASM and Python bindings
- [tests/js/pkg/algokit_transact.js](tests/js/pkg/algokit_transact.js): auto-generated WASM bindings
- [tests/js/index.ts](tests/js/index.ts): example of using the WASM bindings
- [tests/py/algokit_transact.py](tests/py/algokit_transact.py): auto-generated Python bindings
- [tests/py/app.py](tests/py/app.py): example of using the Python bindings
- [build.py](build.py): script to build the WASM and Python bindings

## Example

### TypeScript

Example of using the library in TypeScript:

```ts
const tx = {
  header: {
    sender: tx.snd,
    fee: tx.fee,
    transactionType: "Payment",
    firstValid: tx.fv,
    lastValid: tx.lv,
    genesisHash: tx.gh,
    genesisId: tx.gen,
  },
  receiver: tx.rcv,
  amount: tx.amt,
} as Transaction;

const encodedTx = encodeTransaction(tx);

// Signing with a ed25519 lib that has no idea about Algorand
const txSig = await ed.signAsync(encodedTx, privKey);
const signedTx: SignedTransaction = {
  transaction: tx,
  signature: txSig,
};
const encodedSignedTx = encodeSignedTransaction(signedTx);
```

See [tests/js/index.ts](tests/js/index.ts) for the full example.
