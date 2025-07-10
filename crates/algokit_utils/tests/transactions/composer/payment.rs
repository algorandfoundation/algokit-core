use algokit_utils::testing::*;
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

    let receiver_addr = receiver.account().expect("Failed to get receiver address");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender address");

    let payment_params = PaymentParams {
        common_params: CommonParams {
            sender: sender_addr.address(),
            ..Default::default()
        },
        receiver: receiver_addr.address(),
        amount: 500_000, // 0.5 ALGO
        close_remainder_to: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_payment(payment_params)
        .expect("Failed to add payment");

    let result = composer.send().await.expect("Failed to send payment");
    let transaction = result.txn.transaction;

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
