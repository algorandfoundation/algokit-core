use crate::common::{
    AlgorandFixture, AlgorandFixtureResult, TestAccount, TestResult, algorand_fixture,
    deploy_arc56_contract,
};
use algokit_abi::{ABIValue, Arc56Contract};
use algokit_test_artifacts::sandbox;
use algokit_transact::{Address, OnApplicationComplete};
use algokit_utils::transactions::{
    AppCallMethodCallParams, AppCreateParams, AppMethodCallArg, AssetCreateParams,
    AssetOptInParams, AssetOptOutParams, AssetTransferParams, CommonTransactionParams,
    PaymentParams, TransactionSenderError,
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
        common_params: CommonTransactionParams {
            sender: sender_address,
            ..Default::default()
        },
        receiver: receiver.account().address(),
        amount: 1_000_000,
    };

    let result = algorand_fixture
        .algorand_client
        .send()
        .payment(params, None)
        .await?;

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
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;

    let sender_address = algorand_fixture.test_account.account().address();
    let receiver = algorand_fixture.generate_account(None).await?;

    let params = PaymentParams {
        common_params: CommonTransactionParams {
            sender: sender_address,
            ..Default::default()
        },
        receiver: receiver.account().address(),
        amount: 0, // Zero amount should be allowed
    };

    let result = algorand_fixture
        .algorand_client
        .send()
        .payment(params, None)
        .await?;

    // Validate that zero-amount payment succeeds
    assert!(!result.tx_ids.is_empty());
    assert!(!result.confirmations.is_empty());
    assert!(result.confirmation.confirmed_round.is_some());
    assert_eq!(result.transactions.len(), 1);

    // Verify the transaction has amount 0
    if let algokit_transact::Transaction::Payment(payment_fields) = &result.transactions[0] {
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
        common_params: CommonTransactionParams {
            sender: sender_address,
            ..Default::default()
        },
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
    assert!(!result.common_params.tx_ids.is_empty());
    assert!(result.common_params.confirmation.confirmed_round.is_some());

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
        common_params: CommonTransactionParams {
            sender: sender_address,
            ..Default::default()
        },
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
    assert!(!result.common_params.tx_ids.is_empty());
    assert!(result.common_params.confirmation.confirmed_round.is_some());

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
    let app_id = deploy_arc56_contract(&algorand_fixture, &sender_address, &arc56_contract).await?;

    let arc56_contract: Arc56Contract = serde_json::from_str(sandbox::APPLICATION_ARC56)?;
    let method = arc56_contract
        .methods
        .iter()
        .find(|m| m.name == "hello_world")
        .expect("Failed to find hello_world method")
        .try_into()?;

    let params = AppCallMethodCallParams {
        common_params: CommonTransactionParams {
            sender: sender_address,
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

    let result = algorand_fixture
        .algorand_client
        .send()
        .app_call_method_call(params, None)
        .await?;

    // Validate enhanced ABI return processing with AppManager - Sender's orchestration value
    assert!(!result.common_params.tx_ids.is_empty());
    assert!(result.common_params.confirmation.confirmed_round.is_some());
    assert!(result.abi_return.is_some());

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
        common_params: CommonTransactionParams {
            sender: opt_out_address,
            signer: Some(Arc::new(opt_out_account)),
            ..Default::default()
        },
        asset_id,
        close_remainder_to: None, // Let it auto-resolve to creator
    };

    let result = algorand_fixture
        .algorand_client
        .send()
        .asset_opt_out(params, None, Some(true))
        .await?;

    // Validate Sender orchestrated AssetManager to resolve creator automatically
    assert!(!result.tx_ids.is_empty());
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
        common_params: CommonTransactionParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        asset_id,
        amount: 10,
        receiver: opt_out_address.clone(),
    };
    algorand_fixture
        .algorand_client
        .send()
        .asset_transfer(transfer_params, None)
        .await?;

    // Attempt opt-out with non-zero balance
    let params = AssetOptOutParams {
        common_params: CommonTransactionParams {
            sender: opt_out_address,
            signer: Some(Arc::new(opt_out_account)),
            ..Default::default()
        },
        asset_id,
        close_remainder_to: None, // Let it auto-resolve to creator
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
        common_params: CommonTransactionParams {
            sender: opt_out_address,
            signer: Some(Arc::new(opt_out_account)),
            ..Default::default()
        },
        asset_id: 999999999,      // Non-existent asset
        close_remainder_to: None, // Let it try to auto-resolve (will fail for non-existent asset)
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
        common_params: CommonTransactionParams {
            sender: sender_address,
            ..Default::default()
        },
        receiver: receiver.account().address(),
        amount: 1_000_000,
    };

    let result = algorand_fixture
        .algorand_client
        .send()
        .payment(params, None)
        .await?;

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
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let _composer = algorand_fixture.algorand_client.send().new_group();

    // Validate Sender's Composer orchestration capability
    // Implementation details tested in composer tests
    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_utility_methods(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;

    // Test lease encoding utilities - Sender's utility orchestration
    let short_data = b"test";
    let lease1 = algorand_fixture
        .algorand_client
        .send()
        .encode_lease(short_data)?;
    assert_eq!(lease1.len(), 32);

    let long_data = vec![1u8; 100];
    let lease2 = algorand_fixture
        .algorand_client
        .send()
        .encode_lease(&long_data)?;
    assert_eq!(lease2.len(), 32);

    // Test string lease consistency
    let lease3 = algorand_fixture
        .algorand_client
        .send()
        .string_lease("test-identifier");
    let lease4 = algorand_fixture
        .algorand_client
        .send()
        .string_lease("test-identifier");
    assert_eq!(lease3, lease4);

    Ok(())
}

async fn create_test_asset(
    algorand_fixture: &AlgorandFixture,
    sender_address: &Address,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let params = AssetCreateParams {
        common_params: CommonTransactionParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
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
        common_params: CommonTransactionParams {
            sender: address,
            signer: Some(Arc::new(account)),
            ..Default::default()
        },
        asset_id,
    };

    algorand_fixture
        .algorand_client
        .send()
        .asset_opt_in(params, None)
        .await?;
    Ok(())
}
