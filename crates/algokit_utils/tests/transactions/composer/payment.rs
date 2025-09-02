use algokit_utils::{AccountCloseParams, CommonTransactionParams, PaymentParams};
use rstest::*;
use std::sync::Arc;

use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture};

#[rstest]
#[tokio::test]
async fn test_basic_payment_transaction(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();

    let receiver = algorand_fixture.generate_account(None).await?;
    let receiver_account = receiver.account();

    let payment_params = PaymentParams {
        common_params: CommonTransactionParams {
            sender: sender_address,
            ..Default::default()
        },
        receiver: receiver_account.address(),
        amount: 500_000, // 0.5 ALGO
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_payment(payment_params)?;

    let result = composer.send(None).await?;
    let transaction = &result.confirmations[0].txn.transaction;

    match transaction {
        algokit_transact::Transaction::Payment(payment_fields) => {
            assert_eq!(
                payment_fields.amount, 500_000,
                "Payment amount should be 500_000 microALGOs"
            );
        }
        _ => return Err("Transaction should be a payment transaction".into()),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_basic_account_close_transaction(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();

    let close_remainder_to = algorand_fixture.generate_account(None).await?;
    let close_remainder_to_addr = close_remainder_to.account().address();

    let account_close_params = AccountCloseParams {
        common_params: CommonTransactionParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        close_remainder_to: close_remainder_to_addr.clone(),
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_account_close(account_close_params)?;

    let result = composer.send(None).await?;
    let transaction = result.confirmations[0].txn.transaction.clone();

    match transaction {
        algokit_transact::Transaction::Payment(payment_fields) => {
            assert_eq!(
                payment_fields.receiver, sender_address,
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
        _ => return Err("Transaction should be a payment transaction".into()),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_payment_transactions_with_signers(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;
    let receiver_addr = algorand_fixture.test_account.account().address();

    // Generate a new account that will be the sender
    let sender_account = algorand_fixture.generate_account(None).await?;
    let sender_addr = sender_account.account().address();

    let mut composer = algorand_fixture.algorand_client.new_group();
    let signer = Arc::new(sender_account.clone());

    // Add two payment transactions with the same signer
    for i in 0..2 {
        let payment_params = PaymentParams {
            common_params: CommonTransactionParams {
                sender: sender_addr.clone(),
                signer: Some(signer.clone()),
                ..Default::default()
            },
            receiver: receiver_addr.clone(),
            amount: 50_000 + (i * 10_000),
        };
        composer.add_payment(payment_params)?;
    }

    let result = composer.send(None).await?;

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
        _ => return Err("Transaction should be a payment transaction".into()),
    }

    Ok(())
}
