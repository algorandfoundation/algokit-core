use algod_client::AlgodClient;
use algokit_http_client::DefaultHttpClient;
use algokit_utils::ClientManager;
use std::sync::Arc;

use crate::common::logging::init_test_logging;

#[tokio::test]
async fn test_get_transaction_params() {
    init_test_logging();

    // Create algod client using ClientManager
    let config = ClientManager::get_config_from_environment_or_localnet();
    let algod_client = ClientManager::get_algod_client(&config.algod_config).unwrap();

    // Call the transaction params endpoint
    let result = algod_client.transaction_params().await;

    // Verify the call succeeded
    assert!(
        result.is_ok(),
        "Get transaction params call should succeed: {:?}",
        result.err()
    );

    let params = result.unwrap();

    // Basic validation of the response
    assert!(
        !params.genesis_id.is_empty(),
        "Genesis ID should not be empty"
    );
    assert!(params.min_fee > 0, "Min fee should be greater than 0");
}

#[tokio::test]
async fn test_transaction_params_error_handling() {
    init_test_logging();

    // Test with an invalid algod client (should fail)
    let http_client = Arc::new(DefaultHttpClient::new("http://invalid-host:4001"));
    let algod_client = AlgodClient::new(http_client);

    let result = algod_client.transaction_params().await;

    // This should fail due to invalid host
    assert!(result.is_err(), "Call to invalid algod should fail");
}
