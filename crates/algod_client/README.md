# Algod REST API.

API endpoint for algod operations.

**Version:** 0.0.1
**Contact:** contact@algorand.com

This Rust crate provides a client library for the Algod REST API. API.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
algod_client = "0.0.1"
```

## Usage

```rust
use algod_client::AlgodClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client (choose one based on your network)
    let client = AlgodClient::localnet();  // For local development
    // let client = AlgodClient::testnet();  // For TestNet
    // let client = AlgodClient::mainnet();  // For MainNet

    // Example: Get network status
    let status = client.get_status().await?;
    println!("Network status: {:?}", status);

    // Example: Get transaction parameters
    let params = client.transaction_params().await?;
    println!("Min fee: {}", params.min_fee);
    println!("Last round: {}", params.last_round);

    // Example: Get account information
    let account_address = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    let account_info = client.account_information(
        None,  // format
        account_address,
        None,  // exclude
    ).await?;
    println!("Account balance: {}", account_info.amount);

    Ok(())
}
```

## Configuration

The client provides convenient constructors for different networks:

```rust
use algod_client::AlgodClient;

// For local development (uses localhost:4001 with default API token)
let client = AlgodClient::localnet();

// For Algorand TestNet
let client = AlgodClient::testnet();

// For Algorand MainNet
let client = AlgodClient::mainnet();
```

For custom configurations, you can use a custom HTTP client:

```rust
use algod_client::AlgodClient;
use algokit_http_client::DefaultHttpClient;
use std::sync::Arc;

