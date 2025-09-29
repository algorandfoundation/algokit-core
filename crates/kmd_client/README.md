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
- `list_multisg` - List multisig accounts
- `sign_multisig_transaction` - Sign a multisig transaction
- `sign_multisig_program` - Sign a program for a multisig account
- `sign_program` - Sign program
- `sign_transaction` - Sign a transaction
- `create_wallet` - Create a wallet
- `get_wallet_info` - Get wallet info
- `init_wallet_handle_token` - Initialize a wallet handle token
- `release_wallet_handle_token` - Release a wallet handle token
- `rename_wallet` - Rename a wallet
- `renew_wallet_handle_token` - Renew a wallet handle token
- `list_wallets` - List wallets
- `get_version` - Retrieves the current version

## Models

The following data models are available:

- `DeleteKeyResponse` - APIV1DELETEKeyResponse is the response to `DELETE /v1/key`
friendly:DeleteKeyResponse
- `DeleteMultisigResponse` - APIV1DELETEMultisigResponse is the response to POST /v1/multisig/delete`
friendly:DeleteMultisigResponse
- `GetWalletsResponse` - APIV1GETWalletsResponse is the response to `GET /v1/wallets`
friendly:ListWalletsResponse
- `PostKeyExportResponse` - APIV1POSTKeyExportResponse is the response to `POST /v1/key/export`
friendly:ExportKeyResponse
- `PostKeyImportResponse` - APIV1POSTKeyImportResponse is the response to `POST /v1/key/import`
friendly:ImportKeyResponse
- `PostKeyListResponse` - APIV1POSTKeyListResponse is the response to `POST /v1/key/list`
friendly:ListKeysResponse
- `PostKeyResponse` - APIV1POSTKeyResponse is the response to `POST /v1/key`
friendly:GenerateKeyResponse
- `PostMasterKeyExportResponse` - APIV1POSTMasterKeyExportResponse is the response to `POST /v1/master-key/export`
friendly:ExportMasterKeyResponse
- `PostMultisigExportResponse` - APIV1POSTMultisigExportResponse is the response to `POST /v1/multisig/export`
friendly:ExportMultisigResponse
- `PostMultisigImportResponse` - APIV1POSTMultisigImportResponse is the response to `POST /v1/multisig/import`
friendly:ImportMultisigResponse
- `PostMultisigListResponse` - APIV1POSTMultisigListResponse is the response to `POST /v1/multisig/list`
friendly:ListMultisigResponse
- `PostMultisigProgramSignResponse` - APIV1POSTMultisigProgramSignResponse is the response to `POST /v1/multisig/signdata`
friendly:SignProgramMultisigResponse
- `PostMultisigTransactionSignResponse` - APIV1POSTMultisigTransactionSignResponse is the response to `POST /v1/multisig/sign`
friendly:SignMultisigResponse
- `PostProgramSignResponse` - APIV1POSTProgramSignResponse is the response to `POST /v1/data/sign`
friendly:SignProgramResponse
- `PostTransactionSignResponse` - APIV1POSTTransactionSignResponse is the response to `POST /v1/transaction/sign`
friendly:SignTransactionResponse
- `PostWalletInfoResponse` - APIV1POSTWalletInfoResponse is the response to `POST /v1/wallet/info`
friendly:WalletInfoResponse
- `PostWalletInitResponse` - APIV1POSTWalletInitResponse is the response to `POST /v1/wallet/init`
friendly:InitWalletHandleTokenResponse
- `PostWalletReleaseResponse` - APIV1POSTWalletReleaseResponse is the response to `POST /v1/wallet/release`
friendly:ReleaseWalletHandleTokenResponse
- `PostWalletRenameResponse` - APIV1POSTWalletRenameResponse is the response to `POST /v1/wallet/rename`
friendly:RenameWalletResponse
- `PostWalletRenewResponse` - APIV1POSTWalletRenewResponse is the response to `POST /v1/wallet/renew`
friendly:RenewWalletHandleTokenResponse
- `PostWalletResponse` - APIV1POSTWalletResponse is the response to `POST /v1/wallet`
friendly:CreateWalletResponse
- `Wallet` - APIV1Wallet is the API's representation of a wallet
- `WalletHandle` - APIV1WalletHandle includes the wallet the handle corresponds to
and the number of number of seconds to expiration
- `CreateWalletRequest` - APIV1POSTWalletRequest is the request for `POST /v1/wallet`
- `DeleteKeyRequest` - APIV1DELETEKeyRequest is the request for `DELETE /v1/key`
- `DeleteMultisigRequest` - APIV1DELETEMultisigRequest is the request for `DELETE /v1/multisig`
- `Digest` - No description
- `ExportKeyRequest` - APIV1POSTKeyExportRequest is the request for `POST /v1/key/export`
- `ExportMasterKeyRequest` - APIV1POSTMasterKeyExportRequest is the request for `POST /v1/master-key/export`
- `ExportMultisigRequest` - APIV1POSTMultisigExportRequest is the request for `POST /v1/multisig/export`
- `GenerateKeyRequest` - APIV1POSTKeyRequest is the request for `POST /v1/key`
- `ImportKeyRequest` - APIV1POSTKeyImportRequest is the request for `POST /v1/key/import`
- `ImportMultisigRequest` - APIV1POSTMultisigImportRequest is the request for `POST /v1/multisig/import`
- `InitWalletHandleTokenRequest` - APIV1POSTWalletInitRequest is the request for `POST /v1/wallet/init`
- `ListKeysRequest` - APIV1POSTKeyListRequest is the request for `POST /v1/key/list`
- `ListMultisigRequest` - APIV1POSTMultisigListRequest is the request for `POST /v1/multisig/list`
- `ListWalletsRequest` - APIV1GETWalletsRequest is the request for `GET /v1/wallets`
- `MasterDerivationKey` - MasterDerivationKey is used to derive ed25519 keys for use in wallets
- `MultisigSig` - MultisigSig is the structure that holds multiple Subsigs
- `MultisigSubsig` - MultisigSubsig is a struct that holds a pair of public key and signatures
signatures may be empty
- `PrivateKey` - No description
- `PublicKey` - No description
- `ReleaseWalletHandleTokenRequest` - APIV1POSTWalletReleaseRequest is the request for `POST /v1/wallet/release`
- `RenameWalletRequest` - APIV1POSTWalletRenameRequest is the request for `POST /v1/wallet/rename`
- `RenewWalletHandleTokenRequest` - APIV1POSTWalletRenewRequest is the request for `POST /v1/wallet/renew`
- `SignMultisigRequest` - APIV1POSTMultisigTransactionSignRequest is the request for `POST /v1/multisig/sign`
- `SignProgramMultisigRequest` - APIV1POSTMultisigProgramSignRequest is the request for `POST /v1/multisig/signprogram`
- `SignProgramRequest` - APIV1POSTProgramSignRequest is the request for `POST /v1/program/sign`
- `SignTransactionRequest` - APIV1POSTTransactionSignRequest is the request for `POST /v1/transaction/sign`
- `Signature` - No description
- `TxType` - TxType is the type of the transaction written to the ledger
- `VersionsRequest` - VersionsRequest is the request for `GET /versions`
- `VersionsResponse` - VersionsResponse is the response to `GET /versions`
friendly:VersionsResponse
- `WalletInfoRequest` - APIV1POSTWalletInfoRequest is the request for `POST /v1/wallet/info`
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
