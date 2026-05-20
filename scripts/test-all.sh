#!/bin/bash

set -e

echo "=== Running all tests as done in CI ==="

echo "1-4. Running sanity checks (formatting, clippy, cargo check, basic tests)..."
./scripts/sanity.sh

echo "5. Comprehensive Rust tests with cargo t (cargo-nextest)..."
cargo t --workspace --all-targets --profile default --failure-output=immediate --status-level=all

echo "7. Building and testing Python..."
for pkg in algokit_transact algokit_composer; do
  cargo pkg "$pkg" py
  (
    cd "packages/python/$pkg"
    poetry install --with test
    poetry run pytest
  )
done

echo "=== All tests completed successfully! ==="
