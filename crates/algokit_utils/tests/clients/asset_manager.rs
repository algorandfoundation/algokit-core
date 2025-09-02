use algokit_transact::Address;
use algokit_utils::{
    clients::asset_manager::AssetManagerError,
    transactions::{AssetCreateParams, AssetOptInParams, CommonTransactionParams},
};
use rstest::*;
use std::sync::Arc;

use crate::common::{AlgorandFixture, AlgorandFixtureResult, TestResult, algorand_fixture};

/// Test asset information retrieval
#[rstest]
#[tokio::test]
async fn test_get_asset_by_id(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;

    // Create test asset
    let (asset_id, _) = create_test_asset_with_creator(&mut algorand_fixture).await?;

    // Test successful retrieval
    let asset_manager = algorand_fixture.algorand_client.asset();
    let asset_info = asset_manager.get_by_id(asset_id).await?;
    assert_eq!(asset_info.asset_id, asset_id);
    assert_eq!(asset_info.total, 1000);
    assert_eq!(asset_info.decimals, 0);
    assert_eq!(asset_info.unit_name, Some("TEST".to_string()));
    assert_eq!(asset_info.asset_name, Some("Test Asset".to_string()));

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_get_asset_by_id_nonexistent(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let asset_manager = algorand_fixture.algorand_client.asset();

    // Test non-existent asset
    let result = asset_manager.get_by_id(999999999).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        AssetManagerError::AlgodClientError { source: _ }
    ));

    Ok(())
}

/// Test account asset information retrieval
#[rstest]
#[tokio::test]
async fn test_get_account_information(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;

    let (asset_id, creator_address) = create_test_asset_with_creator(&mut algorand_fixture).await?;

    // Test account information for asset creator (should be opted in by default)
    let asset_manager = algorand_fixture.algorand_client.asset();
    let account_info = asset_manager
        .get_account_information(&creator_address, asset_id)
        .await?;

    let asset_holding = account_info
        .asset_holding
        .as_ref()
        .expect("Creator should have asset holding");
    assert_eq!(asset_holding.asset_id, asset_id);
    assert_eq!(asset_holding.amount, 1000); // Creator gets all initial supply
    assert!(!asset_holding.is_frozen);
    assert!(account_info.round > 0);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_get_account_information_not_opted_in(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;

    let (asset_id, _) = create_test_asset_with_creator(&mut algorand_fixture).await?;
    let test_account = algorand_fixture.generate_account(None).await?;

    let asset_manager = algorand_fixture.algorand_client.asset();

    // Test account information for non-opted-in account should return error
    let result = asset_manager
        .get_account_information(&test_account.account().address(), asset_id)
        .await;

    // For non-opted-in accounts, algod returns 404 which becomes an AlgodClientError
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        AssetManagerError::AlgodClientError { source: _ }
    ));

    Ok(())
}

/// Helper function to create a test asset and return both asset ID and creator address
async fn create_test_asset_with_creator(
    fixture: &mut AlgorandFixture,
) -> Result<(u64, Address), Box<dyn std::error::Error + Send + Sync>> {
    let creator = fixture.generate_account(None).await?;
    let creator_address = creator.account().address();

    let params = AssetCreateParams {
        common_params: CommonTransactionParams {
            sender: creator_address.clone(),
            signer: Some(Arc::new(creator.clone())),
            ..Default::default()
        },
        total: 1000,
        decimals: Some(0),
        unit_name: Some("TEST".to_string()),
        asset_name: Some("Test Asset".to_string()),
        ..Default::default()
    };

    let result = fixture
        .algorand_client
        .send()
        .asset_create(params, None)
        .await?;

    let asset_id = result.asset_id;
    Ok((asset_id, creator_address))
}

/// Helper function to create multiple test assets
async fn create_multiple_test_assets(
    fixture: &mut AlgorandFixture,
    count: usize,
) -> Result<Vec<(u64, Address)>, Box<dyn std::error::Error + Send + Sync>> {
    let mut assets = Vec::new();
    for _ in 0..count {
        assets.push(create_test_asset_with_creator(fixture).await?);
    }
    Ok(assets)
}

/// Test bulk opt-in functionality
#[rstest]
#[tokio::test]
async fn test_bulk_opt_in_success(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;

    // Create multiple test assets
    let assets = create_multiple_test_assets(&mut algorand_fixture, 3).await?;
    let asset_ids: Vec<u64> = assets.iter().map(|(id, _)| *id).collect();

    // Create a test account that will opt into the assets
    let opt_in_account = algorand_fixture.generate_account(None).await?;
    let opt_in_address = opt_in_account.account().address();

    // Perform bulk opt-in
    let asset_manager = algorand_fixture.algorand_client.asset();
    let results = asset_manager
        .bulk_opt_in(&opt_in_address, &asset_ids)
        .await?;

    // Verify results
    assert_eq!(results.len(), 3);
    for (i, result) in results.iter().enumerate() {
        assert_eq!(result.asset_id, asset_ids[i]);
        assert!(!result.transaction_id.is_empty());
    }

    // Verify that account is now opted into all assets
    for &asset_id in &asset_ids {
        let account_info = asset_manager
            .get_account_information(&opt_in_address, asset_id)
            .await?;
        let asset_holding = account_info
            .asset_holding
            .as_ref()
            .expect("Account should be opted in");
        assert_eq!(asset_holding.asset_id, asset_id);
        assert_eq!(asset_holding.amount, 0); // Should have zero balance after opt-in
    }

    Ok(())
}

