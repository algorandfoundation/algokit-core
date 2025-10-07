use crate::common::app_fixture::{sandbox_app_fixture, testing_app_fixture, testing_app_spec};
use crate::common::{TestResult, nested_contract_fixture};
use algokit_abi::{ABIMethod, ABIValue};
use algokit_transact::{BoxReference, SignedTransaction, Transaction};
use algokit_utils::applications::app_client::AppClientMethodCallParams;
use algokit_utils::transactions::{PaymentParams, TransactionSigner, TransactionWithSigner};
use algokit_utils::{AppCallMethodCallParams, AppManager, AppMethodCallArg};
use async_trait::async_trait;
use num_bigint::BigUint;
use rand::Rng;
use rstest::*;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

#[rstest]
#[tokio::test]
async fn test_create_then_call_app(
    #[future] testing_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let f = testing_app_fixture.await?;
    let client = f.client;
    let sender = f.sender_address;

    let result = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "call_abi".to_string(),
                args: vec![AppMethodCallArg::ABIValue(ABIValue::from("test"))],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    let abi_return = result.result.abi_return.expect("Expected ABI return");
    match abi_return.return_value {
        Some(ABIValue::String(s)) => assert_eq!(s, "Hello, test"),
        _ => return Err("Expected string ABI return".into()),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_construct_transaction_with_abi_encoding_including_transaction(
    #[future] testing_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let mut f = testing_app_fixture.await?;
    let funded_account = f.algorand_fixture.generate_account(None).await?;
    let funded_addr = funded_account.account().address();

    let mut rng = rand::thread_rng();
    let amount: u64 = rng.gen_range(1..=10000);

    let payment_txn = f
        .algorand_fixture
        .algorand_client
        .create()
        .payment(PaymentParams {
            sender: funded_addr.clone(),
            receiver: funded_addr.clone(),
            amount,
            ..Default::default()
        })
        .await?;
    let client = f.client;

    let result = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "call_abi_txn".to_string(),
                args: vec![
                    AppMethodCallArg::Transaction(payment_txn),
                    AppMethodCallArg::ABIValue(ABIValue::from("test")),
                ],
                sender: Some(funded_addr.to_string()),
                signer: Some(Arc::new(funded_account.clone())),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    assert_eq!(result.group_results.len(), 2);

    let abi_return = result
        .result
        .abi_return
        .as_ref()
        .expect("Expected ABI return");
    let expected_return = format!("Sent {}. {}", amount, "test");
    match &abi_return.return_value {
        Some(ABIValue::String(s)) => assert_eq!(s, &expected_return),
        _ => return Err("Expected string ABI return".into()),
    }

    let method = testing_app_spec()
        .find_abi_method("call_abi_txn")
        .expect("ABI method");
    let decoded = AppManager::get_abi_return(&abi_return.raw_return_value, &method)
        .expect("Decoded ABI return");
    match decoded.return_value {
        Some(ABIValue::String(s)) => assert_eq!(s, expected_return),
        _ => return Err("Expected string ABI return from AppManager decoding".into()),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_call_app_with_too_many_args(
    #[future] testing_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let f = testing_app_fixture.await?;
    let sender = f.sender_address;
    let client = f.client;

    let err = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "call_abi".to_string(),
                args: vec![
                    AppMethodCallArg::ABIValue(ABIValue::from("one")),
                    AppMethodCallArg::ABIValue(ABIValue::from("two")),
                ],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await
        .expect_err("Expected validation error due to too many args");

    assert!(
        err.to_string()
            .contains("The number of provided arguments is"),
        "Unexpected error message: {}",
        err
    );

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_call_app_with_rekey(
    #[future] testing_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let mut f = testing_app_fixture.await?;
    let sender = f.sender_address;

    let rekey_to_account = f.algorand_fixture.generate_account(None).await?;
    let rekey_to_addr = rekey_to_account.account().address();
    let client = f.client;

    client
        .send()
        .opt_in(
            AppClientMethodCallParams {
                method: "opt_in".to_string(),
                args: vec![],
                sender: Some(sender.to_string()),
                rekey_to: Some(rekey_to_addr.to_string()),
                ..Default::default()
            },
            None,
        )
        .await?;

    let _payment_result = client
        .algorand()
        .send()
        .payment(
            PaymentParams {
                sender: sender.clone(),
                signer: Some(Arc::new(rekey_to_account.clone())),
                receiver: sender.clone(),
                amount: 0,
                ..Default::default()
            },
            None,
        )
        .await?;

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_sign_all_transactions_in_group_with_abi_call_with_transaction_arg(
    #[future] testing_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let mut f = testing_app_fixture.await?;
    let _sender = f.sender_address;

    let funded_account = f.algorand_fixture.generate_account(None).await?;
    let funded_addr = funded_account.account().address();

    let mut rng = rand::thread_rng();
    let amount = rng.gen_range(1..=10000);

    let payment_txn = f
        .algorand_fixture
        .algorand_client
        .create()
        .payment(PaymentParams {
            sender: funded_addr.clone(),
            receiver: funded_addr.clone(),
            amount,
            ..Default::default()
        })
        .await?;

    let called_indexes = Arc::new(Mutex::new(Vec::new()));

    struct IndexCapturingSigner {
        original_signer: Arc<dyn TransactionSigner>,
        called_indexes: Arc<Mutex<Vec<usize>>>,
    }

    #[async_trait]
    impl TransactionSigner for IndexCapturingSigner {
        async fn sign_transactions(
            &self,
            transactions: &[Transaction],
            indices: &[usize],
        ) -> Result<Vec<SignedTransaction>, String> {
            {
                let mut indexes = self.called_indexes.lock().unwrap();
                indexes.extend_from_slice(indices);
            }
            self.original_signer
                .sign_transactions(transactions, indices)
                .await
        }
    }

    let client = f.client;

    client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "call_abi_txn".to_string(),
                args: vec![
                    AppMethodCallArg::Transaction(payment_txn),
                    AppMethodCallArg::ABIValue(ABIValue::from("test")),
                ],
                sender: Some(funded_addr.to_string()),
                signer: Some(Arc::new(IndexCapturingSigner {
                    original_signer: Arc::new(funded_account.clone()),
                    called_indexes: called_indexes.clone(),
                })),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    let indexes = called_indexes.lock().unwrap().clone();

    assert_eq!(indexes, vec![0, 1], "Expected indexes 0 and 1 to be signed");

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_sign_transaction_in_group_with_different_signer_if_provided(
    #[future] testing_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let mut f = testing_app_fixture.await?;
    let sender = f.sender_address;

    let new_account = f.algorand_fixture.generate_account(None).await?;
    let new_addr = new_account.account().address();

    let payment_txn = f
        .algorand_fixture
        .algorand_client
        .create()
        .payment(PaymentParams {
            sender: new_addr.clone(),
            receiver: new_addr.clone(),
            amount: 2_000,
            ..Default::default()
        })
        .await?;
    let client = f.client;

    client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "call_abi_txn".to_string(),
                args: vec![
                    AppMethodCallArg::TransactionWithSigner(TransactionWithSigner {
                        transaction: payment_txn,
                        signer: Arc::new(new_account.clone()),
                    }),
                    AppMethodCallArg::ABIValue(ABIValue::from("test")),
                ],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_sign_nested_transactions_in_group_with_different_signers(
    #[future] nested_contract_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    eprintln!("=== Starting test_sign_transaction_in_group_with_different_signer_if_provided2 ===");
    let mut f = nested_contract_fixture.await?;
    let bob_account = f.algorand_fixture.generate_account(None).await?;
    let bob_addr = bob_account.account().address();

    let alice_account = f.algorand_fixture.generate_account(None).await?;
    let alice_addr = alice_account.account().address();

    let payment_txn = f
        .algorand_fixture
        .algorand_client
        .create()
        .payment(PaymentParams {
            sender: bob_addr.clone(),
            signer: Some(Arc::new(bob_account.clone())),
            receiver: bob_addr.clone(),
            amount: 2_000,
            ..Default::default()
        })
        .await?;
    let client = f.client;

    let result = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "nestedTxnArg".to_string(),
                args: vec![
                    AppMethodCallArg::TransactionPlaceholder,
                    AppMethodCallArg::AppCallMethodCall(AppCallMethodCallParams {
                        sender: bob_addr.clone(),
                        signer: Some(Arc::new(bob_account.clone())),
                        app_id: f.app_id,
                        method: ABIMethod::from_str("txnArg(pay)address").unwrap(),
                        args: vec![AppMethodCallArg::Transaction(payment_txn)],
                        ..Default::default()
                    }),
                ],
                sender: Some(alice_addr.to_string()),
                signer: Some(Arc::new(alice_account.clone())),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    assert_eq!(
        result.result.abi_return.as_ref().unwrap().return_value,
        Some(ABIValue::Uint(BigUint::from(client.app_id())))
    );

    Ok(())
}

#[rstest]
#[tokio::test]
async fn bare_call_with_box_reference_builds_and_sends(
    #[future] sandbox_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let f = sandbox_app_fixture.await?;
    let sender = f.sender_address;
    let client = f.client;

    let result = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "hello_world".to_string(),
                args: vec![AppMethodCallArg::ABIValue(ABIValue::from("test"))],
                sender: Some(sender.to_string()),
                box_references: Some(vec![BoxReference {
                    app_id: 0,
                    name: b"1".to_vec(),
                }]),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    match &result.result.transaction {
        algokit_transact::Transaction::AppCall(fields) => {
            assert_eq!(fields.app_id, f.app_id);
            assert_eq!(
                fields.box_references.as_ref().unwrap(),
                &vec![BoxReference {
                    app_id: 0,
                    name: b"1".to_vec()
                }]
            );
        }
        _ => return Err("expected app call".into()),
    }

    Ok(())
}
