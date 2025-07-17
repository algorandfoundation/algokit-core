use algokit_transact::Transaction;
use algokit_utils::transactions::AssetOptInParams;
use algokit_utils::{CommonParams, Composer};
use algokit_utils::{
    testing::*,
    transactions::{
        AssetCreateParams, AssetFreezeParams, AssetTransferParams, AssetUnfreezeParams,
    },
};

use crate::common::init_test_logging;

#[tokio::test]
async fn test_asset_freeze_unfreeze() {
    // This integration test validates the complete asset freeze/unfreeze cycle by:
    //
    // SETUP PHASE:
    // 1. Create an asset with the asset creator account set as the freeze account (unfrozen by default)
    // 2. Target account opts into the asset
    // 3. Transfer asset units to target account
    //
    // FREEZE PHASE:
    // 4. Freeze the asset for target account using AssetFreezeParams
    // 5. Verify freeze transaction was confirmed and has correct structure
    // 6. Verify account holding shows asset is frozen via algod API
    // 7. Prove freeze works by attempting transfer (should fail)
    //
    // UNFREEZE PHASE:
    // 8. Unfreeze the asset for target account using AssetUnfreezeParams
    // 9. Verify unfreeze transaction was confirmed and has correct structure
    // 10. Verify account holding shows asset is no longer frozen via algod API
    // 11. Prove unfreeze works by successfully transferring the asset

    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");
    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    // Create a target account to hold the asset
    let target_account = fixture
        .generate_account(None)
        .await
        .expect("Failed to create target account");
    let target_addr = target_account
        .account()
        .expect("Failed to get target account")
        .address();

    let context = fixture.context().expect("Failed to get context");
    let asset_creator_account = context
        .test_account
        .account()
        .expect("Failed to get asset creator account");
    let asset_creator_addr = asset_creator_account.address();

    // Create a composer for the target account that can send transactions
    let target_composer = Composer::new(
        context.algod.clone(),
        std::sync::Arc::new(target_account.clone()),
    );

    // SETUP PHASE

    // Step 1: Create an asset with the asset creator account set as the freeze account
    let asset_create_params = AssetCreateParams {
        common_params: CommonParams {
            sender: asset_creator_addr.clone(),
            ..Default::default()
        },
        total: 1_000_000,
        decimals: Some(0),
        default_frozen: Some(false),
        asset_name: Some("Freeze Test Asset".to_string()),
        unit_name: Some("FTA".to_string()),
        url: None,
        metadata_hash: None,
        manager: Some(asset_creator_addr.clone()),
        reserve: None,
        freeze: Some(asset_creator_addr.clone()),
        clawback: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_asset_create(asset_create_params)
        .expect("Failed to add asset create");

    let create_result = composer
        .send(None)
        .await
        .expect("Failed to send asset create");
    let asset_id = create_result.confirmations[0]
        .asset_index
        .expect("Failed to get asset ID");

    // Step 2: Target account opts into the asset
    let asset_opt_in_params = AssetOptInParams {
        common_params: CommonParams {
            sender: target_addr.clone(),
            ..Default::default()
        },
        asset_id,
    };

    let mut composer = target_composer.clone();
    composer
        .add_asset_opt_in(asset_opt_in_params)
        .expect("Failed to add asset opt-in");

    let opt_in_result = composer
        .send(None)
        .await
        .expect("Failed to send asset opt-in");

    assert!(
        opt_in_result.confirmations[0].confirmed_round.is_some(),
        "Asset opt-in should be confirmed"
    );

    println!("Target account opted into asset");

    // Step 3: Send some asset units to the target account
    let asset_transfer_params = AssetTransferParams {
        common_params: CommonParams {
            sender: asset_creator_addr.clone(),
            ..Default::default()
        },
        asset_id,
        amount: 1000,
        receiver: target_addr.clone(),
    };

    let mut composer = context.composer.clone();
    composer
        .add_asset_transfer(asset_transfer_params)
        .expect("Failed to add asset transfer");

    let transfer_result = composer
        .send(None)
        .await
        .expect("Failed to send asset transfer");

    assert!(
        transfer_result.confirmations[0].confirmed_round.is_some(),
        "Asset transfer should be confirmed"
    );

    println!("Transferred 1000 asset units to target account");

    // FREEZE PHASE

    // Step 4: Freeze the asset for the target account
    let asset_freeze_params = AssetFreezeParams {
        common_params: CommonParams {
            sender: asset_creator_addr.clone(),
            ..Default::default()
        },
        asset_id,
        target_address: target_addr.clone(),
    };

    let mut composer = context.composer.clone();
    composer
        .add_asset_freeze(asset_freeze_params)
        .expect("Failed to add asset freeze");

    let freeze_result = composer
        .send(None)
        .await
        .expect("Failed to send asset freeze");

    // Step 5: Verify freeze transaction was confirmed and has correct structure
    let freeze_confirmation = &freeze_result.confirmations[0];
    assert!(
        freeze_confirmation.confirmed_round.is_some(),
        "Asset freeze transaction should be confirmed"
    );
    assert!(
        freeze_confirmation.confirmed_round.unwrap() > 0,
        "Freeze confirmed round should be greater than 0"
    );

    match &freeze_confirmation.txn.transaction {
        Transaction::AssetFreeze(txn) => {
            assert_eq!(txn.asset_id, asset_id, "Asset ID should match");
            assert_eq!(txn.freeze_target, target_addr, "Freeze target should match");
            assert!(txn.frozen, "Asset should be frozen");
        }
        _ => panic!("Transaction should be an AssetFreeze transaction"),
    }

    println!("Asset frozen for target account");

    // Step 6: Verify account holding shows asset is frozen via algod API
    let account_info = context
        .algod
        .account_information(None, &target_addr.to_string(), None)
        .await
        .expect("Failed to get account information");

    let assets = account_info.assets.expect("Account should have assets");

    let asset_holding = assets
        .iter()
        .find(|asset| asset.asset_id == asset_id)
        .expect("Target account should have the asset");

    assert!(
        asset_holding.is_frozen,
        "Asset should be frozen in account holding"
    );

    println!("Verified asset is frozen in account holding");

    // Step 7: Prove freeze works by attempting transfer (should fail)
    let attempt_transfer_params = AssetTransferParams {
        common_params: CommonParams {
            sender: target_addr.clone(),
            ..Default::default()
        },
        asset_id,
        amount: 100,
        receiver: asset_creator_addr.clone(),
    };

    let mut composer = target_composer.clone();
    composer
        .add_asset_transfer(attempt_transfer_params)
        .expect("Failed to add asset transfer attempt");

    let transfer_attempt_result = composer.send(None).await;

    assert!(
        transfer_attempt_result.is_err(),
        "Transfer of frozen asset should fail"
    );
    // Verify the error is related to the asset being frozen
    let error_message = transfer_attempt_result.unwrap_err().to_string();
    assert!(
        error_message.contains(&format!("asset {} frozen", asset_id)),
        "Error should indicate the asset is frozen: {}",
        error_message
    );

    println!("Confirmed that frozen asset cannot be transferred");

    // UNFREEZE PHASE

    // Step 8: Unfreeze the asset for the target account
    let asset_unfreeze_params = AssetUnfreezeParams {
        common_params: CommonParams {
            sender: asset_creator_addr.clone(),
            ..Default::default()
        },
        asset_id,
        target_address: target_addr.clone(),
    };

    let mut composer = context.composer.clone();
    composer
        .add_asset_unfreeze(asset_unfreeze_params)
        .expect("Failed to add asset unfreeze");

    let unfreeze_result = composer
        .send(None)
        .await
        .expect("Failed to send asset unfreeze");

    // Step 9: Verify unfreeze transaction was confirmed and has correct structure
    let unfreeze_confirmation = &unfreeze_result.confirmations[0];
    assert!(
        unfreeze_confirmation.confirmed_round.is_some(),
        "Asset unfreeze transaction should be confirmed"
    );
    assert!(
        unfreeze_confirmation.confirmed_round.unwrap() > 0,
        "Unfreeze confirmed round should be greater than 0"
    );

    match &unfreeze_confirmation.txn.transaction {
        Transaction::AssetFreeze(txn) => {
            assert_eq!(txn.asset_id, asset_id, "Asset ID should match");
            assert_eq!(txn.freeze_target, target_addr, "Freeze target should match");
            assert!(!txn.frozen, "Asset should be unfrozen");
        }
        _ => panic!("Transaction should be an AssetFreeze transaction"),
    }

    println!("Asset unfrozen for target account");

    // Step 10: Verify account holding shows asset is no longer frozen via algod API
    let account_info_after = context
        .algod
        .account_information(None, &target_addr.to_string(), None)
        .await
        .expect("Failed to get account information after unfreeze");

    let assets_after = account_info_after
        .assets
        .expect("Account should still have assets");

    let asset_holding_after = assets_after
        .iter()
        .find(|asset| asset.asset_id == asset_id)
        .expect("Target account should still have the asset");

    assert!(
        !asset_holding_after.is_frozen,
        "Asset should no longer be frozen in account holding"
    );

    println!("Verified asset is no longer frozen in account holding");

    // Step 11: Prove unfreeze works by successfully transferring the asset
    let test_transfer_params = AssetTransferParams {
        common_params: CommonParams {
            sender: target_addr.clone(),
            ..Default::default()
        },
        asset_id,
        amount: 100,
        receiver: asset_creator_addr.clone(),
    };

    let mut composer = target_composer.clone();
    composer
        .add_asset_transfer(test_transfer_params)
        .expect("Failed to add test asset transfer");

    let test_transfer_result = composer
        .send(None)
        .await
        .expect("Failed to send test asset transfer - asset should be unfrozen");

    assert!(
        test_transfer_result.confirmations[0]
            .confirmed_round
            .is_some(),
        "Test asset transfer should be confirmed, proving asset is unfrozen"
    );

    println!("Successfully transferred asset, confirming it's unfrozen");
}
