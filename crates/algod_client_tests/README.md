# Algod Client Tests

Integration tests for the `algod_client` crate, demonstrating usage patterns and ensuring compatibility with the Algorand network.

> TODO: Temporary crate! Tests are to be merged into the algokit_utils crate.

## Overview

This crate provides integration tests for the Algod API client, using a local Algorand network (localnet) for testing. The tests showcase:

- **Transaction parameter retrieval**
- **Raw transaction broadcasting**
- **Pending transaction information**
- **Transaction simulation**

## Test Architecture

### Consolidated Client Pattern

All tests now use a **global client fixture** for consistent, idiomatic Rust testing:

```rust
use algod_client_tests::{get_algod_client, LocalnetManager};

#[tokio::test]
async fn test_example() {
    // Ensure localnet is running
    LocalnetManager::ensure_running().await.expect("Failed to start localnet");
    
    // Use the global client - no need to create individual clients
    let result = get_algod_client().transaction_params().await;
    
    assert!(result.is_ok());
    let params = result.unwrap();
    println!("Genesis ID: {}", params.genesis_id);
}
```

### Benefits of the New Approach

✅ **DRY Principle**: No code duplication across test files
✅ **Consistent Configuration**: Single source of truth for client setup  
✅ **Idiomatic Rust**: Uses `OnceLock` for thread-safe global state
✅ **Easy Maintenance**: Configuration changes apply to all tests automatically
✅ **Consolidated API**: Uses the ergonomic `AlgodClient` instead of individual endpoint functions

### Before vs After

**Before (❌ Old pattern)**:

```rust
// Each test file had duplicate client setup
use algod_client::apis::{configuration::Configuration, transaction_params};
use std::sync::OnceLock;

static CONFIG: OnceLock<Configuration> = OnceLock::new();

fn get_config() -> &'static Configuration {
    CONFIG.get_or_init(|| ALGOD_CONFIG.clone())
}

// Individual endpoint function calls
let result = transaction_params::transaction_params(get_config()).await;
```

**After (✅ New pattern)**:

```rust
// Simple import and usage
use algod_client_tests::get_algod_client;

// Consolidated client method calls  
let result = get_algod_client().transaction_params().await;
```

## Running Tests

Tests require a algokit-cli installed with docker available. Tests will programmatically invoke localnet via algokit-cli:

```bash
# Run all integration tests
cargo test -p algod_client_tests

# Run a specific test with output
cargo test -p algod_client_tests transaction_params -- --nocapture

# Check test compilation without running
cargo check -p algod_client_tests
```

## Test Configuration

The global client uses environment variables with sensible defaults:

- `ALGORAND_HOST` (default: `http://localhost:4001`)
- `ALGORAND_API_TOKEN` (default: localnet token)

## Test Structure

```
crates/algod_client_tests/
├── src/
│   ├── lib.rs              # Module exports
│   ├── fixtures.rs         # Global client fixture & test utilities  
│   ├── localnet.rs         # Localnet management
│   ├── account_helpers.rs  # Test account creation & management
│   └── mnemonic.rs         # Mnemonic utilities
└── tests/
    ├── transaction_params.rs           # Parameter retrieval tests
    ├── raw_transaction.rs             # Transaction broadcasting tests  
    ├── pending_transaction_information.rs  # Pending transaction tests
    └── simulate_transactions.rs       # Transaction simulation tests
```

## Key Components

- **`get_algod_client()`**: Global client fixture for all tests
- **`LocalnetManager`**: Ensures test environment is available
- **`TestAccountManager`**: Creates funded test accounts
- **`LocalnetTransactionMother`**: Builds test transactions
