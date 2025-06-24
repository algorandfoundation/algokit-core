#!/bin/bash

set -ex

cargo fmt --check

# Run clippy and treat warnings as errors
cargo clippy -- -D warnings

cargo check

# By default uniffi is enabled for FFI crates, so we need to explicitly check WASM
cargo check --no-default-features --features ffi_wasm

cargo test
