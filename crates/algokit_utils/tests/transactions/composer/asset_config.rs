use crate::common::{AlgorandFixture, AlgorandFixtureResult, TestResult, algorand_fixture};
use algokit_transact::Address;
use algokit_utils::{AssetConfigParams, AssetCreateParams, AssetDestroyParams};
use rstest::*;

#[rstest]
#[tokio::test]
async fn test_asset_create_transaction(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();

    let asset_create_params = AssetCreateParams {
        sender: sender_address.clone(),
        total: 1_000_000,
        decimals: Some(2),
        default_frozen: Some(false),
        asset_name: Some("Test Asset".to_string()),
        unit_name: Some("TEST".to_string()),
        url: Some("https://example.com".to_string()),
        metadata_hash: None,
        manager: Some(sender_address.clone()),
        reserve: Some(sender_address.clone()),
        freeze: Some(sender_address.clone()),
        clawback: Some(sender_address),
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_asset_create(asset_create_params)?;

    let result = composer.send(None).await?;
    let confirmation = &result.confirmations[0];

    // Assert transaction was confirmed
    assert!(
        confirmation.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        confirmation.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );

    let transaction = &confirmation.txn.transaction;

    match transaction {
        algokit_transact::Transaction::AssetConfig(asset_config_fields) => {
            assert_eq!(
                asset_config_fields.asset_id, 0,
                "Asset ID should be 0 for creation"
            );
            assert_eq!(
                asset_config_fields.total,
                Some(1_000_000),
                "Total should be 1,000,000"
            );
            assert_eq!(
                asset_config_fields.decimals,
                Some(2),
                "Decimals should be 2"
            );
            assert_eq!(
                asset_config_fields.asset_name,
                Some("Test Asset".to_string()),
                "Asset name should match"
            );
            assert_eq!(
                asset_config_fields.unit_name,
                Some("TEST".to_string()),
                "Unit name should match"
            );
            Ok(())
        }
        _ => Err("Transaction should be an asset config transaction".into()),
    }
}

#[rstest]
#[tokio::test]
async fn test_asset_config_transaction(
    #[future] algorand_fixture: Result<AlgorandFixture, Box<dyn std::error::Error + Send + Sync>>,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();

    let new_manager_addr: Address = algorand_fixture
        .generate_account(None)
        .await?
        .account()
        .address();
    // First create an asset to reconfigure
    let asset_create_params = AssetCreateParams {
        sender: sender_address.clone(),
        total: 1_000_000,
        decimals: Some(0),
        default_frozen: Some(false),
        asset_name: Some("Reconfigure Test".to_string()),
        unit_name: Some("RECONF".to_string()),
        url: None,
        metadata_hash: None,
        manager: Some(sender_address.clone()),
        reserve: None,
        freeze: None,
        clawback: None,
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_asset_create(asset_create_params)?;

    let create_result = composer.send(None).await?;
    let asset_id = create_result.confirmations[0]
        .asset_id
        .ok_or("Failed to get asset ID")?;

    // Now reconfigure the asset
    let asset_config_params = AssetConfigParams {
        sender: sender_address.clone(),
        asset_id,
        manager: Some(new_manager_addr.clone()),
        reserve: None,
        freeze: None,
        clawback: None,
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_asset_config(asset_config_params)?;

    let result = composer.send(None).await?;

    let confirmation = &result.confirmations[0];

    // Assert transaction was confirmed
    assert!(
        confirmation.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        confirmation.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );

    let transaction = &confirmation.txn.transaction;

    match transaction {
        algokit_transact::Transaction::AssetConfig(asset_config_fields) => {
            assert_eq!(
                asset_config_fields.manager,
                Some(new_manager_addr.clone()),
                "Manager should be updated"
            );
            Ok(())
        }
        _ => Err("Transaction should be an asset config transaction".into()),
    }
}

#[rstest]
#[tokio::test]
async fn test_asset_destroy_transaction(
    #[future] algorand_fixture: Result<AlgorandFixture, Box<dyn std::error::Error + Send + Sync>>,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();
    // First create an asset to destroy
    let asset_create_params = AssetCreateParams {
        sender: sender_address.clone(),
        total: 1_000,
        decimals: Some(0),
        default_frozen: Some(false),
        asset_name: Some("Destroy Test".to_string()),
        unit_name: Some("DEST".to_string()),
        url: None,
        metadata_hash: None,
        manager: Some(sender_address.clone()),
        reserve: None,
        freeze: None,
        clawback: None,
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_asset_create(asset_create_params)?;

    let create_result = composer.send(None).await?;
    let asset_id = create_result.confirmations[0]
        .asset_id
        .ok_or("Failed to get asset ID")?;

    // Now destroy the asset
    let asset_destroy_params = AssetDestroyParams {
        sender: sender_address.clone(),
        asset_id,
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_asset_destroy(asset_destroy_params)?;

    let result = composer.send(None).await?;
    let confirmation = &result.confirmations[0];

    // Assert transaction was confirmed
    assert!(
        confirmation.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        confirmation.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );
    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_asset_create_validation_errors(
    #[future] algorand_fixture: Result<AlgorandFixture, Box<dyn std::error::Error + Send + Sync>>,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();
    // Test asset creation with multiple validation errors
    let invalid_asset_create_params = AssetCreateParams {
        sender: sender_address.clone(),
        total: 0,           // Invalid: should be > 0 (will be caught by transact validation)
        decimals: Some(25), // Invalid: should be <= 19
        default_frozen: Some(false),
        asset_name: Some("a".repeat(50)), // Invalid: should be <= 32 bytes
        unit_name: Some("VERYLONGUNITNAME".to_string()), // Invalid: should be <= 8 bytes
        url: Some(format!("https://{}", "a".repeat(100))), // Invalid: should be <= 96 bytes
        metadata_hash: None,
        manager: Some(sender_address.clone()),
        reserve: None,
        freeze: None,
        clawback: None,
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_asset_create(invalid_asset_create_params)?;

    // The validation should fail when building the transaction group
    let result = composer.build(None).await;

    // The build should return an error due to validation failures
    match result {
        Ok(_) => Err("Build with invalid asset create parameters should fail".into()),
        Err(error) => {
            let error_string = error.to_string();

            // Check that the error contains validation-related messages from the transact crate
            assert!(
                error_string.contains("validation")
                    || error_string.contains("Total")
                    || error_string.contains("Decimals")
                    || error_string.contains("Asset name")
                    || error_string.contains("Unit name")
                    || error_string.contains("URL"),
                "Error should contain validation failure details: {}",
                error_string
            );
            Ok(())
        }
    }
}
