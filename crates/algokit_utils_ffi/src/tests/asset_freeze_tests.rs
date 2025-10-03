use crate::{
    clients::algod_client::AlgodClientTrait,
    tests::fixtures::{TestAccount, TestFixture},
    transactions::{
        asset_freeze::{AssetFreezeParams, AssetUnfreezeParams},
        asset_transfer::{AssetOptInParams, AssetTransferParams},
        common::{TransactionSignerGetter, UtilsError},
        composer::ComposerFactory,
    },
};
use std::sync::Arc;
use std::time::Instant;

/// Result of a single test
#[derive(uniffi::Record, Clone, Debug)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub duration_ms: u64,
    pub error: Option<String>,
}

/// Result of an entire test suite
#[derive(uniffi::Record, Clone, Debug)]
pub struct TestSuiteResult {
    pub name: String,
    pub results: Vec<TestResult>,
    pub all_passed: bool,
    pub total_duration_ms: u64,
}

/// Run the complete asset freeze test suite
/// This is the main entry point called from Python/Swift/Kotlin
#[uniffi::export]
pub async fn run_asset_freeze_test_suite(
    algod_client: Arc<dyn AlgodClientTrait>,
    composer_factory: Arc<dyn ComposerFactory>,
    signer_getter: Arc<dyn TransactionSignerGetter>,
) -> Result<TestSuiteResult, UtilsError> {
    let suite_start = Instant::now();
    let mut results = Vec::new();

    // Initialize test fixture with foreign traits
    let fixture = TestFixture::new(
        algod_client.clone(),
        composer_factory.clone(),
        signer_getter,
    )
    .await?;

    // Test 1: Asset Creation and Setup
    let test_start = Instant::now();
    let test1_result = run_asset_creation_setup_test(&fixture).await;
    let (creator, freeze_manager, asset_id) = match &test1_result {
        Ok(data) => data.clone(),
        Err(e) => {
            results.push(TestResult {
                name: "Asset Creation and Setup".to_string(),
                passed: false,
                duration_ms: test_start.elapsed().as_millis() as u64,
                error: Some(format!("{:?}", e)),
            });

            return Ok(TestSuiteResult {
                name: "Asset Freeze Test Suite".to_string(),
                results,
                all_passed: false,
                total_duration_ms: suite_start.elapsed().as_millis() as u64,
            });
        }
    };

    results.push(TestResult {
        name: "Asset Creation and Setup".to_string(),
        passed: true,
        duration_ms: test_start.elapsed().as_millis() as u64,
        error: None,
    });

    // Test 2: Asset Freeze and Unfreeze Test (combined)
    let test_start = Instant::now();
    let test2_result =
        run_asset_freeze_and_unfreeze_test(&fixture, creator, freeze_manager, asset_id).await;
    results.push(TestResult {
        name: "Asset Freeze and Unfreeze Test".to_string(),
        passed: test2_result.is_ok(),
        duration_ms: test_start.elapsed().as_millis() as u64,
        error: test2_result.err().map(|e| format!("{:?}", e)),
    });

    let all_passed = results.iter().all(|r| r.passed);
    let total_duration_ms = suite_start.elapsed().as_millis() as u64;

    Ok(TestSuiteResult {
        name: "Asset Freeze Test Suite".to_string(),
        results,
        all_passed,
        total_duration_ms,
    })
}

/// Test: Create asset with freeze manager
async fn run_asset_creation_setup_test(
    fixture: &TestFixture,
) -> Result<(TestAccount, TestAccount, u64), UtilsError> {
    // Generate creator and freeze manager accounts
    let creator = fixture.generate_account()?;
    let freeze_manager = fixture.generate_account()?;

    // Fund both accounts
    fixture.fund_account(creator.clone(), 10_000_000).await?;
    fixture
        .fund_account(freeze_manager.clone(), 10_000_000)
        .await?;

    // Create asset with freeze manager
    let asset_id = fixture
        .create_test_asset(creator.clone(), Some(freeze_manager.clone()))
        .await?;

    Ok((creator, freeze_manager, asset_id))
}

