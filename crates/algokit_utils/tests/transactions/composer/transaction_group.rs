use algokit_transact::MAX_TX_GROUP_SIZE;
use algokit_transact::test_utils::TransactionGroupMother;
use algokit_utils::testing::*;
use algokit_utils::{AssetCreateParams, CommonParams, PaymentParams};

use crate::common::init_test_logging;

#[tokio::test]
async fn test_payment_and_asset_create_group() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let receiver = fixture
        .generate_account(None)
        .await
        .expect("Failed to create receiver");

    let receiver_addr = receiver
        .account()
        .expect("Failed to get receiver account")
        .address();

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let payment_params = PaymentParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        receiver: receiver_addr,
        amount: 1_000_000,
    };

    let asset_create_params = AssetCreateParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        total: 1_000_000,
        decimals: Some(2),
        default_frozen: Some(false),
        asset_name: Some("Group Test Asset".to_string()),
        unit_name: Some("GTA".to_string()),
        url: Some("https://group-test.com".to_string()),
        metadata_hash: None,
        manager: Some(sender_addr.clone()),
        reserve: Some(sender_addr.clone()),
        freeze: Some(sender_addr.clone()),
        clawback: Some(sender_addr),
    };

    let mut composer = context.composer.clone();
    composer
        .add_payment(payment_params)
        .expect("Failed to add payment");

    composer
        .add_asset_create(asset_create_params)
        .expect("Failed to add asset create");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send transaction group");

    // Verify group properties
    assert_eq!(
        result.transaction_ids.len(),
        2,
        "Should have 2 transaction IDs in the group"
    );
    assert_eq!(
        result.confirmations.len(),
        2,
        "Should have 2 confirmations in the group"
    );

    let group_id = result.group_id;
    assert!(group_id.is_some(), "Group ID should be set");

    // Verify payment transaction
    let payment_confirmation = &result.confirmations[0];
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
        _ => panic!("First transaction should be a payment transaction"),
    }

    // Verify asset creation transaction
    let asset_confirmation = &result.confirmations[1];
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
        _ => panic!("Second transaction should be an asset config transaction"),
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
}

#[tokio::test]
async fn test_add_transactions_to_group_max_size() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let receiver = fixture
        .generate_account(None)
        .await
        .expect("Failed to create receiver");

    let receiver_addr = receiver
        .account()
        .expect("Failed to get receiver account")
        .address();

    let context = fixture.context().expect("Failed to get context");

    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let mut composer = context.composer.clone();

    for i in 0..MAX_TX_GROUP_SIZE - 2 {
        let payment_params = PaymentParams {
            common_params: CommonParams {
                sender: sender_addr.clone(),
                ..Default::default()
            },
            receiver: receiver_addr.clone(),
            amount: i as u64,
        };

        composer
            .add_payment(payment_params)
            .expect("Failed to add payment");
    }

    let new_transactions = TransactionGroupMother::group_of(2);

    composer
        .add_transactions(new_transactions)
        .expect("Failed to add transactions to composer");

    assert!(composer.transactions().len() == MAX_TX_GROUP_SIZE);
}

#[tokio::test]
async fn test_add_transactions_to_group_too_big() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let receiver = fixture
        .generate_account(None)
        .await
        .expect("Failed to create receiver");

    let receiver_addr = receiver
        .account()
        .expect("Failed to get receiver account")
        .address();

    let context = fixture.context().expect("Failed to get context");

    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let mut composer = context.composer.clone();

    for i in 0..MAX_TX_GROUP_SIZE - 2 {
        let payment_params = PaymentParams {
            common_params: CommonParams {
                sender: sender_addr.clone(),
                ..Default::default()
            },
            receiver: receiver_addr.clone(),
            amount: i as u64,
        };

        composer
            .add_payment(payment_params)
            .expect("Failed to add payment");
    }

    let new_transactions = TransactionGroupMother::group_of(3);

    let result = composer.add_transactions(new_transactions);
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Transaction group size exceeds the max limit of")
    );
}