/// Test bulk opt-in with empty asset list
#[rstest]
#[tokio::test]
async fn test_bulk_opt_in_empty_list(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;

    let test_account = algorand_fixture.generate_account(None).await?;
    let test_address = test_account.account().address();

    let asset_manager = algorand_fixture.algorand_client.asset();
    let results = asset_manager.bulk_opt_in(&test_address, &[]).await?;

    assert!(results.is_empty());
    Ok(())
}

/// Test bulk opt-out functionality
#[rstest]
#[tokio::test]
async fn test_bulk_opt_out_success(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;

    // Create test assets
    let assets = create_multiple_test_assets(&mut algorand_fixture, 2).await?;
    let asset_ids: Vec<u64> = assets.iter().map(|(id, _)| *id).collect();

    // Create and opt-in an account
    let test_account = algorand_fixture.generate_account(None).await?;
    let test_address = test_account.account().address();

    let asset_manager = algorand_fixture.algorand_client.asset();

    // First, opt into the assets individually using the Composer
    let mut composer = algorand_fixture.algorand_client.new_group();

    for &asset_id in &asset_ids {
        let opt_in_params = AssetOptInParams {
            common_params: CommonTransactionParams {
                sender: test_address.clone(),
                signer: Some(Arc::new(test_account.clone())),
                ..Default::default()
            },
            asset_id,
        };
        composer.add_asset_opt_in(opt_in_params)?;
    }

    composer.send(Default::default()).await?;

    // Verify accounts are opted in
    for &asset_id in &asset_ids {
        let account_info = asset_manager
            .get_account_information(&test_address, asset_id)
            .await?;
        let asset_holding = account_info
            .asset_holding
            .as_ref()
            .expect("Account should be opted in");
        assert_eq!(asset_holding.amount, 0); // Should be zero balance
    }

    // Now perform bulk opt-out
    let results = asset_manager
        .bulk_opt_out(&test_address, &asset_ids, Some(true))
        .await?;

    // Verify results
    assert_eq!(results.len(), 2);
    for (i, result) in results.iter().enumerate() {
        assert_eq!(result.asset_id, asset_ids[i]);
        assert!(!result.transaction_id.is_empty());
    }

    // Verify that account is no longer opted into the assets
    for &asset_id in &asset_ids {
        let result = asset_manager
            .get_account_information(&test_address, asset_id)
            .await;
        // Should get an error because the account is no longer opted in
        assert!(result.is_err());
    }

    Ok(())
}

/// Test bulk opt-out with empty list
#[rstest]
#[tokio::test]
async fn test_bulk_opt_out_empty_list(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;

    let test_account = algorand_fixture.generate_account(None).await?;
    let test_address = test_account.account().address();

    let asset_manager = algorand_fixture.algorand_client.asset();
    let results = asset_manager.bulk_opt_out(&test_address, &[], None).await?;

    assert!(results.is_empty());
    Ok(())
}

/// Test bulk opt-out with non-zero balance (should fail)
#[rstest]
#[tokio::test]
async fn test_bulk_opt_out_non_zero_balance(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;

    // Create a test asset
    let (asset_id, creator_address) = create_test_asset_with_creator(&mut algorand_fixture).await?;

    let asset_manager = algorand_fixture.algorand_client.asset();

    // The creator account has the entire supply (1000), so it has non-zero balance
    let result = asset_manager
        .bulk_opt_out(&creator_address, &[asset_id], Some(true))
        .await;

    // Should fail due to non-zero balance
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        AssetManagerError::NonZeroBalance { .. }
    ));

    Ok(())
}

/// Test bulk opt-out without balance check
#[rstest]
#[tokio::test]
async fn test_bulk_opt_out_without_balance_check(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;

    // Create a test asset
    let (asset_id, _) = create_test_asset_with_creator(&mut algorand_fixture).await?;

    // Create a test account and opt it in
    let test_account = algorand_fixture.generate_account(None).await?;
    let test_address = test_account.account().address();

    let asset_manager = algorand_fixture.algorand_client.asset();

    let mut composer = algorand_fixture.algorand_client.new_group();

    let opt_in_params = AssetOptInParams {
        common_params: CommonTransactionParams {
            sender: test_address.clone(),
            signer: Some(Arc::new(test_account.clone())),
            ..Default::default()
        },
        asset_id,
    };
    composer.add_asset_opt_in(opt_in_params)?;
    composer.send(Default::default()).await?;

    // Opt out without balance check (ensure_zero_balance = false)
    let results = asset_manager
        .bulk_opt_out(&test_address, &[asset_id], Some(false))
        .await?;

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].asset_id, asset_id);
    assert!(!results[0].transaction_id.is_empty());

    Ok(())
}