// Custom endpoint with API token
let http_client = Arc::new(
    DefaultHttpClient::with_header(
        "http://localhost/",
        "X-API-Key",
        "your-api-key"
    )?
);
let client = AlgodClient::new(http_client);
```

## Complete Example

Here's a more comprehensive example showing how to check network status, get account information, and prepare for transactions:

```rust
use algod_client::AlgodClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to localnet
    let client = AlgodClient::localnet();

    // Check if the node is healthy and ready
    client.health_check().await?;
    client.get_ready().await?;
    println!("✓ Node is healthy and ready");

    // Get network information
    let status = client.get_status().await?;
    println!("✓ Connected to network");
    println!("  Last round: {}", status.last_round);
    println!("  Catching up: {}", status.catchup_time.unwrap_or(0));

    // Get transaction parameters needed for building transactions
    let params = client.transaction_params().await?;
    println!("✓ Retrieved transaction parameters");
    println!("  Genesis ID: {}", params.genesis_id);
    println!("  Min fee: {}", params.min_fee);

    // Example: Get account information
    let test_address = "7ZUECA7HFLZTXENRV24SHLU4AVPUTMTTDUFUBNBD64C73F3UHRTHAIOF6Q";
    match client.account_information(None, test_address, None).await {
        Ok(account) => {
            println!("✓ Account information retrieved");
            println!("  Address: {}", account.address);
            println!("  Balance: {} microAlgos", account.amount);
            println!("  Min balance: {} microAlgos", account.min_balance);
        }
        Err(e) => {
            println!("⚠ Could not retrieve account info: {}", e);
        }
    }

    Ok(())
}
```

## API Operations

This client provides access to 55 API operations:

- `health_check` - Returns OK if healthy.
- `ready` - Returns OK if healthy and fully caught up.
- `metrics` - Return metrics about algod functioning.
- `genesis` - Gets the genesis information.
- `swagger_json` - Gets the current swagger spec.
- `version` - Retrieves the supported API versions, binary build versions, and genesis information.
- `debug_settings_prof` - Retrieves the current settings for blocking and mutex profiles
- `put_debug_settings_prof` - Enables blocking and mutex profiles, and returns the old settings
- `config` - Gets the merged config file.
- `account_information` - Get account information.
- `account_asset_information` - Get account information about a given asset.
- `account_assets_information` - Get a list of assets held by an account, inclusive of asset params.
- `account_application_information` - Get account information about a given app.
- `pending_transactions_by_address` - Get a list of unconfirmed transactions currently in the transaction pool by address.
- `block` - Get the block for the given round.
- `block_tx_ids` - Get the top level transaction IDs for the block on the given round.
- `block_hash` - Get the block hash for the block on the given round.
- `transaction_proof` - Get a proof for a transaction in a block.
- `block_logs` - Get all of the logs from outer and inner app calls in the given round
- `supply` - Get the current supply reported by the ledger.
- `participation_keys` - Return a list of participation keys
- `add_participation_key` - Add a participation key to the node
- `generate_participation_keys` - Generate and install participation keys to the node.
- `participation_key_by_id` - Get participation key info given a participation ID
- `append_keys` - Append state proof keys to a participation key
- `delete_participation_key_by_id` - Delete a given participation key by ID
- `shutdown_node` - Special management endpoint to shutdown the node. Optionally provide a timeout parameter to indicate that the node should begin shutting down after a number of seconds.
- `status` - Gets the current node status.
- `status_after_block` - Gets the node status after waiting for a round after the given round.
- `raw_transaction` - Broadcasts a raw transaction or transaction group to the network.
- `raw_transaction_async` - Fast track for broadcasting a raw transaction or transaction group to the network through the tx handler without performing most of the checks and reporting detailed errors. Should be only used for development and performance testing.
- `simulate_transactions` - Simulates a raw transaction or transaction group as it would be evaluated on the network. The simulation will use blockchain state from the latest committed round.
- `transaction_params` - Get parameters for constructing a new transaction
- `pending_transactions` - Get a list of unconfirmed transactions currently in the transaction pool.
- `pending_transaction_information` - Get a specific pending transaction.
- `ledger_state_delta` - Get a LedgerStateDelta object for a given round
- `transaction_group_ledger_state_deltas_for_round` - Get LedgerStateDelta objects for all transaction groups in a given round
- `ledger_state_delta_for_transaction_group` - Get a LedgerStateDelta object for a given transaction group
- `state_proof` - Get a state proof that covers a given round
- `light_block_header_proof` - Gets a proof for a given light block header inside a state proof commitment
- `application_by_id` - Get application information.
- `application_boxes` - Get all box names for a given application.
- `application_box_by_name` - Get box information for a given application.
- `asset_by_id` - Get asset information.
- `sync_round` - Returns the minimum sync round the ledger is keeping in cache.
- `unset_sync_round` - Removes minimum sync round restriction from the ledger.
- `set_sync_round` - Given a round, tells the ledger to keep that round in its cache.
- `teal_compile` - Compile TEAL source code to binary, produce its hash
- `teal_disassemble` - Disassemble program bytes into the TEAL source code.
- `start_catchup` - Starts a catchpoint catchup.
- `abort_catchup` - Aborts a catchpoint catchup.
- `teal_dryrun` - Provide debugging information for a transaction (or group).
- `experimental_check` - Returns OK if experimental API is enabled.
- `block_time_stamp_offset` - Returns the timestamp offset. Timestamp offsets can only be set in dev mode.
- `set_block_time_stamp_offset` - Given a timestamp offset in seconds, adds the offset to every subsequent block header's timestamp.

## Models

The following data models are available:

- `GenesisAllocation` - No description
- `Genesis` - No description
- `LedgerStateDelta` - Ledger StateDelta object
- `LedgerStateDeltaForTransactionGroup` - Contains a ledger delta for a single transaction group
- `Account` - Account information at a given round.

Definition:
data/basics/userBalance.go : AccountData

- `AccountAssetHolding` - AccountAssetHolding describes the account's asset holding and asset parameters (if either exist) for a specific asset ID.
- `AccountParticipation` - AccountParticipation describes the parameters used by this account in consensus protocol.
- `Asset` - Specifies both the unique identifier and the parameters for an asset
- `AssetHolding` - Describes an asset held by an account.

Definition:
data/basics/userBalance.go : AssetHolding
- `AssetParams` - AssetParams specifies the parameters for an asset.

\[apar\] when part of an AssetConfig transaction.

Definition:
data/transactions/asset.go : AssetParams
- `AssetHoldingReference` - References an asset held by an account.
- `ApplicationLocalReference` - References an account's local state for an application.
- `ApplicationStateSchema` - Specifies maximums on the number of each type that may be stored.
- `ApplicationLocalState` - Stores local state associated with an application.
- `ParticipationKey` - Represents a participation key used by the node.
- `TealKeyValueStore` - Represents a key-value store for use in an application.
- `TealKeyValue` - Represents a key-value pair in an application store.
- `TealValue` - Represents a TEAL value.
- `AvmValue` - Represents an AVM value.
- `AvmKeyValue` - Represents an AVM key-value pair in an application store.
- `StateDelta` - Application state delta.
- `AccountStateDelta` - Application state delta.
- `EvalDeltaKeyValue` - Key-value pairs for StateDelta.
- `EvalDelta` - Represents a TEAL value delta.
- `Application` - Application index and its parameters
- `ApplicationParams` - Stores the global information associated with an application.
- `DryrunState` - Stores the TEAL eval step data
- `DryrunTxnResult` - DryrunTxnResult contains any LogicSig or ApplicationCall program debug information and state updates from a dryrun.
- `ErrorResponse` - An error response with optional data field.
- `DryrunRequest` - Request data type for dryrun endpoint. Given the Transactions and simulated ledger state upload, run TEAL scripts and return debugging information.
- `DryrunSource` - DryrunSource is TEAL source text that gets uploaded, compiled, and inserted into transactions or application state.
- `SimulateRequest` - Request type for simulation endpoint.
- `SimulateRequestTransactionGroup` - A transaction group to simulate.
- `SimulateTraceConfig` - An object that configures simulation execution trace.
- `Box` - Box name and its content.
- `BoxDescriptor` - Box descriptor describes a Box.
- `BoxReference` - References a box of an application.
- `Version` - algod version information.
- `DebugSettingsProf` - algod mutex and blocking profiling state.
- `BuildVersion` - No description
- `PendingTransactionResponse` - Details about a pending transaction. If the transaction was recently confirmed, includes confirmation details like the round and reward details.
- `SimulateTransactionGroupResult` - Simulation result for an atomic transaction group
- `SimulateTransactionResult` - Simulation result for an individual transaction
- `StateProof` - Represents a state proof and its corresponding message
- `LightBlockHeaderProof` - Proof of membership and position of a light block header.
- `StateProofMessage` - Represents the message that the state proofs are attesting to.
- `SimulationEvalOverrides` - The set of parameters and limits override during simulation. If this set of parameters is present, then evaluation parameters may differ from standard evaluation in certain ways.
- `ScratchChange` - A write operation into a scratch slot.
- `ApplicationStateOperation` - An operation against an application's global/local/box state.
- `ApplicationKvStorage` - An application's global/local/box state.
- `ApplicationInitialStates` - An application's initial global/local/box states that were accessed during simulation.
- `SimulationOpcodeTraceUnit` - The set of trace information and effect from evaluating a single opcode.
- `SimulationTransactionExecTrace` - The execution trace of calling an app or a logic sig, containing the inner app call trace in a recursive way.
- `SimulateUnnamedResourcesAccessed` - These are resources that were accessed by this group that would normally have caused failure, but were allowed in simulation. Depending on where this object is in the response, the unnamed resources it contains may or may not qualify for group resource sharing. If this is a field in SimulateTransactionGroupResult, the resources do qualify, but if this is a field in SimulateTransactionResult, they do not qualify. In order to make this group valid for actual submission, resources that qualify for group sharing can be made available by any transaction of the group; otherwise, resources must be placed in the same transaction which accessed them.
- `SimulateInitialStates` - Initial states of resources that were accessed during simulation.
- `AppCallLogs` - The logged messages from an app call along with the app ID and outer transaction ID. Logs appear in the same order that they were emitted.
- `TransactionProof` - Proof of transaction in a block.
- `GetBlockTimeStampOffsetResponse` - No description
- `GetSyncRoundResponse` - No description
- `TransactionGroupLedgerStateDeltasForRoundResponse` - No description
- `AccountAssetResponse` - No description
- `AccountAssetsInformationResponse` - No description
- `AccountApplicationResponse` - No description
- `BlockResponse` - No description
- `BlockTxidsResponse` - No description
- `BlockHashResponse` - No description
- `CatchpointStartResponse` - An catchpoint start response.
- `CatchpointAbortResponse` - An catchpoint abort response.
- `NodeStatusResponse` - NodeStatus contains the information about a node status
- `PendingTransactionsResponse` - PendingTransactions is an array of signed transactions exactly as they were submitted.
- `ParticipationKeysResponse` - No description
- `PostParticipationResponse` - No description
- `PostTransactionsResponse` - No description
- `SimulateResponse` - No description
- `BlockLogsResponse` - No description
- `SupplyResponse` - Supply represents the current supply of MicroAlgos in the system
- `TransactionParametersResponse` - TransactionParams contains the parameters that help a client construct
a new transaction.
- `BoxesResponse` - No description
- `CompileResponse` - No description
- `DisassembleResponse` - No description
- `DryrunResponse` - No description
- `SourceMap` - Source map for the program

## Error Handling

All API operations return a `Result` type. Errors include:

- Network errors (connection issues, timeouts)
- HTTP errors (4xx, 5xx status codes)
- Serialization errors (invalid JSON responses)

```rust
// Example error handling
match client.get_status().await {
    Ok(status) => {
        println!("Node is running on round: {}", status.last_round);
    }
    Err(error) => {
        eprintln!("Failed to get node status: {:?}", error);
        // Handle specific error types if needed
    }
}

// Or use the ? operator for early returns
let params = client.transaction_params().await
    .map_err(|e| format!("Failed to get transaction params: {}", e))?;
```

## Generated Code

This client was generated from an OpenAPI specification using a custom Rust code generator.

**Generated on:** Generated by Rust OpenAPI Generator
**OpenAPI Version:** 3.0.0
**Generator:** Rust OpenAPI Generator
