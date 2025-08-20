use algokit_abi::{ABIValue, Arc56Contract};
use algokit_test_artifacts::sandbox;
use algokit_transact::{Address, OnApplicationComplete};
use algokit_utils::{
    clients::{AppManager, AssetManager},
    testing::{AlgorandFixture, TestAccount, algorand_fixture},
    transactions::{
        AppCallMethodCallParams, AppCreateParams, AppMethodCallArg, AssetCreateParams,
        AssetOptInParams, AssetOptOutParams, AssetTransferParams, CommonParams, Composer,
        EmptySigner, PaymentParams, TransactionSender, TransactionSenderError,
    },
};
use base64::prelude::*;
use rstest::*;
use std::sync::Arc;

type TestResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
type SenderSetup = (TransactionSender, AlgorandFixture, Address);

#[fixture]
async fn sender_setup() -> Result<SenderSetup, Box<dyn std::error::Error + Send + Sync>> {
    let mut fixture = algorand_fixture().await?;
    fixture.new_scope().await?;
    let context = fixture.context()?;

    let algod_client = context.algod.clone();
    let sender = TransactionSender::new(
        {
            let client = algod_client.clone();
            move || Composer::new(client.clone(), Arc::new(EmptySigner {}))
        },
        AssetManager::new(algod_client.clone()),
        AppManager::new(algod_client.clone()),
    );

    let sender_address = context.test_account.account()?.address();
    Ok((sender, fixture, sender_address))
}

