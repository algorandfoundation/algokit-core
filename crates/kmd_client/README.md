# for KMD HTTP API

API for KMD (Key Management Daemon)

**Version:** 0.0.1
**Contact:** contact@algorand.com

This Rust crate provides a client library for the for KMD HTTP API API.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
kmd_client = "0.0.1"
```

## Usage

```rust
use kmd_client::KmdClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client (choose one based on your network)
    let client = KmdClient::localnet();  // For local development
    // let client = KmdClient::testnet();  // For TestNet
    // let client = KmdClient::mainnet();  // For MainNet

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
use kmd_client::KmdClient;

// For local development (uses localhost:4001 with default API token)
let client = KmdClient::localnet();

// For Algorand TestNet
let client = KmdClient::testnet();

// For Algorand MainNet
let client = KmdClient::mainnet();
```

For custom configurations, you can use a custom HTTP client:

```rust
use kmd_client::KmdClient;
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
let client = KmdClient::new(http_client);
```

## Complete Example

Here's a more comprehensive example showing how to check network status, get account information, and prepare for transactions:

```rust
use kmd_client::KmdClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to localnet
    let client = KmdClient::localnet();

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

This client provides access to 23 API operations:

- `swagger_handler` - Gets the current swagger spec.
- `generate_key` - Generate a key
- `delete_key` - Delete a key
- `export_key` - Export a key
- `import_key` - Import a key
- `list_keys_in_wallet` - List keys in wallet
- `export_master_key` - Export the master derivation key from a wallet
- `delete_multisig` - Delete a multisig
- `export_multisig` - Export multisig address metadata
- `import_multisig` - Import a multisig account
- `list_multisig` - List multisig accounts
- `sign_multisig_transaction` - Sign a multisig transaction
- `sign_multisig_program` - Sign a program for a multisig account
- `sign_program` - Sign program
- `sign_transaction` - Sign a transaction
- `create_wallet` - Create a wallet
- `wallet_info` - Get wallet info
- `init_wallet_handle` - Initialize a wallet handle token
- `release_wallet_handle_token` - Release a wallet handle token
- `rename_wallet` - Rename a wallet
- `renew_wallet_handle_token` - Renew a wallet handle token
- `list_wallets` - List wallets
- `version` - Retrieves the current version

## Models

The following data models are available:

- `ListWalletsResponse` - ListWalletsResponse is the response to `GET /v1/wallets`
- `ExportKeyResponse` - ExportKeyResponse is the response to `POST /v1/key/export`
- `ImportKeyResponse` - ImportKeyResponse is the response to `POST /v1/key/import`
- `ListKeysResponse` - ListKeysResponse is the response to `POST /v1/key/list`
- `GenerateKeyResponse` - GenerateKeyResponse is the response to `POST /v1/key`
- `ExportMasterKeyResponse` - ExportMasterKeyResponse is the response to `POST /v1/master-key/export`
- `ExportMultisigResponse` - ExportMultisigResponse is the response to `POST /v1/multisig/export`
- `ImportMultisigResponse` - ImportMultisigResponse is the response to `POST /v1/multisig/import`
- `ListMultisigResponse` - ListMultisigResponse is the response to `POST /v1/multisig/list`
- `SignProgramMultisigResponse` - SignProgramMultisigResponse is the response to `POST /v1/multisig/signdata`
- `SignMultisigResponse` - SignMultisigResponse is the response to `POST /v1/multisig/sign`
- `SignProgramResponse` - SignProgramResponse is the response to `POST /v1/data/sign`
- `SignTransactionResponse` - SignTransactionResponse is the response to `POST /v1/transaction/sign`
- `WalletInfoResponse` - WalletInfoResponse is the response to `POST /v1/wallet/info`
- `InitWalletHandleTokenResponse` - InitWalletHandleTokenResponse is the response to `POST /v1/wallet/init`
- `RenameWalletResponse` - RenameWalletResponse is the response to `POST /v1/wallet/rename`
- `RenewWalletHandleTokenResponse` - RenewWalletHandleTokenResponse is the response to `POST /v1/wallet/renew`
- `CreateWalletResponse` - CreateWalletResponse is the response to `POST /v1/wallet`
- `Wallet` - Wallet is the API's representation of a wallet
- `WalletHandle` - WalletHandle includes the wallet the handle corresponds to
and the number of number of seconds to expiration
- `CreateWalletRequest` - The request for `POST /v1/wallet`
- `DeleteKeyRequest` - The request for `DELETE /v1/key`
- `DeleteMultisigRequest` - The request for `DELETE /v1/multisig`
- `Digest` - No description
- `ExportKeyRequest` - The request for `POST /v1/key/export`
- `ExportMasterKeyRequest` - The request for `POST /v1/master-key/export`
- `ExportMultisigRequest` - The request for `POST /v1/multisig/export`
- `GenerateKeyRequest` - The request for `POST /v1/key`
- `ImportKeyRequest` - The request for `POST /v1/key/import`
- `ImportMultisigRequest` - The request for `POST /v1/multisig/import`
- `InitWalletHandleTokenRequest` - The request for `POST /v1/wallet/init`
- `ListKeysRequest` - The request for `POST /v1/key/list`
- `ListMultisigRequest` - The request for `POST /v1/multisig/list`
- `ListWalletsRequest` - APIV1GETWalletsRequest is the request for `GET /v1/wallets`
- `MasterDerivationKey` - MasterDerivationKey is used to derive ed25519 keys for use in wallets
- `MultisigSig` - MultisigSig is the structure that holds multiple Subsigs
- `MultisigSubsig` - MultisigSubsig is a struct that holds a pair of public key and signatures
signatures may be empty
- `PrivateKey` - No description
- `PublicKey` - No description
- `ReleaseWalletHandleTokenRequest` - The request for `POST /v1/wallet/release`
- `RenameWalletRequest` - The request for `POST /v1/wallet/rename`
- `RenewWalletHandleTokenRequest` - The request for `POST /v1/wallet/renew`
- `SignMultisigTxnRequest` - The request for `POST /v1/multisig/sign`
- `SignProgramMultisigRequest` - The request for `POST /v1/multisig/signprogram`
- `SignProgramRequest` - The request for `POST /v1/program/sign`
- `SignTxnRequest` - The request for `POST /v1/transaction/sign`
- `Signature` - No description
- `TxType` - TxType is the type of the transaction written to the ledger
- `VersionsRequest` - VersionsRequest is the request for `GET /versions`
- `VersionsResponse` - VersionsResponse is the response to `GET /versions`
friendly:VersionsResponse
- `WalletInfoRequest` - The request for `POST /v1/wallet/info`
- `Ed25519PrivateKey` - No description
- `Ed25519PublicKey` - No description
- `Ed25519Signature` - No description

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
