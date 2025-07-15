use algokit_utils::{AccountCloseParams, testing::*};
use algokit_utils::{CommonParams, PaymentParams};

use crate::common::init_test_logging;

#[tokio::test]
async fn test_basic_payment_transaction() {
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

    let receiver_account = receiver.account().expect("Failed to get receiver account");

    let context = fixture.context().expect("Failed to get context");
    let sender_account = context
        .test_account
        .account()
        .expect("Failed to get sender account");

    let payment_params = PaymentParams {
        common_params: CommonParams {
            sender: sender_account.address(),
            ..Default::default()
        },
        receiver: receiver_account.address(),
        amount: 500_000, // 0.5 ALGO
    };

    let mut composer = context.composer.clone();
    composer
        .add_payment(payment_params)
        .expect("Failed to add payment");

    let result = composer.send(None).await.expect("Failed to send payment");
    let transaction = &result.confirmations[0].txn.transaction;

    match transaction {
        algokit_transact::Transaction::Payment(payment_fields) => {
            assert_eq!(
                payment_fields.amount, 500_000,
                "Payment amount should be 500_000 microALGOs"
            );
        }
        _ => panic!("Transaction should be a payment transaction"),
    }
}

#[tokio::test]
async fn test_basic_account_close_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let close_remainder_to = fixture
        .generate_account(None)
        .await
        .expect("Failed to create close_remainder_to account");

    let close_remainder_to_addr = close_remainder_to
        .account()
        .expect("Failed to get close_remainder_to account")
        .address();

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let account_close_params = AccountCloseParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        close_remainder_to: close_remainder_to_addr.clone(),
    };

    let mut composer = context.composer.clone();
    composer
        .add_account_close(account_close_params)
        .expect("Failed to add account close");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send account close");
    let transaction = result.confirmations[0].txn.transaction.clone();

    match transaction {
        algokit_transact::Transaction::Payment(payment_fields) => {
            assert_eq!(
                payment_fields.receiver, sender_addr,
                "receiver should be set to the sender address"
            );
            assert_eq!(payment_fields.amount, 0, "Account close amount should be 0");
            assert!(
                payment_fields.close_remainder_to.is_some(),
                "close_remainder_to should be set for account close"
            );
            assert_eq!(
                payment_fields.close_remainder_to.unwrap(),
                close_remainder_to_addr,
                "close_remainder_to should match the provided address"
            );
        }
        _ => panic!("Transaction should be a payment transaction"),
    }
}

#[tokio::test]
async fn test_payment_transactions_with_signers() {
    use std::sync::Arc;

    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    // Generate a new account that will be the sender
    let sender_account = fixture
        .generate_account(None)
        .await
        .expect("Failed to create sender account");

    let sender_addr = sender_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let context = fixture.context().expect("Failed to get context");
    let receiver_addr = context
        .test_account
        .account()
        .expect("Failed to get receiver account")
        .address();

    let mut composer = context.composer.clone();
    let signer = Arc::new(sender_account.clone());

    // Add two payment transactions with the same signer
    for i in 0..2 {
        let payment_params = PaymentParams {
            common_params: CommonParams {
                sender: sender_addr.clone(),
                signer: Some(signer.clone()),
                ..Default::default()
            },
            receiver: receiver_addr.clone(),
            amount: 50_000 + (i * 10_000),
        };
        composer
            .add_payment(payment_params)
            .expect("Failed to add payment");
    }

    let result = composer.send(None).await.expect("Failed to send payments");

    // Verify the transaction was processed successfully
    let transaction = &result.confirmations[0].txn.transaction;
    match transaction {
        algokit_transact::Transaction::Payment(payment_fields) => {
            // This will be the first transaction in the group
            assert_eq!(
                payment_fields.header.sender, sender_addr,
                "Transaction sender should be the sender account"
            );
            assert_eq!(
                payment_fields.receiver, receiver_addr,
                "Payment receiver should match test account address"
            );
        }
        _ => panic!("Transaction should be a payment transaction"),
    }
}
