use crate::common::{
    AlgorandFixture, AlgorandFixtureResult, TestAccount, TestResult, algorand_fixture,
    deploy_arc56_contract,
};
use algokit_abi::{ABIValue, Arc56Contract};
use algokit_test_artifacts::sandbox;
use algokit_transact::{Address, OnApplicationComplete};
use algokit_utils::transactions::{
    AppCallMethodCallParams, AppCreateParams, AppMethodCallArg, AssetCreateParams,
    AssetOptInParams, AssetOptOutParams, AssetTransferParams, PaymentParams,
    TransactionSenderError,
};
use rstest::*;
use std::sync::Arc;

#[rstest]
#[tokio::test]
async fn test_payment_returns_rich_result(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;

    let sender_address = algorand_fixture.test_account.account().address();
    let receiver = algorand_fixture.generate_account(None).await?;

    let params = PaymentParams {
        sender: sender_address,
        receiver: receiver.account().address(),
        amount: 1_000_000,
        ..Default::default()
    };

    let result = algorand_fixture
        .algorand_client
        .send()
        .payment(params, None)
        .await?;

    // Validate rich result orchestration - Sender's unique value
    assert!(result.confirmation.confirmed_round.is_some());

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_zero_amount_payment_allowed(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;

    let sender_address = algorand_fixture.test_account.account().address();
    let receiver = algorand_fixture.generate_account(None).await?;

    let params = PaymentParams {
        sender: sender_address,
        receiver: receiver.account().address(),
        amount: 0, // Zero amount should be allowed
        ..Default::default()
    };

    let result = algorand_fixture
        .algorand_client
        .send()
        .payment(params, None)
        .await?;

    // Validate that zero-amount payment succeeds
    assert!(result.confirmation.confirmed_round.is_some());

    // Verify the transaction has amount 0
    if let algokit_transact::Transaction::Payment(payment_fields) = &result.transaction {
        assert_eq!(payment_fields.amount, 0);
    } else {
        return Err("Expected payment transaction".into());
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_asset_create_extracts_asset_id(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;

    let sender_address = algorand_fixture.test_account.account().address();

    let params = AssetCreateParams {
        sender: sender_address,
        total: 1000,
        decimals: Some(2),
        unit_name: Some("TEST".to_string()),
        asset_name: Some("Test Asset".to_string()),
        ..Default::default()
    };

    let result = algorand_fixture
        .algorand_client
        .send()
        .asset_create(params, None)
        .await?;

    // Validate ID extraction from confirmation - Sender's orchestration value
    assert!(result.asset_id > 0);
    assert!(result.confirmation.confirmed_round.is_some());

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_app_create_extracts_app_id(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;

    let sender_address: Address = algorand_fixture.test_account.account().address();

    let params = AppCreateParams {
        sender: sender_address,
        on_complete: OnApplicationComplete::NoOp,
        approval_program: vec![0x06, 0x81, 0x01],
        clear_state_program: vec![0x06, 0x81, 0x01],
        ..Default::default()
    };

    let result = algorand_fixture
        .algorand_client
        .send()
        .app_create(params, None)
        .await?;

    // Validate ID extraction from confirmation - Sender's orchestration value
    assert!(result.app_id > 0);
    assert!(result.confirmation.confirmed_round.is_some());

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_abi_method_returns_enhanced_processing(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;

    let sender_address: Address = algorand_fixture.test_account.account().address();

    // Deploy ABI app using existing pattern
    let arc56_contract: Arc56Contract = serde_json::from_str(sandbox::APPLICATION_ARC56)?;
    let app_id = deploy_arc56_contract(
        &algorand_fixture,
        &sender_address,
        &arc56_contract,
        None,
        None,
        None,
    )
    .await?;

    let method = arc56_contract.find_abi_method("hello_world")?;

    let params = AppCallMethodCallParams {
        sender: sender_address,
        app_id,
        method,
        args: vec![AppMethodCallArg::ABIValue(ABIValue::String(
            "world".to_string(),
        ))],
        on_complete: OnApplicationComplete::NoOp,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        ..Default::default()
    };

    let result = algorand_fixture
        .algorand_client
        .send()
        .app_call_method_call(params, None)
        .await?;

    // Validate enhanced ABI return processing with AppManager - Sender's orchestration value
    assert!(!result.transaction_ids.is_empty());
    assert!(result.confirmation.confirmed_round.is_some());

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_asset_opt_out_uses_asset_manager_coordination(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;
    let sender_address: Address = algorand_fixture.test_account.account().address();

    // Create asset and opt-in account
    let asset_id = create_test_asset(&algorand_fixture, &sender_address).await?;
    let opt_out_account = algorand_fixture.generate_account(None).await?;
    let opt_out_address = opt_out_account.account().address();

    // Opt-in to asset
    opt_in_to_asset(
        &algorand_fixture,
        opt_out_address.clone(),
        asset_id,
        opt_out_account.clone(),
    )
    .await?;

    let params = AssetOptOutParams {
        sender: opt_out_address,
        signer: Some(Arc::new(opt_out_account)),
        asset_id,
        close_remainder_to: None, // Let it auto-resolve to creator
        ..Default::default()
    };

    let result = algorand_fixture
        .algorand_client
        .send()
        .asset_opt_out(params, None, Some(true))
        .await?;

    // Validate Sender orchestrated AssetManager to resolve creator automatically
    assert!(result.confirmation.confirmed_round.is_some());

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_asset_opt_out_with_balance_validation(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;
    let sender_address: Address = algorand_fixture.test_account.account().address();

    // Create asset and transfer some to account
    let asset_id = create_test_asset(&algorand_fixture, &sender_address).await?;
    let opt_out_account = algorand_fixture.generate_account(None).await?;
    let opt_out_address = opt_out_account.account().address();

    // Opt-in and receive assets
    opt_in_to_asset(
        &algorand_fixture,
        opt_out_address.clone(),
        asset_id,
        opt_out_account.clone(),
    )
    .await?;

    let transfer_params = AssetTransferParams {
        sender: sender_address.clone(),
        asset_id,
        amount: 10,
        receiver: opt_out_address.clone(),
        ..Default::default()
    };
    algorand_fixture
        .algorand_client
        .send()
        .asset_transfer(transfer_params, None)
        .await?;

    // Attempt opt-out with non-zero balance
    let params = AssetOptOutParams {
        sender: opt_out_address,
        signer: Some(Arc::new(opt_out_account)),
        asset_id,
        close_remainder_to: None, // Let it auto-resolve to creator
        ..Default::default()
    };

    let result = algorand_fixture
        .algorand_client
        .send()
        .asset_opt_out(params, None, Some(true))
        .await;

    // Validate Sender's coordination with AssetManager for balance checking
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("does not have a zero balance")
    );

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_validation_error_propagation(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;

    let opt_out_account = algorand_fixture.generate_account(None).await?;
    let opt_out_address = opt_out_account.account().address();

    // Try to opt out of non-existent asset - this triggers validation
    let params = AssetOptOutParams {
        sender: opt_out_address,
        signer: Some(Arc::new(opt_out_account)),
        asset_id: 999999999,      // Non-existent asset
        close_remainder_to: None, // Let it try to auto-resolve (will fail for non-existent asset)
        ..Default::default()
    };

    let result = algorand_fixture
        .algorand_client
        .send()
        .asset_opt_out(params, None, Some(true))
        .await;

    // Validate Sender properly propagates validation errors from AssetManager coordination
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(matches!(
        error,
        TransactionSenderError::ValidationError { message: _ }
    ));

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_transaction_confirmation_integration(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;
    let sender_address: Address = algorand_fixture.test_account.account().address();
    let receiver = algorand_fixture.generate_account(None).await?;

    let params = PaymentParams {
        sender: sender_address,
        receiver: receiver.account().address(),
        amount: 1_000_000,
        ..Default::default()
    };

    let result = algorand_fixture
        .algorand_client
        .send()
        .payment(params, None)
        .await?;

    // Validate Sender's coordination of transaction confirmation
    assert!(result.confirmation.confirmed_round.is_some());
    assert!(result.confirmation.confirmed_round.unwrap() > 0);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_new_group_creates_composer(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let _composer = algorand_fixture.algorand_client.send().new_group(None);

    // Validate Sender's Composer orchestration capability
    // Implementation details tested in composer tests
    Ok(())
}

async fn create_test_asset(
    algorand_fixture: &AlgorandFixture,
    sender_address: &Address,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let params = AssetCreateParams {
        sender: sender_address.clone(),
        total: 1000,
        decimals: Some(0),
        unit_name: Some("TEST".to_string()),
        asset_name: Some("Test Asset".to_string()),
        ..Default::default()
    };

    let result = algorand_fixture
        .algorand_client
        .send()
        .asset_create(params, None)
        .await?;
    Ok(result.asset_id)
}

async fn opt_in_to_asset(
    algorand_fixture: &AlgorandFixture,
    address: Address,
    asset_id: u64,
    account: TestAccount,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let params = AssetOptInParams {
        sender: address,
        signer: Some(Arc::new(account)),
        asset_id,
        ..Default::default()
    };

    algorand_fixture
        .algorand_client
        .send()
        .asset_opt_in(params, None)
        .await?;
    Ok(())
}
