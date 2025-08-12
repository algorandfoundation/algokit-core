# Indexer

Algorand ledger analytics API.

**Version:** 2.0

This Rust crate provides a client library for the Indexer API.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
indexer_client = "2.0"
```

## Usage

```rust
use indexer_client::IndexerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client (choose one based on your network)
    let client = IndexerClient::localnet();  // For local development
    // let client = IndexerClient::testnet();  // For TestNet
    // let client = IndexerClient::mainnet();  // For MainNet

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
use indexer_client::IndexerClient;

// For local development (uses localhost:4001 with default API token)
let client = IndexerClient::localnet();

// For Algorand TestNet
let client = IndexerClient::testnet();

// For Algorand MainNet
let client = IndexerClient::mainnet();
```

For custom configurations, you can use a custom HTTP client:

```rust
use indexer_client::IndexerClient;
use algokit_http_client::DefaultHttpClient;
use std::sync::Arc;

// Custom endpoint with API token
let http_client = Arc::new(
    DefaultHttpClient::with_header(
        "https://example.com/",
        "X-API-Key",
        "your-api-key"
    )?
);
let client = IndexerClient::new(http_client);
```

## Complete Example

Here's a more comprehensive example showing how to check network status, get account information, and prepare for transactions:

```rust
use indexer_client::IndexerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to localnet
    let client = IndexerClient::localnet();

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

This client provides access to 21 API operations:

- `make_health_check` - Returns 200 if healthy.
- `search_for_accounts` - Search for accounts.
- `lookup_account_by_id` - Lookup account information.
- `lookup_account_assets` - Lookup an account's asset holdings, optionally for a specific ID.
- `lookup_account_created_assets` - Lookup an account's created asset parameters, optionally for a specific ID.
- `lookup_account_app_local_states` - Lookup an account's asset holdings, optionally for a specific ID.
- `lookup_account_created_applications` - Lookup an account's created application parameters, optionally for a specific ID.
- `lookup_account_transactions` - Lookup account transactions. Transactions are returned newest to oldest.
- `search_for_applications` - Search for applications
- `lookup_application_by_id` - Lookup application.
- `search_for_application_boxes` - Get box names for a given application.
- `lookup_application_box_by_id_and_name` - Get box information for a given application.
- `lookup_application_logs_by_id` - Lookup application logs.
- `search_for_assets` - Search for assets.
- `lookup_asset_by_id` - Lookup asset information.
- `lookup_asset_balances` - Lookup the list of accounts who hold this asset 
- `lookup_asset_transactions` - Lookup transactions for an asset. Transactions are returned oldest to newest.
- `search_for_block_headers` - Search for block headers. Block headers are returned in ascending round order. Transactions are not included in the output.
- `lookup_block` - Lookup block.
- `lookup_transaction` - Lookup a single transaction.
- `search_for_transactions` - Search for transactions. Transactions are returned oldest to newest unless the address parameter is used, in which case results are returned newest to oldest.

## Models

The following data models are available:

- `Hashtype` - The type of hash function used to create the proof, must be one of: 
* sha512_256 
* sha256
- `Account` - Account information at a given round.

Definition:
data/basics/userBalance.go : AccountData

- `AccountParticipation` - AccountParticipation describes the parameters used by this account in consensus protocol.
- `ApplicationStateSchema` - Specifies maximums on the number of each type that may be stored.
- `ApplicationLocalState` - Stores local state associated with an application.
- `TealKeyValueStore` - Represents a key-value store for use in an application.
- `TealKeyValue` - Represents a key-value pair in an application store.
- `TealValue` - Represents a TEAL value.
- `Application` - Application index and its parameters
- `ApplicationParams` - Stores the global information associated with an application.
- `ApplicationLogData` - Stores the global information associated with an application.
- `Asset` - Specifies both the unique identifier and the parameters for an asset
- `AssetHolding` - Describes an asset held by an account.

Definition:
data/basics/userBalance.go : AssetHolding
- `AssetParams` - AssetParams specifies the parameters for an asset.

\[apar\] when part of an AssetConfig transaction.

Definition:
data/transactions/asset.go : AssetParams
- `Block` - Block information.

