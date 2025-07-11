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
