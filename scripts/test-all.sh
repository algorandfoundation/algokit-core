#!/bin/bash

set -e

echo "=== Running all tests as done in CI ==="

echo "1-4. Running sanity checks (formatting, clippy, cargo check, basic tests)..."
./scripts/sanity.sh

echo "5. Comprehensive Rust tests with cargo t (cargo-nextest)..."
cargo t --workspace --all-targets --profile default --failure-output=immediate --status-level=all

echo "6. Building and testing TypeScript..."
cargo pkg algokit_transact typescript
cd packages/typescript/algokit_transact
bun test
cd ../../..

echo "7. Building and testing Python..."
cargo pkg algokit_transact py
cd packages/python/algokit_transact
poetry install --with test
poetry run pytest
cd ../../..

echo "=== All tests completed successfully! ==="