#[rstest]
#[tokio::test]
async fn test_payment_returns_rich_result(
    #[future] sender_setup: Result<SenderSetup, Box<dyn std::error::Error + Send + Sync>>,
) -> TestResult {
    let (sender, mut fixture, sender_address) = sender_setup.await?;
    let receiver = fixture.generate_account(None).await?;
    let test_account = fixture.context()?.test_account.clone();

    let params = PaymentParams {
        common_params: CommonParams {
            sender: sender_address,
            signer: Some(Arc::new(test_account)),
            ..Default::default()
        },
        receiver: receiver.account()?.address(),
        amount: 1_000_000,
    };

    let result = sender.payment(params, None).await?;

    // Validate rich result orchestration - Sender's unique value
    assert!(!result.tx_ids.is_empty());
    assert!(!result.confirmations.is_empty());
    assert!(result.confirmation.confirmed_round.is_some());
    assert!(!result.transactions.is_empty());
    assert_eq!(result.transactions.len(), 1);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_zero_amount_payment_allowed(
    #[future] sender_setup: Result<SenderSetup, Box<dyn std::error::Error + Send + Sync>>,
) -> TestResult {
    let (sender, mut fixture, sender_address) = sender_setup.await?;
    let receiver = fixture.generate_account(None).await?;
    let test_account = fixture.context()?.test_account.clone();

    let params = PaymentParams {
        common_params: CommonParams {
            sender: sender_address,
            signer: Some(Arc::new(test_account)),
            ..Default::default()
        },
        receiver: receiver.account()?.address(),
        amount: 0, // Zero amount should be allowed
    };

    let result = sender.payment(params, None).await?;

    // Validate that zero-amount payment succeeds
    assert!(!result.tx_ids.is_empty());
    assert!(!result.confirmations.is_empty());
    assert!(result.confirmation.confirmed_round.is_some());
    assert_eq!(result.transactions.len(), 1);

    // Verify the transaction has amount 0
    if let algokit_transact::Transaction::Payment(payment_fields) = &result.transactions[0] {
        assert_eq!(payment_fields.amount, 0);
    } else {
        panic!("Expected payment transaction");
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_asset_create_extracts_asset_id(
    #[future] sender_setup: Result<SenderSetup, Box<dyn std::error::Error + Send + Sync>>,
) -> TestResult {
    let (sender, fixture, sender_address) = sender_setup.await?;
    let test_account = fixture.context()?.test_account.clone();

    let params = AssetCreateParams {
        common_params: CommonParams {
            sender: sender_address,
            signer: Some(Arc::new(test_account)),
            ..Default::default()
        },
        total: 1000,
        decimals: Some(2),
        unit_name: Some("TEST".to_string()),
        asset_name: Some("Test Asset".to_string()),
        ..Default::default()
    };

    let result = sender.asset_create(params, None).await?;

    // Validate ID extraction from confirmation - Sender's orchestration value
    assert!(result.asset_id > 0);
    assert!(!result.common_params.tx_ids.is_empty());
    assert!(result.common_params.confirmation.confirmed_round.is_some());

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_app_create_extracts_app_id(
    #[future] sender_setup: Result<SenderSetup, Box<dyn std::error::Error + Send + Sync>>,
) -> TestResult {
    let (sender, fixture, sender_address) = sender_setup.await?;
    let test_account = fixture.context()?.test_account.clone();

    let params = AppCreateParams {
        common_params: CommonParams {
            sender: sender_address,
            signer: Some(Arc::new(test_account)),
            ..Default::default()
        },
        on_complete: OnApplicationComplete::NoOp,
        approval_program: vec![0x06, 0x81, 0x01],
        clear_state_program: vec![0x06, 0x81, 0x01],
        ..Default::default()
    };

    let result = sender.app_create(params, None).await?;

    // Validate ID extraction from confirmation - Sender's orchestration value
    assert!(result.app_id > 0);
    assert!(!result.common_params.tx_ids.is_empty());
    assert!(result.common_params.confirmation.confirmed_round.is_some());

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_abi_method_returns_enhanced_processing(
    #[future] sender_setup: Result<SenderSetup, Box<dyn std::error::Error + Send + Sync>>,
) -> TestResult {
    let (sender, mut fixture, sender_address) = sender_setup.await?;
    let test_account = fixture.context()?.test_account.clone();

    // Deploy ABI app using existing pattern
    let app_id = deploy_abi_app(&mut fixture, sender_address.clone()).await?;

    let arc56_contract: Arc56Contract = serde_json::from_str(sandbox::APPLICATION_ARC56)?;
    let method = arc56_contract
        .methods
        .iter()
        .find(|m| m.name == "hello_world")
        .expect("Failed to find hello_world method")
        .try_into()?;

    let params = AppCallMethodCallParams {
        common_params: CommonParams {
            sender: sender_address,
            signer: Some(Arc::new(test_account)),
            ..Default::default()
        },
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
    };

    let result = sender.app_call_method_call(params, None).await?;

    // Validate enhanced ABI return processing with AppManager - Sender's orchestration value
    assert!(!result.common_params.tx_ids.is_empty());
    assert!(result.common_params.confirmation.confirmed_round.is_some());
    assert!(result.abi_return.is_some());

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_asset_opt_out_uses_asset_manager_coordination(
    #[future] sender_setup: Result<SenderSetup, Box<dyn std::error::Error + Send + Sync>>,
) -> TestResult {
    let (sender, mut fixture, sender_address) = sender_setup.await?;
    let test_account = fixture.context()?.test_account.clone();

    // Create asset and opt-in account
    let asset_id = create_test_asset(&sender, sender_address.clone(), test_account.clone()).await?;
    let opt_out_account = fixture.generate_account(None).await?;
    let opt_out_address = opt_out_account.account()?.address();

    // Opt-in to asset
    opt_in_to_asset(
        &sender,
        opt_out_address.clone(),
        asset_id,
        opt_out_account.clone(),
    )
    .await?;

    let params = AssetOptOutParams {
        common_params: CommonParams {
            sender: opt_out_address,
            signer: Some(Arc::new(opt_out_account)),
            ..Default::default()
        },
        asset_id,
        close_remainder_to: None, // Let it auto-resolve to creator
    };

    let result = sender.asset_opt_out(params, None, Some(true)).await?;

    // Validate Sender orchestrated AssetManager to resolve creator automatically
    assert!(!result.tx_ids.is_empty());
    assert!(result.confirmation.confirmed_round.is_some());

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_asset_opt_out_with_balance_validation(
    #[future] sender_setup: Result<SenderSetup, Box<dyn std::error::Error + Send + Sync>>,
) -> TestResult {
    let (sender, mut fixture, sender_address) = sender_setup.await?;
    let test_account = fixture.context()?.test_account.clone();

    // Create asset and transfer some to account
    let asset_id = create_test_asset(&sender, sender_address.clone(), test_account.clone()).await?;
    let opt_out_account = fixture.generate_account(None).await?;
    let opt_out_address = opt_out_account.account()?.address();

    // Opt-in and receive assets
    opt_in_to_asset(
        &sender,
        opt_out_address.clone(),
        asset_id,
        opt_out_account.clone(),
    )
    .await?;

    let transfer_params = AssetTransferParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            signer: Some(Arc::new(test_account)),
            ..Default::default()
        },
        asset_id,
        amount: 10,
        receiver: opt_out_address.clone(),
    };
    sender.asset_transfer(transfer_params, None).await?;

    // Attempt opt-out with non-zero balance
    let params = AssetOptOutParams {
        common_params: CommonParams {
            sender: opt_out_address,
            signer: Some(Arc::new(opt_out_account)),
            ..Default::default()
        },
        asset_id,
        close_remainder_to: None, // Let it auto-resolve to creator
    };

    let result = sender.asset_opt_out(params, None, Some(true)).await;

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
    #[future] sender_setup: Result<SenderSetup, Box<dyn std::error::Error + Send + Sync>>,
) -> TestResult {
    let (sender, mut fixture, _) = sender_setup.await?;
    let opt_out_account = fixture.generate_account(None).await?;
    let opt_out_address = opt_out_account.account()?.address();

    // Try to opt out of non-existent asset - this triggers validation
    let params = AssetOptOutParams {
        common_params: CommonParams {
            sender: opt_out_address,
            signer: Some(Arc::new(opt_out_account)),
            ..Default::default()
        },
        asset_id: 999999999,      // Non-existent asset
        close_remainder_to: None, // Let it try to auto-resolve (will fail for non-existent asset)
    };

    let result = sender.asset_opt_out(params, None, Some(true)).await;

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
    #[future] sender_setup: Result<SenderSetup, Box<dyn std::error::Error + Send + Sync>>,
) -> TestResult {
    let (sender, mut fixture, sender_address) = sender_setup.await?;
    let receiver = fixture.generate_account(None).await?;
    let test_account = fixture.context()?.test_account.clone();

    let params = PaymentParams {
        common_params: CommonParams {
            sender: sender_address,
            signer: Some(Arc::new(test_account)),
            ..Default::default()
        },
        receiver: receiver.account()?.address(),
        amount: 1_000_000,
    };

    let result = sender.payment(params, None).await?;

    // Validate Sender's coordination of transaction confirmation
    assert!(result.confirmation.confirmed_round.is_some());
    assert!(result.confirmation.confirmed_round.unwrap() > 0);
    assert!(!result.tx_ids.is_empty());

    // Validate transaction parsing integration
    assert_eq!(result.transactions.len(), 1);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_new_group_creates_composer(
    #[future] sender_setup: Result<SenderSetup, Box<dyn std::error::Error + Send + Sync>>,
) -> TestResult {
    let (sender, _, _) = sender_setup.await?;

    let _composer = sender.new_group();

    // Validate Sender's Composer orchestration capability
    // Implementation details tested in composer tests
    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_utility_methods(
    #[future] sender_setup: Result<SenderSetup, Box<dyn std::error::Error + Send + Sync>>,
) -> TestResult {
    let (sender, _, _) = sender_setup.await?;

    // Test lease encoding utilities - Sender's utility orchestration
    let short_data = b"test";
    let lease1 = sender.encode_lease(short_data)?;
    assert_eq!(lease1.len(), 32);

    let long_data = vec![1u8; 100];
    let lease2 = sender.encode_lease(&long_data)?;
    assert_eq!(lease2.len(), 32);

    // Test string lease consistency
    let lease3 = sender.string_lease("test-identifier");
    let lease4 = sender.string_lease("test-identifier");
    assert_eq!(lease3, lease4);

    Ok(())
}

async fn create_test_asset(
    sender: &TransactionSender,
    sender_address: Address,
    test_account: TestAccount,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let params = AssetCreateParams {
        common_params: CommonParams {
            sender: sender_address,
            signer: Some(Arc::new(test_account)),
            ..Default::default()
        },
        total: 1000,
        decimals: Some(0),
        unit_name: Some("TEST".to_string()),
        asset_name: Some("Test Asset".to_string()),
        ..Default::default()
    };

    let result = sender.asset_create(params, None).await?;
    Ok(result.asset_id)
}

async fn opt_in_to_asset(
    sender: &TransactionSender,
    address: Address,
    asset_id: u64,
    account: TestAccount,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let params = AssetOptInParams {
        common_params: CommonParams {
            sender: address,
            signer: Some(Arc::new(account)),
            ..Default::default()
        },
        asset_id,
    };

    sender.asset_opt_in(params, None).await?;
    Ok(())
}

async fn deploy_abi_app(
    fixture: &mut AlgorandFixture,
    sender_address: Address,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let context = fixture.context()?;
    let test_account = context.test_account.clone();

    let arc56_contract: Arc56Contract = serde_json::from_str(sandbox::APPLICATION_ARC56)?;
    let teal_source = arc56_contract
        .source
        .expect("No source found in application spec");

    let approval_bytes = BASE64_STANDARD.decode(teal_source.approval)?;
    let clear_state_bytes = BASE64_STANDARD.decode(teal_source.clear)?;

    let approval_compile_result = context.algod.teal_compile(approval_bytes, None).await?;
    let clear_state_compile_result = context.algod.teal_compile(clear_state_bytes, None).await?;

    let params = AppCreateParams {
        common_params: CommonParams {
            sender: sender_address,
            signer: Some(Arc::new(test_account)),
            ..Default::default()
        },
        on_complete: OnApplicationComplete::NoOp,
        approval_program: approval_compile_result.result,
        clear_state_program: clear_state_compile_result.result,
        ..Default::default()
    };

    let mut composer = fixture.new_composer()?;
    composer.add_app_create(params)?;
    let result = composer.send(None).await?;

    Ok(result.confirmations[0].app_id.expect("No app ID returned"))
}

// Test constants removed - using simple programs instead
