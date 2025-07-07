use algokit_utils::testing::*;
use algokit_utils::{AssetCreateParams, AssetDestroyParams, AssetReconfigureParams, CommonParams};

use crate::common::init_test_logging;

#[tokio::test]
async fn test_asset_create_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .address()
        .expect("Failed to get sender address");

    let asset_create_params = AssetCreateParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        total: 1_000_000,
        decimals: Some(2),
        default_frozen: Some(false),
        asset_name: Some("Test Asset".to_string()),
        unit_name: Some("TEST".to_string()),
        url: Some("https://example.com".to_string()),
        metadata_hash: None,
        manager: Some(sender_addr.clone()),
        reserve: Some(sender_addr.clone()),
        freeze: Some(sender_addr.clone()),
        clawback: Some(sender_addr),
    };

    let mut composer = context.composer.clone();
    composer
        .add_asset_create(asset_create_params)
        .expect("Failed to add asset create");

    let result = composer.send().await.expect("Failed to send asset create");

    // Assert transaction was confirmed
    assert!(
        result.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        result.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );

    let transaction = result.txn.transaction;

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
        }
        _ => panic!("Transaction should be an asset config transaction"),
    }
}

#[tokio::test]
async fn test_asset_reconfigure_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let new_manager = fixture
        .generate_account(None)
        .await
        .expect("Failed to create new manager");

    let new_manager_addr = new_manager
        .address()
        .expect("Failed to get new manager address");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .address()
        .expect("Failed to get sender address");

    // First create an asset to reconfigure
    let asset_create_params = AssetCreateParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        total: 1_000_000,
        decimals: Some(0),
        default_frozen: Some(false),
        asset_name: Some("Reconfigure Test".to_string()),
        unit_name: Some("RECONF".to_string()),
        url: None,
        metadata_hash: None,
        manager: Some(sender_addr.clone()),
        reserve: None,
        freeze: None,
        clawback: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_asset_create(asset_create_params)
        .expect("Failed to add asset create");

    let create_result = composer.send().await.expect("Failed to send asset create");
    let asset_id = create_result.asset_index.expect("Failed to get asset ID");

    // Now reconfigure the asset
    let asset_reconfigure_params = AssetReconfigureParams {
        common_params: CommonParams {
            sender: sender_addr,
            ..Default::default()
        },
        asset_id,
        manager: Some(new_manager_addr.clone()),
        reserve: None,
        freeze: None,
        clawback: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_asset_reconfigure(asset_reconfigure_params)
        .expect("Failed to add asset reconfigure");

    let result = composer
        .send()
        .await
        .expect("Failed to send asset reconfigure");

    // Assert transaction was confirmed
    assert!(
        result.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        result.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );

    let transaction = result.txn.transaction;

    match transaction {
        algokit_transact::Transaction::AssetConfig(asset_config_fields) => {
            assert_eq!(
                asset_config_fields.manager,
                Some(new_manager_addr.clone()),
                "Manager should be updated"
            );
        }
        _ => panic!("Transaction should be an asset config transaction"),
    }
}

#[tokio::test]
async fn test_asset_destroy_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .address()
        .expect("Failed to get sender address");

    // First create an asset to destroy
    let asset_create_params = AssetCreateParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        total: 1_000,
        decimals: Some(0),
        default_frozen: Some(false),
        asset_name: Some("Destroy Test".to_string()),
        unit_name: Some("DEST".to_string()),
        url: None,
        metadata_hash: None,
        manager: Some(sender_addr.clone()),
        reserve: None,
        freeze: None,
        clawback: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_asset_create(asset_create_params)
        .expect("Failed to add asset create");

    let create_result = composer.send().await.expect("Failed to send asset create");
    let asset_id = create_result.asset_index.expect("Failed to get asset ID");

    // Now destroy the asset
    let asset_destroy_params = AssetDestroyParams {
        common_params: CommonParams {
            sender: sender_addr,
            ..Default::default()
        },
        asset_id,
    };

    let mut composer = context.composer.clone();
    composer
        .add_asset_destroy(asset_destroy_params)
        .expect("Failed to add asset destroy");

    let result = composer.send().await.expect("Failed to send asset destroy");

    // Assert transaction was confirmed
    assert!(
        result.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        result.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );
}
