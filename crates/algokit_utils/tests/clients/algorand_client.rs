use algokit_utils::{AlgorandClient, testing::algorand_fixture};

/// Test basic functionality of AlgorandClient
#[tokio::test]
async fn test_algorand_client_basic_functionality()
-> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = AlgorandClient::default_localnet();

    // Test that we can get suggested params (this verifies client connectivity)
    let suggested_params = client
        .get_suggested_params()
        .await
        .map_err(|e| format!("Failed to get suggested params: {}", e))?;

    // Basic validation that we got a valid response
    // Note: fee might be 0 on localnet with flat fees, but min_fee should always be set
    assert!(!suggested_params.genesis_id.is_empty());
    assert!(!suggested_params.genesis_hash.is_empty());
    assert!(suggested_params.last_round > 0);
    assert!(
        suggested_params.min_fee > 0,
        "Min fee should always be greater than 0"
    );

    Ok(())
}

/// Test AlgorandClient initialization methods
#[tokio::test]
async fn test_algorand_client_initialization() {
    // Test default localnet initialization - we can't access internal fields,
    // so just verify the client can be created without panicking
    let _client_localnet = AlgorandClient::default_localnet();

    // Test testnet initialization
    let _client_testnet = AlgorandClient::testnet();

    // Test mainnet initialization
    let _client_mainnet = AlgorandClient::mainnet();

    // Test from environment (should default to localnet if no env vars set)
    let _client_env = AlgorandClient::from_environment();

    // All clients were successfully created without errors
    assert!(true);
}

/// Test AlgorandClient with fixture integration
#[tokio::test]
async fn test_algorand_client_with_fixture() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let mut fixture = algorand_fixture().await?;
    fixture.new_scope().await?;

    // Use from_environment since the fixture sets up the environment properly
    let client = AlgorandClient::from_environment();

    // Test suggested params through fixture
    let suggested_params = client
        .get_suggested_params()
        .await
        .map_err(|e| format!("Failed to get suggested params: {}", e))?;

    // Basic validation
    assert!(suggested_params.last_round > 0);
    assert!(
        suggested_params.min_fee > 0,
        "Min fee should always be greater than 0"
    );

    Ok(())
}
