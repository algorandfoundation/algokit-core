#!/bin/bash

set -ex

cargo api generate-all
cargo api format-algod
cargo api format-indexer

cargo fmt --check

# Run clippy and treat warnings as errors
cargo clippy -- -D warnings

cargo check

cargo test
