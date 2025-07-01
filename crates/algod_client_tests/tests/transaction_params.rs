use algod_client::AlgodClient;
use algod_client_tests::{LocalnetManager, get_algod_client};
use algokit_http_client::DefaultHttpClient;
use std::sync::Arc;

#[tokio::test]
async fn test_get_transaction_params() {
    // Ensure localnet is running
    LocalnetManager::ensure_running()
        .await
        .expect("Failed to start localnet");

    // Call the transaction params endpoint
    let result = get_algod_client().transaction_params().await;

    // Verify the call succeeded
    assert!(
        result.is_ok(),
        "Get transaction params should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();

    // Basic response validation
    assert!(
        !response.genesis_id.is_empty(),
        "Genesis ID should not be empty"
    );
    assert!(
        !response.genesis_hash.is_empty(),
        "Genesis hash should not be empty"
    );
}

#[tokio::test]
async fn test_transaction_params_error_handling() {
    let http_client = Arc::new(DefaultHttpClient::new("http://invalid-host:9999"));
    let invalid_client = AlgodClient::new(http_client);
    let result = invalid_client.transaction_params().await;

    // This should fail due to connection error
    assert!(result.is_err(), "Invalid host should result in error");
}
