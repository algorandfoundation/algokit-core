# algod_client tests

## Layout

- `models.rs` — offline unit tests for the generated models (run in normal `cargo t`).
- `integration_localnet.rs` — a small hand-written localnet smoke test.
- `generated.rs` + `generated/*.rs` — one per-endpoint test per algod operation, generated from the
  [`algorandfoundation/algokit-polytest`](https://github.com/algorandfoundation/algokit-polytest)
  catalog via `cargo api polytest-algod`. **Do not hand-edit `generated.rs`** (it is rewritten on
  regeneration); the per-endpoint `generated/*.rs` bodies are preserved across regeneration.

## What the endpoint tests check

Each endpoint test calls the typed client method (proving the generated client **decodes** a real
node's response) and, for JSON endpoints, validates the **raw response bytes** against the endpoint's
schema in `api/specs/algod.oas3.json` (proving the spec, the generated client, and what the node
emits agree). Endpoints that only speak msgpack are decode-only — the JSON schema validator cannot
parse their bytes, so those tests assert the typed decode succeeds and skip schema validation.

The validation harness, capturing HTTP client, KMD fixtures, and seed logic live in the
`algokit_localnet_testing` crate.

## Running

The endpoint tests run against a live [algokit localnet](https://github.com/algorandfoundation/algokit-cli)
and are part of the normal `cargo t`. They need a running, **seeded** localnet:

```bash
algokit localnet start
cargo api seed-localnet     # creates shared on-chain state, writes the test manifest
cargo t -p algod_client
```

`scripts/sanity.sh` and CI run `cargo api seed-localnet` before `cargo t`, so a fresh localnet is
seeded automatically there. Without a seeded localnet the tests fail with a clear
"run `cargo api seed-localnet` first" message.

`seed-localnet` writes `tests/.localnet-manifest.json` (gitignored) recording the ids the tests read
(a funded address, an asset, two apps — one holding a box — and a confirmed txid/round). ids are
assigned per run, so tests always read them from the manifest rather than hardcoding.

## Deferred endpoints

Some endpoints cannot be exercised against devmode localnet today. Their tests keep an empty body and
an `#[ignore = "deferred: …"]` reason, so `cargo t` stays green while the reason is on record:

| Endpoint(s) | Why deferred |
| --- | --- |
| `block` | `BlockResponse` types the block as a JSON value; it cannot decode the binary the msgpack-only block body carries. |
| `teal_dryrun` | The node returns `null` for empty arrays that the generated dryrun response models type as required, non-optional `Vec`s. |
| `teal_disassemble` | The generated client sends the program as a JSON-encoded string, not raw program bytes. |
| `ledger_state_delta` (txn group / by id) | Localnet does not run the transaction tracer (501). |
| `stateproofs`, block `lightheader` proof | Localnet does not run the state-proof protocol, so no proofs exist (404). |
| `sync_round` (get / set / unset) | Only served by a node in ledger follower mode, which localnet is not (404). |

Removing a deferral means fixing the underlying generated model or running against a node that
supports the feature.