/// Test: Freeze an account, verify transfers are blocked, then unfreeze and verify transfers work
async fn run_asset_freeze_and_unfreeze_test(
    fixture: &TestFixture,
    creator: TestAccount,
    freeze_manager: TestAccount,
    asset_id: u64,
) -> Result<(), UtilsError> {
    // Step 1: Generate target account and fund it
    let target = fixture.generate_account()?;
    fixture.fund_account(target.clone(), 1_000_000).await?;

    // Step 2: Target opts into asset
    let target_signer = fixture.signer_getter.get_signer(target.address.clone())?;
    let opt_in_params = AssetOptInParams {
        sender: target.address.clone(),
        asset_id,
        signer: Some(target_signer.clone()),
        rekey_to: None,
        note: None,
        lease: None,
        static_fee: None,
        extra_fee: None,
        max_fee: None,
        validity_window: None,
        first_valid_round: None,
        last_valid_round: None,
    };

    let opt_in_composer = fixture.composer_factory.create_composer();
    opt_in_composer.add_asset_opt_in(opt_in_params).await?;
    opt_in_composer.build().await?;
    opt_in_composer.send().await?;

    // Step 3: Transfer assets to target (initial balance)
    let creator_signer = fixture.signer_getter.get_signer(creator.address.clone())?;
    let transfer_params = AssetTransferParams {
        sender: creator.address.clone(),
        asset_id,
        amount: 100,
        receiver: target.address.clone(),
        signer: Some(creator_signer.clone()),
        rekey_to: None,
        note: None,
        lease: None,
        static_fee: None,
        extra_fee: None,
        max_fee: None,
        validity_window: None,
        first_valid_round: None,
        last_valid_round: None,
    };

    let transfer_composer = fixture.composer_factory.create_composer();
    transfer_composer
        .add_asset_transfer(transfer_params)
        .await?;
    transfer_composer.build().await?;
    transfer_composer.send().await?;

    // Step 4: Freeze manager freezes target account
    let freeze_signer = fixture
        .signer_getter
        .get_signer(freeze_manager.address.clone())?;
    let freeze_params = AssetFreezeParams {
        sender: freeze_manager.address.clone(),
        asset_id,
        target_address: target.address.clone(),
        signer: Some(freeze_signer.clone()),
        rekey_to: None,
        note: None,
        lease: None,
        static_fee: None,
        extra_fee: None,
        max_fee: None,
        validity_window: None,
        first_valid_round: None,
        last_valid_round: None,
    };

    let freeze_composer = fixture.composer_factory.create_composer();
    freeze_composer.add_asset_freeze(freeze_params).await?;
    freeze_composer.build().await?;
    freeze_composer.send().await?;

    // Step 5: Try to transfer from frozen account (should fail)
    let transfer_from_frozen_params = AssetTransferParams {
        sender: target.address.clone(),
        asset_id,
        amount: 1,
        receiver: creator.address.clone(),
        signer: Some(target_signer.clone()),
        rekey_to: None,
        note: None,
        lease: None,
        static_fee: None,
        extra_fee: None,
        max_fee: None,
        validity_window: None,
        first_valid_round: None,
        last_valid_round: None,
    };

    let frozen_transfer_composer = fixture.composer_factory.create_composer();
    frozen_transfer_composer
        .add_asset_transfer(transfer_from_frozen_params)
        .await?;
    frozen_transfer_composer.build().await?;

    // This should fail because account is frozen
    let transfer_result = frozen_transfer_composer.send().await;

    match transfer_result {
        Ok(_) => {
            return Err(UtilsError::UtilsError {
                message: "Transfer from frozen account should have failed but succeeded"
                    .to_string(),
            });
        }
        Err(e) => {
            // Verify it failed for the RIGHT reason (asset frozen)
            let error_msg = e.to_string();
            if !error_msg.contains("frozen") {
                return Err(UtilsError::UtilsError {
                    message: format!(
                        "Transfer failed with unexpected error (expected 'frozen'): {}",
                        error_msg
                    ),
                });
            }
        }
    }

    // Step 6: Unfreeze the account
    let unfreeze_params = AssetUnfreezeParams {
        sender: freeze_manager.address.clone(),
        asset_id,
        target_address: target.address.clone(),
        signer: Some(freeze_signer),
        rekey_to: None,
        note: None,
        lease: None,
        static_fee: None,
        extra_fee: None,
        max_fee: None,
        validity_window: None,
        first_valid_round: None,
        last_valid_round: None,
    };

    let unfreeze_composer = fixture.composer_factory.create_composer();
    unfreeze_composer
        .add_asset_unfreeze(unfreeze_params)
        .await?;
    unfreeze_composer.build().await?;
    unfreeze_composer.send().await?;

    // Step 7: Transfer from unfrozen account (should now succeed)
    let transfer_after_unfreeze_params = AssetTransferParams {
        sender: target.address.clone(),
        asset_id,
        amount: 50,
        receiver: creator.address.clone(),
        signer: Some(target_signer),
        rekey_to: None,
        note: None,
        lease: None,
        static_fee: None,
        extra_fee: None,
        max_fee: None,
        validity_window: None,
        first_valid_round: None,
        last_valid_round: None,
    };

    let unfrozen_transfer_composer = fixture.composer_factory.create_composer();
    unfrozen_transfer_composer
        .add_asset_transfer(transfer_after_unfreeze_params)
        .await?;
    unfrozen_transfer_composer.build().await?;
    unfrozen_transfer_composer.send().await?;

    Ok(()) // Test passed - freeze blocked transfer, unfreeze allowed transfer
}