Definition:
data/bookkeeping/block.go : Block
- `BlockRewards` - Fields relating to rewards,
- `BlockUpgradeState` - Fields relating to a protocol upgrade.
- `BlockUpgradeVote` - Fields relating to voting for a protocol upgrade.
- `Box` - Box name and its content.
- `BoxDescriptor` - Box descriptor describes an app box without a value.
- `BoxReference` - BoxReference names a box by its name and the application ID it belongs to.
- `HealthCheck` - A health check response.
- `MiniAssetHolding` - A simplified version of AssetHolding 
- `OnCompletion` - \[apan\] defines the what additional actions occur with the transaction.

Valid types:
* noop
* optin
* closeout
* clear
* update
* delete
- `ParticipationUpdates` - Participation account data that needs to be checked/acted on by the network.
- `StateDelta` - Application state delta.
- `AccountStateDelta` - Application state delta.
- `EvalDeltaKeyValue` - Key-value pairs for StateDelta.
- `EvalDelta` - Represents a TEAL value delta.
- `StateSchema` - Represents a \[apls\] local-state or \[apgs\] global-state schema. These schemas determine how much storage may be used in a local-state or global-state for an application. The more space used, the larger minimum balance must be maintained in the account holding the data.
- `Transaction` - Contains all fields common to all transactions and serves as an envelope to all transactions type. Represents both regular and inner transactions.

Definition:
data/transactions/signedtxn.go : SignedTxn
data/transactions/transaction.go : Transaction

- `TransactionApplication` - Fields for application transactions.

Definition:
data/transactions/application.go : ApplicationCallTxnFields
- `TransactionAssetConfig` - Fields for asset allocation, re-configuration, and destruction.


A zero value for asset-id indicates asset creation.
A zero value for the params indicates asset destruction.

Definition:
data/transactions/asset.go : AssetConfigTxnFields
- `TransactionAssetFreeze` - Fields for an asset freeze transaction.

Definition:
data/transactions/asset.go : AssetFreezeTxnFields
- `TransactionStateProof` - Fields for a state proof transaction. 

Definition:
data/transactions/stateproof.go : StateProofTxnFields
- `TransactionHeartbeat` - Fields for a heartbeat transaction.

Definition:
data/transactions/heartbeat.go : HeartbeatTxnFields
- `TransactionAssetTransfer` - Fields for an asset transfer transaction.

Definition:
data/transactions/asset.go : AssetTransferTxnFields
- `TransactionKeyreg` - Fields for a keyreg transaction.

Definition:
data/transactions/keyreg.go : KeyregTxnFields
- `TransactionPayment` - Fields for a payment transaction.

Definition:
data/transactions/payment.go : PaymentTxnFields
- `TransactionSignature` - Validation signature associated with some data. Only one of the signatures should be provided.
- `TransactionSignatureLogicsig` - \[lsig\] Programatic transaction signature.

Definition:
data/transactions/logicsig.go
- `TransactionSignatureMultisig` - \[msig\] structure holding multiple subsignatures.

Definition:
crypto/multisig.go : MultisigSig
- `TransactionSignatureMultisigSubsignature` - No description
- `StateProofFields` - \[sp\] represents a state proof.

Definition:
crypto/stateproof/structs.go : StateProof
- `HbProofFields` - \[hbprf\] HbProof is a signature using HeartbeatAddress's partkey, thereby showing it is online.
- `IndexerStateProofMessage` - No description
- `StateProofReveal` - No description
- `StateProofSigSlot` - No description
- `StateProofSignature` - No description
- `StateProofParticipant` - No description
- `StateProofVerifier` - No description
- `StateProofTracking` - No description
- `MerkleArrayProof` - No description
- `HashFactory` - No description
- `SearchForAccounts` - (empty)
- `LookupAccountById` - (empty)
- `LookupAccountAssets` - (empty)
- `LookupAccountCreatedAssets` - (empty)
- `LookupAccountAppLocalStates` - (empty)
- `LookupAccountCreatedApplications` - (empty)
- `LookupAccountTransactions` - (empty)
- `SearchForApplications` - (empty)
- `LookupApplicationById` - (empty)
- `SearchForApplicationBoxes` - Box names of an application
- `LookupApplicationLogsById` - (empty)
- `SearchForAssets` - (empty)
- `LookupAssetById` - (empty)
- `LookupAssetBalances` - (empty)
- `LookupAssetTransactions` - (empty)
- `SearchForBlockHeaders` - (empty)
- `LookupTransaction` - (empty)
- `SearchForTransactions` - (empty)

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
