#!/bin/bash

set -ex

cargo api generate-all
cargo api format-algod
cargo api format-indexer

cargo fmt --check

# Run clippy and treat warnings as errors
cargo clippy -- -D warnings

cargo check

# The generated client endpoint tests run against a live localnet; seed it first.
cargo api seed-localnet

cargo t
