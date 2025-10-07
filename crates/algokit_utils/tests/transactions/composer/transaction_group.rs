use algokit_transact::test_utils::TransactionGroupMother;
use algokit_transact::{MAX_TX_GROUP_SIZE, test_utils::TransactionMother};
use algokit_utils::{AssetCreateParams, PaymentParams};
use rstest::*;

use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture};

#[rstest]
#[tokio::test]
async fn test_payment_and_asset_create_group(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();

    let receiver = algorand_fixture.generate_account(None).await?;
    let receiver_addr = receiver.account().address();

    let payment_params = PaymentParams {
        sender: sender_address.clone(),
        receiver: receiver_addr,
        amount: 1_000_000,
        ..Default::default()
    };

    let asset_create_params = AssetCreateParams {
        sender: sender_address.clone(),
        total: 1_000_000,
        decimals: Some(2),
        default_frozen: Some(false),
        asset_name: Some("Group Test Asset".to_string()),
        unit_name: Some("GTA".to_string()),
        url: Some("https://group-test.com".to_string()),
        metadata_hash: None,
        manager: Some(sender_address.clone()),
        reserve: Some(sender_address.clone()),
        freeze: Some(sender_address.clone()),
        clawback: Some(sender_address),
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_composer(None);
    composer.add_payment(payment_params)?;
    composer.add_asset_create(asset_create_params)?;

    let result = composer.send(None).await?;

    // Verify group properties
    assert_eq!(
        result.results.len(),
        2,
        "Should have 2 transaction IDs in the group"
    );
    assert_eq!(
        result.results.len(),
        2,
        "Should have 2 confirmations in the group"
    );

    assert!(result.group.is_some(), "Group ID should be set");

    // Verify payment transaction
    let payment_confirmation = &result.results[0].confirmation;
    assert!(
        payment_confirmation.confirmed_round.is_some(),
        "Payment transaction should be confirmed"
    );
    assert!(
        payment_confirmation.confirmed_round.unwrap() > 0,
        "Payment confirmed round should be greater than 0"
    );

    match &payment_confirmation.txn.transaction {
        algokit_transact::Transaction::Payment(payment_fields) => {
            assert_eq!(
                payment_fields.amount, 1_000_000,
                "Payment amount should be 1,000,000 microALGOs"
            );
        }
        _ => return Err("First transaction should be a payment transaction".into()),
    }

    // Verify asset creation transaction
    let asset_confirmation = &result.results[1].confirmation;
    assert!(
        asset_confirmation.confirmed_round.is_some(),
        "Asset creation transaction should be confirmed"
    );
    assert!(
        asset_confirmation.confirmed_round.unwrap() > 0,
        "Asset creation confirmed round should be greater than 0"
    );

    match &asset_confirmation.txn.transaction {
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
                Some("Group Test Asset".to_string()),
                "Asset name should match"
            );
            assert_eq!(
                asset_config_fields.unit_name,
                Some("GTA".to_string()),
                "Unit name should match"
            );
        }
        _ => return Err("Second transaction should be an asset config transaction".into()),
    }

    // Verify that the asset was actually created
    assert!(
        asset_confirmation.asset_id.is_some(),
        "Asset ID should be present for successful asset creation"
    );
    assert!(
        asset_confirmation.asset_id.unwrap() > 0,
        "Asset index should be greater than 0"
    );

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_add_transactions_to_group_max_size(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();

    let receiver = algorand_fixture.generate_account(None).await?;
    let receiver_addr = receiver.account().address();

    let mut composer = algorand_fixture.algorand_client.new_composer(None);

    for i in 0..MAX_TX_GROUP_SIZE - 2 {
        let payment_params = PaymentParams {
            sender: sender_address.clone(),
            receiver: receiver_addr.clone(),
            amount: i as u64,
            ..Default::default()
        };

        composer.add_payment(payment_params)?;
    }

    let new_transactions = TransactionGroupMother::group_of(2)
        .iter()
        .map(|tx| {
            let mut tx = tx.clone();
            tx.header_mut().sender = sender_address.clone();
            tx
        })
        .collect::<Vec<_>>();

    for tx in new_transactions {
        composer.add_transaction(tx, None)?;
    }

    assert!(composer.build().await.unwrap().len() == MAX_TX_GROUP_SIZE);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_add_transaction_to_group_too_big(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();

    let receiver = algorand_fixture.generate_account(None).await?;
    let receiver_addr = receiver.account().address();

    let mut composer = algorand_fixture.algorand_client.new_composer(None);

    for i in 0..MAX_TX_GROUP_SIZE {
        let payment_params = PaymentParams {
            sender: sender_address.clone(),
            receiver: receiver_addr.clone(),
            amount: i as u64,
            ..Default::default()
        };

        composer.add_payment(payment_params)?;
    }

    let new_transaction = TransactionMother::simple_payment().build()?;

    let result = composer.add_transaction(new_transaction, None);

    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Transaction group size exceeds the max limit of")
    );

    Ok(())
}
