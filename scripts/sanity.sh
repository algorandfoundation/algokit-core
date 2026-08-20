#!/bin/bash

set -ex

cargo api generate-all
cargo api format-algod
cargo api format-indexer

# algod_client/ffi_uniffi must never be enabled. It does not compile, and because Cargo
# unifies features across the workspace, enabling it anywhere flips the SignedTransaction
# type inside the simulate and pending-transaction models for every consumer — breaking
# algokit_composer in CI while `cargo build -p algokit_composer` still passes.
if cargo tree --workspace -e features -i algod_client 2>/dev/null \
     | grep -q 'algod_client feature "ffi_uniffi"'; then
  echo "ERROR: algod_client/ffi_uniffi is enabled somewhere. Remove the feature forwarding." >&2
  exit 1
fi

# Core crates must not depend on UniFFI, and must stay clear of the wasm-hostile
# getrandom 0.2 that uuid used to drag in through the generated clients.
if cargo tree -p algokit_composer -e normal --prefix none 2>/dev/null \
     | grep -Eq '^(uniffi|getrandom v0\.2)'; then
  echo "ERROR: forbidden dependency in algokit_composer's normal tree." >&2
  exit 1
fi

# The composer must keep compiling to wasm32. Lib target only — dev-dependencies pull
# reqwest for the localnet test and are not expected to build for wasm.
if rustup target list --installed | grep -qx wasm32-unknown-unknown; then
  cargo check -p algokit_composer --target wasm32-unknown-unknown
fi

cargo fmt --check

# Run clippy and treat warnings as errors
cargo clippy -- -D warnings

cargo check

cargo t
