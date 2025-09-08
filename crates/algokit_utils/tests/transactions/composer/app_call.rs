use crate::common::{
    AlgorandFixture, AlgorandFixtureResult, TestAccountConfig, TestResult, algorand_fixture,
    deploy_arc56_contract,
};
use algokit_abi::{ABIMethod, ABIReferenceValue, ABIReturn, ABIValue, Arc56Contract};
use algokit_test_artifacts::{nested_contract, sandbox};
use algokit_transact::{
    Address, OnApplicationComplete, PaymentTransactionFields, StateSchema, Transaction,
    TransactionHeader, TransactionId,
};
use algokit_utils::{AppCallMethodCallParams, AssetCreateParams, ComposerError};
use algokit_utils::{
    AppCallParams, AppCreateParams, AppDeleteParams, AppMethodCallArg, AppUpdateParams,
    PaymentParams,
};
use base64::{Engine, prelude::BASE64_STANDARD};
use num_bigint::BigUint;
use rstest::*;
use serde::Deserialize;
use std::str::FromStr;
use std::sync::Arc;

#[rstest]
#[tokio::test]
async fn test_app_call_transaction(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();

    let app_id = create_test_app(&algorand_fixture, &sender_address).await?;

    let app_call_params = AppCallParams {
        sender: sender_address.clone(),
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![b"Call".to_vec()]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_app_call(app_call_params)?;

    let result = composer.send(None).await?;
    let confirmation = &result.confirmations[0];

    assert!(
        confirmation.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        confirmation.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );

    let transaction = &confirmation.txn.transaction;

    match transaction {
        algokit_transact::Transaction::AppCall(app_call_fields) => {
            assert_eq!(app_call_fields.app_id, app_id, "App ID should match");
            assert_eq!(
                app_call_fields.on_complete,
                OnApplicationComplete::NoOp,
                "On Complete should match"
            );
            Ok(())
        }
        _ => Err("Transaction should be an app call transaction".into()),
    }
}

#[rstest]
#[tokio::test]
async fn test_app_create_transaction(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();

    let app_create_params = AppCreateParams {
        sender: sender_address.clone(),
        approval_program: HELLO_WORLD_APPROVAL_PROGRAM.to_vec(),
        clear_state_program: HELLO_WORLD_CLEAR_STATE_PROGRAM.to_vec(),
        args: Some(vec![b"Create".to_vec()]),
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_app_create(app_create_params)?;

    let result = composer.send(None).await?;
    let confirmation = &result.confirmations[0];

    assert!(
        confirmation.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        confirmation.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );

    let transaction = &confirmation.txn.transaction;

    match transaction {
        algokit_transact::Transaction::AppCall(app_call_fields) => {
            assert_eq!(
                app_call_fields.app_id, 0,
                "Application ID should be 0 for create"
            );
            assert_eq!(
                app_call_fields.on_complete,
                OnApplicationComplete::NoOp,
                "Clear state program should match"
            );
            assert_eq!(
                app_call_fields.approval_program,
                Some(HELLO_WORLD_APPROVAL_PROGRAM.to_vec()),
                "Approval program should match"
            );
            assert_eq!(
                app_call_fields.clear_state_program,
                Some(HELLO_WORLD_CLEAR_STATE_PROGRAM.to_vec()),
                "Clear state program should match"
            );
            Ok(())
        }
        _ => Err("Transaction should be an app call transaction".into()),
    }
}

#[rstest]
#[tokio::test]
async fn test_app_delete_transaction(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();

    let app_id = create_test_app(&algorand_fixture, &sender_address).await?;

    let app_delete_params = AppDeleteParams {
        sender: sender_address.clone(),
        app_id,
        args: Some(vec![b"Delete".to_vec()]),
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_app_delete(app_delete_params)?;

    let result = composer.send(None).await?;
    let confirmation = &result.confirmations[0];

    assert!(
        confirmation.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        confirmation.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );

    let transaction = &confirmation.txn.transaction;

    match transaction {
        algokit_transact::Transaction::AppCall(app_call_fields) => {
            assert_eq!(
                app_call_fields.app_id, app_id,
                "Application ID should match"
            );
            assert_eq!(
                app_call_fields.on_complete,
                OnApplicationComplete::DeleteApplication,
                "On Complete should be DeleteApplication"
            );
            Ok(())
        }
        _ => Err("Transaction should be an app delete transaction".into()),
    }
}

#[rstest]
#[tokio::test]
async fn test_app_update_transaction(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();

    let app_id = create_test_app(&algorand_fixture, &sender_address).await?;

    let app_update_params = AppUpdateParams {
        sender: sender_address.clone(),
        app_id,
        approval_program: HELLO_WORLD_CLEAR_STATE_PROGRAM.to_vec(), // Update the approval program
        clear_state_program: HELLO_WORLD_CLEAR_STATE_PROGRAM.to_vec(),
        args: Some(vec![b"Update".to_vec()]),
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_app_update(app_update_params)?;

    let result = composer.send(None).await?;

    let confirmation = &result.confirmations[0];

    assert!(
        confirmation.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        confirmation.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );
    let transaction = &confirmation.txn.transaction;

    match transaction {
        algokit_transact::Transaction::AppCall(app_call_fields) => {
            assert_eq!(
                app_call_fields.app_id, app_id,
                "Application ID should match"
            );
            assert_eq!(
                app_call_fields.on_complete,
                OnApplicationComplete::UpdateApplication,
                "On Complete should be UpdateApplication"
            );
            assert_eq!(
                app_call_fields.approval_program,
                Some(HELLO_WORLD_CLEAR_STATE_PROGRAM.to_vec()),
                "Updated approval program should match"
            );
            assert_eq!(
                app_call_fields.clear_state_program,
                Some(HELLO_WORLD_CLEAR_STATE_PROGRAM.to_vec()),
                "Clear state program should match"
            );
            Ok(())
        }
        _ => Err("Transaction should be an app update transaction".into()),
    }
}

#[rstest]
#[tokio::test]
async fn test_hello_world_app_method_call(
    #[future] arc56_algorand_fixture: Arc56AppFixtureResult,
) -> TestResult {
    let Arc56AppFixture {
        sender_address,
        app_id,
        arc56_contract,
        algorand_fixture,
    } = arc56_algorand_fixture.await?;

    let abi_method = get_abi_method(&arc56_contract, "hello_world")?;

    let args = vec![AppMethodCallArg::ABIValue(ABIValue::String(
        "world".to_string(),
    ))];

    let method_call_params = AppCallMethodCallParams {
        sender: sender_address.clone(),
        app_id,
        method: abi_method,
        args,
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;
    let abi_return = get_abi_return(&result.abi_returns, 0)?;

    match &abi_return.return_value {
        ABIValue::String(value) => {
            assert_eq!(value, "Hello, world",);
            Ok(())
        }
        _ => Err("Invalid return type".into()),
    }
}

#[rstest]
#[tokio::test]
async fn test_add_app_call_method_call(
    #[future] arc56_algorand_fixture: Arc56AppFixtureResult,
) -> TestResult {
    let Arc56AppFixture {
        sender_address,
        app_id,
        arc56_contract,
        algorand_fixture,
    } = arc56_algorand_fixture.await?;

    let abi_method = get_abi_method(&arc56_contract, "add")?;

    let args = vec![
        AppMethodCallArg::ABIValue(ABIValue::Uint(BigUint::from(1u8))),
        AppMethodCallArg::ABIValue(ABIValue::Uint(BigUint::from(2u8))),
    ];

    let method_call_params = AppCallMethodCallParams {
        sender: sender_address.clone(),
        app_id,
        method: abi_method,
        args,
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = get_abi_return(&result.abi_returns, 0)?;

    match &abi_return.return_value {
        ABIValue::Uint(value) => {
            assert_eq!(*value, BigUint::from(3u8));
            Ok(())
        }
        _ => Err("Invalid return type".into()),
    }
}

#[rstest]
#[tokio::test]
async fn test_echo_byte_app_call_method_call(
    #[future] arc56_algorand_fixture: Arc56AppFixtureResult,
) -> TestResult {
    let Arc56AppFixture {
        sender_address,
        app_id,
        arc56_contract,
        algorand_fixture,
    } = arc56_algorand_fixture.await?;

    let abi_method = get_abi_method(&arc56_contract, "echo_bytes")?;

    let test_array = vec![
        ABIValue::Byte(1u8),
        ABIValue::Byte(2u8),
        ABIValue::Byte(3u8),
        ABIValue::Byte(4u8),
    ];
    let args = vec![AppMethodCallArg::ABIValue(ABIValue::Array(
        test_array.clone(),
    ))];

    let method_call_params = AppCallMethodCallParams {
        sender: sender_address.clone(),
        app_id,
        method: abi_method,
        args,
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = get_abi_return(&result.abi_returns, 0)?;

    match &abi_return.return_value {
        ABIValue::Array(value) => {
            assert_eq!(*value, test_array);
            Ok(())
        }
        _ => Err("Invalid return type".into()),
    }
}

#[rstest]
#[tokio::test]
async fn test_echo_static_array_app_call_method_call(
    #[future] arc56_algorand_fixture: Arc56AppFixtureResult,
) -> TestResult {
    let Arc56AppFixture {
        sender_address,
        app_id,
        arc56_contract,
        algorand_fixture,
    } = arc56_algorand_fixture.await?;

    let abi_method = get_abi_method(&arc56_contract, "echo_static_array")?;

    let test_array = vec![
        ABIValue::Uint(BigUint::from(1u8)),
        ABIValue::Uint(BigUint::from(2u8)),
        ABIValue::Uint(BigUint::from(3u8)),
        ABIValue::Uint(BigUint::from(4u8)),
    ];
    let args = vec![AppMethodCallArg::ABIValue(ABIValue::Array(
        test_array.clone(),
    ))];

    let method_call_params = AppCallMethodCallParams {
        sender: sender_address.clone(),
        app_id,
        method: abi_method,
        args,
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = get_abi_return(&result.abi_returns, 0)?;

    match &abi_return.return_value {
        ABIValue::Array(value) => {
            assert_eq!(*value, test_array);
            Ok(())
        }
        _ => Err("Invalid return type".into()),
    }
}

#[rstest]
#[tokio::test]
async fn test_echo_dynamic_array_app_call_method_call(
    #[future] arc56_algorand_fixture: Arc56AppFixtureResult,
) -> TestResult {
    let Arc56AppFixture {
        sender_address,
        app_id,
        arc56_contract,
        algorand_fixture,
    } = arc56_algorand_fixture.await?;

    let abi_method = get_abi_method(&arc56_contract, "echo_dynamic_array")?;

    let test_array = vec![
        ABIValue::Uint(BigUint::from(10u8)),
        ABIValue::Uint(BigUint::from(20u8)),
        ABIValue::Uint(BigUint::from(30u8)),
    ];
    let args = vec![AppMethodCallArg::ABIValue(ABIValue::Array(
        test_array.clone(),
    ))];

    let method_call_params = AppCallMethodCallParams {
        sender: sender_address.clone(),
        app_id,
        method: abi_method,
        args,
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = get_abi_return(&result.abi_returns, 0)?;

    match &abi_return.return_value {
        ABIValue::Array(value) => {
            assert_eq!(*value, test_array, "Return array should match input array");
            Ok(())
        }
        _ => Err("Return value should be an array".into()),
    }
}

#[rstest]
#[tokio::test]
async fn test_nest_array_and_tuple_app_call_method_call(
    #[future] arc56_algorand_fixture: Arc56AppFixtureResult,
) -> TestResult {
    let Arc56AppFixture {
        sender_address,
        app_id,
        arc56_contract,
        algorand_fixture,
    } = arc56_algorand_fixture.await?;

    let abi_method = get_abi_method(&arc56_contract, "nest_array_and_tuple")?;

    let nested_array = vec![
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(1u8)),
            ABIValue::Uint(BigUint::from(2u8)),
        ]),
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(3u8)),
            ABIValue::Uint(BigUint::from(4u8)),
            ABIValue::Uint(BigUint::from(5u8)),
        ]),
    ];

    let tuple_arg = ABIValue::Array(vec![
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(10u8)),
            ABIValue::Uint(BigUint::from(20u8)),
        ]),
        ABIValue::String("test string".to_string()),
    ]);

    let args = vec![
        AppMethodCallArg::ABIValue(ABIValue::Array(nested_array.clone())),
        AppMethodCallArg::ABIValue(tuple_arg.clone()),
    ];

    let method_call_params = AppCallMethodCallParams {
        sender: sender_address.clone(),
        app_id,
        method: abi_method,
        args,
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = get_abi_return(&result.abi_returns, 0)?;

    match &abi_return.return_value {
        ABIValue::Array(returned_tuple) => {
            assert_eq!(
                returned_tuple.len(),
                2,
                "Return tuple should have 2 elements"
            );

            match &returned_tuple[0] {
                ABIValue::Array(returned_nested_array) => {
                    assert_eq!(
                        *returned_nested_array, nested_array,
                        "Returned nested array should match input"
                    );
                }
                _ => return Err("First element should be a nested array".into()),
            };

            match &returned_tuple[1] {
                ABIValue::Array(returned_inner_tuple) => {
                    assert_eq!(
                        *returned_inner_tuple,
                        match &tuple_arg {
                            ABIValue::Array(t) => t.clone(),
                            _ => return Err("tuple_arg should be an array".into()),
                        },
                        "Returned inner tuple should match input"
                    );
                }
                _ => return Err("Second element should be a tuple".into()),
            };
            Ok(())
        }
        _ => Err("Return value should be a tuple".into()),
    }
}

#[rstest]
#[tokio::test]
async fn test_get_pay_txn_amount_app_call_method_call(
    #[future] arc56_algorand_fixture: Arc56AppFixtureResult,
) -> TestResult {
    let Arc56AppFixture {
        sender_address,
        app_id,
        arc56_contract,
        mut algorand_fixture,
    } = arc56_algorand_fixture.await?;

    let receiver = algorand_fixture.generate_account(None).await?;
    let receiver_addr = receiver.account().address();

    let abi_method = get_abi_method(&arc56_contract, "get_pay_txn_amount")?;

    let payment_amount = 1_234_567u64;
    let args = vec![AppMethodCallArg::Payment(PaymentParams {
        sender: sender_address.clone(),
        receiver: receiver_addr,
        amount: payment_amount,
        ..Default::default()
    })];

    let method_call_params = AppCallMethodCallParams {
        sender: sender_address.clone(),
        app_id,
        method: abi_method,
        args,
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = get_abi_return(&result.abi_returns, 0)?;

    match &abi_return.return_value {
        ABIValue::Uint(returned_amount) => {
            assert_eq!(
                *returned_amount,
                BigUint::from(payment_amount),
                "Returned amount should match payment amount"
            );
            Ok(())
        }
        _ => Err("Return value should be a UInt".into()),
    }
}

#[rstest]
#[tokio::test]
async fn test_get_pay_txn_amount_app_call_method_call_using_a_different_signer(
    #[future] arc56_algorand_fixture: Arc56AppFixtureResult,
) -> TestResult {
    let Arc56AppFixture {
        sender_address,
        app_id,
        arc56_contract,
        mut algorand_fixture,
    } = arc56_algorand_fixture.await?;

    let receiver = algorand_fixture.generate_account(None).await?;
    let receiver_addr = receiver.account().address();

    let alice = algorand_fixture.generate_account(None).await?;
    let alice_addr = alice.account().address();

    let abi_method = get_abi_method(&arc56_contract, "get_pay_txn_amount")?;

    let payment_amount = 1_234_567u64;
    let alice_signer = Arc::new(alice.clone());
    let args = vec![AppMethodCallArg::Payment(PaymentParams {
        sender: alice_addr.clone(),
        signer: Some(alice_signer),
        receiver: receiver_addr,
        amount: payment_amount,
        ..Default::default()
    })];

    let method_call_params = AppCallMethodCallParams {
        sender: sender_address.clone(),
        app_id,
        method: abi_method,
        args,
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = get_abi_return(&result.abi_returns, 0)?;

    match &abi_return.return_value {
        ABIValue::Uint(returned_amount) => {
            assert_eq!(
                *returned_amount,
                BigUint::from(payment_amount),
                "Returned amount should match payment amount"
            );
            Ok(())
        }
        _ => Err("Return value should be a UInt".into()),
    }
}

#[rstest]
#[tokio::test]
async fn test_get_returned_value_of_app_call_txn_app_call_method_call(
    #[future] arc56_algorand_fixture: Arc56AppFixtureResult,
) -> TestResult {
    let Arc56AppFixture {
        sender_address,
        app_id,
        arc56_contract,
        mut algorand_fixture,
    } = arc56_algorand_fixture.await?;

    let receiver = algorand_fixture.generate_account(None).await?;
    let receiver_addr = receiver.account().address();

    let get_pay_txn_amount_method = get_abi_method(&arc56_contract, "get_pay_txn_amount")?;

    let get_returned_value_of_app_call_txn_method =
        get_abi_method(&arc56_contract, "get_returned_value_of_app_call_txn")?;

    let payment_amount = 2_500_000u64;
    let mut composer = algorand_fixture.algorand_client.new_group();

    let first_method_call_params = AppCallMethodCallParams {
        sender: sender_address.clone(),
        app_id,
        method: get_pay_txn_amount_method,
        args: vec![AppMethodCallArg::Payment(PaymentParams {
            sender: sender_address.clone(),
            receiver: receiver_addr,
            amount: payment_amount,
            ..Default::default()
        })],
        ..Default::default()
    };

    let second_method_call_params = AppCallMethodCallParams {
        sender: sender_address.clone(),
        app_id,
        method: get_returned_value_of_app_call_txn_method,
        args: vec![AppMethodCallArg::AppCallMethodCall(
            first_method_call_params,
        )],
        ..Default::default()
    };

    composer.add_app_call_method_call(second_method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = get_abi_return(&result.abi_returns, 0)?;

    match &abi_return.return_value {
        ABIValue::Uint(returned_amount) => {
            assert_eq!(
                *returned_amount,
                BigUint::from(payment_amount),
                "Returned amount should match payment amount"
            );
            Ok(())
        }
        _ => Err("Return value should be a UInt".into()),
    }
}

#[rstest]
#[tokio::test]
async fn test_get_returned_value_of_nested_app_call_method_calls(
    #[future] arc56_algorand_fixture: Arc56AppFixtureResult,
) -> TestResult {
    let Arc56AppFixture {
        sender_address,
        app_id,
        arc56_contract,
        mut algorand_fixture,
    } = arc56_algorand_fixture.await?;

    let receiver = algorand_fixture.generate_account(None).await?;
    let receiver_addr = receiver.account().address();

    let get_pay_txn_amount_method = get_abi_method(&arc56_contract, "get_pay_txn_amount")?;

    let get_pay_txns_amount_sum_method =
        get_abi_method(&arc56_contract, "get_pay_txns_amount_sum")?;

    let payment_amount = 5_000u64;
    let mut composer = algorand_fixture.algorand_client.new_group();

    let get_pay_txn_amount_method_call_params = AppCallMethodCallParams {
        sender: sender_address.clone(),
        app_id,
        method: get_pay_txn_amount_method,
        args: vec![AppMethodCallArg::Payment(PaymentParams {
            sender: sender_address.clone(),
            receiver: receiver_addr.clone(),
            amount: payment_amount,
            ..Default::default()
        })],
        ..Default::default()
    };

    let get_pay_txns_amount_sum_method_method_call_params = AppCallMethodCallParams {
        sender: sender_address.clone(),
        app_id,
        method: get_pay_txns_amount_sum_method,
        args: vec![
            AppMethodCallArg::Payment(PaymentParams {
                sender: sender_address.clone(),
                receiver: receiver_addr.clone(),
                amount: payment_amount,
                note: Some("second txn".as_bytes().to_vec()),
                ..Default::default()
            }),
            AppMethodCallArg::TransactionPlaceholder,
            AppMethodCallArg::AppCallMethodCall(get_pay_txn_amount_method_call_params),
        ],
        ..Default::default()
    };

    composer.add_app_call_method_call(get_pay_txns_amount_sum_method_method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = get_abi_return(&result.abi_returns, 1)?;

    let expected_result = BigUint::from(15_000u64);
    match &abi_return.return_value {
        ABIValue::Uint(returned_amount) => {
            assert_eq!(
                *returned_amount, expected_result,
                "Returned amount should match payment amount"
            );
            Ok(())
        }
        _ => Err("Return value should be a UInt".into()),
    }
}

struct Arc56AppFixture {
    sender_address: Address,
    app_id: u64,
    arc56_contract: Arc56Contract,
    algorand_fixture: AlgorandFixture,
}

type Arc56AppFixtureResult = Result<Arc56AppFixture, Box<dyn std::error::Error + Send + Sync>>;

#[fixture]
async fn arc56_algorand_fixture(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> Arc56AppFixtureResult {
    let algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();

    let arc56_contract: Arc56Contract = serde_json::from_str(sandbox::APPLICATION_ARC56)?;
    let app_id = deploy_arc56_contract(&algorand_fixture, &sender_address, &arc56_contract).await?;

    Ok(Arc56AppFixture {
        sender_address,
        app_id,
        arc56_contract,
        algorand_fixture,
    })
}

// Raw (non ABI) hello world approval program
const HELLO_WORLD_APPROVAL_PROGRAM: [u8; 18] = [
    10, 128, 7, 72, 101, 108, 108, 111, 44, 32, 54, 26, 0, 80, 176, 129, 1, 67,
];
// Raw (non ABI) hello world clear state program
const HELLO_WORLD_CLEAR_STATE_PROGRAM: [u8; 4] = [10, 129, 1, 67];

async fn create_test_app(
    algorand_fixture: &AlgorandFixture,
    sender: &Address,
) -> Result<u64, ComposerError> {
    let app_create_params = AppCreateParams {
        sender: sender.clone(),
        approval_program: HELLO_WORLD_APPROVAL_PROGRAM.to_vec(),
        clear_state_program: HELLO_WORLD_CLEAR_STATE_PROGRAM.to_vec(),
        global_state_schema: Some(StateSchema {
            num_uints: 1,
            num_byte_slices: 1,
        }),
        local_state_schema: Some(StateSchema {
            num_uints: 1,
            num_byte_slices: 1,
        }),
        args: Some(vec![b"Create".to_vec()]),
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();

    composer.add_app_create(app_create_params)?;

    let result = composer.send(None).await?;

    Ok(result.confirmations[0]
        .app_id
        .expect("App Id must be returned"))
}

#[rstest]
#[tokio::test]
async fn test_more_than_15_args_with_ref_types_app_call_method_call(
    #[future] arc56_algorand_fixture: Arc56AppFixtureResult,
) -> TestResult {
    let Arc56AppFixture {
        sender_address,
        app_id,
        arc56_contract,
        mut algorand_fixture,
    } = arc56_algorand_fixture.await?;

    let receiver = algorand_fixture
        .generate_account(Some(TestAccountConfig {
            initial_funds: 0u64,
            ..Default::default()
        }))
        .await?;
    let receiver_addr = receiver.account().address();

    let asset_create_params = AssetCreateParams {
        sender: sender_address.clone(),
        total: 1_000_000,
        decimals: Some(2),
        default_frozen: Some(false),
        asset_name: Some("Test Asset".to_string()),
        unit_name: Some("TEST".to_string()),
        url: Some("https://example.com".to_string()),
        metadata_hash: None,
        manager: Some(sender_address.clone()),
        reserve: Some(sender_address.clone()),
        freeze: Some(sender_address.clone()),
        clawback: Some(sender_address.clone()),
        ..Default::default()
    };

    let mut asset_composer = algorand_fixture.algorand_client.new_group();
    asset_composer.add_asset_create(asset_create_params)?;

    let asset_result = asset_composer.send(None).await?;
    let asset_id = asset_result.confirmations[0]
        .asset_id
        .expect("No asset ID returned");

    let abi_method = get_abi_method(&arc56_contract, "more_than_15_args_with_ref_types")?;

    let tx_params = algorand_fixture.algod.transaction_params().await?;

    let payment_amount = 200_000u64;
    let genesis_hash: Option<[u8; 32]> = tx_params.genesis_hash.try_into().ok();
    let payment_transaction = Transaction::Payment(PaymentTransactionFields {
        header: TransactionHeader {
            sender: sender_address.clone(),
            fee: Some(tx_params.min_fee),
            first_valid: tx_params.last_round,
            last_valid: tx_params.last_round + 1000,
            genesis_hash,
            genesis_id: Some(tx_params.genesis_id),
            note: None,
            rekey_to: None,
            lease: None,
            group: None,
        },
        receiver: receiver_addr.clone(),
        amount: payment_amount,
        close_remainder_to: None,
    });

    let mut args = vec![];

    for i in 1..=17 {
        args.push(AppMethodCallArg::ABIValue(ABIValue::Uint(BigUint::from(
            i as u64,
        ))));
    }

    args.push(AppMethodCallArg::ABIReference(ABIReferenceValue::Asset(
        asset_id,
    )));

    args.push(AppMethodCallArg::ABIValue(ABIValue::Uint(BigUint::from(
        18u64,
    ))));

    args.push(AppMethodCallArg::ABIReference(
        ABIReferenceValue::Application(app_id),
    ));

    args.push(AppMethodCallArg::Transaction(payment_transaction));

    args.push(AppMethodCallArg::ABIReference(ABIReferenceValue::Account(
        receiver_addr.to_string(),
    )));

    let method_call_params = AppCallMethodCallParams {
        sender: sender_address.clone(),
        app_id,
        method: abi_method,
        args,
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = get_abi_return(&result.abi_returns, 0)?;

    match &abi_return.return_value {
        ABIValue::Array(returned_tuple) => {
            assert_eq!(
                returned_tuple.len(),
                4,
                "Return tuple should have 4 elements"
            );

            match &returned_tuple[0] {
                ABIValue::Uint(returned_asset_id) => {
                    assert_eq!(
                        *returned_asset_id,
                        BigUint::from(asset_id),
                        "Returned asset ID should match created asset"
                    );
                }
                _ => return Err("First element should be asset ID".into()),
            }

            match &returned_tuple[1] {
                ABIValue::Uint(returned_app_id) => {
                    assert_eq!(
                        *returned_app_id,
                        BigUint::from(app_id),
                        "Returned app ID should match deployed app"
                    );
                }
                _ => return Err("Second element should be app ID".into()),
            }

            match &returned_tuple[2] {
                ABIValue::Uint(returned_balance) => {
                    assert_eq!(*returned_balance, BigUint::from(payment_amount),);
                }
                _ => return Err("Third element should be account balance".into()),
            }

            match &returned_tuple[3] {
                ABIValue::Array(txn_id_bytes) => {
                    assert!(
                        !txn_id_bytes.is_empty(),
                        "Transaction ID should not be empty"
                    );

                    let signed_group = composer
                        .gather_signatures()
                        .await
                        .expect("Signed group should be available after send");

                    let actual_txn_id = signed_group[0]
                        .id_raw()
                        .expect("Failed to get raw transaction ID")
                        .to_vec();

                    let returned_txn_id: Vec<u8> = txn_id_bytes
                        .iter()
                        .map(|byte_abi_value| match byte_abi_value {
                            ABIValue::Byte(b) => Ok(*b),
                            _ => Err("Transaction ID bytes should be ABIValue::Byte"),
                        })
                        .collect::<Result<Vec<_>, _>>()?;

                    assert_eq!(
                        returned_txn_id, actual_txn_id,
                        "Returned transaction ID should match actual transaction ID"
                    );
                }
                _ => return Err("Fourth element should be transaction ID bytes".into()),
            }
        }
        _ => return Err("Return value should be a tuple".into()),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_more_than_15_args_app_call_method_call(
    #[future] arc56_algorand_fixture: Arc56AppFixtureResult,
) -> TestResult {
    let Arc56AppFixture {
        sender_address,
        app_id,
        arc56_contract,
        algorand_fixture,
    } = arc56_algorand_fixture.await?;

    let abi_method = get_abi_method(&arc56_contract, "more_than_15_args")?;

    let mut args = vec![];
    for i in 1..=18 {
        args.push(AppMethodCallArg::ABIValue(ABIValue::Uint(BigUint::from(
            i as u64,
        ))));
    }

    let method_call_params = AppCallMethodCallParams {
        sender: sender_address.clone(),
        app_id,
        method: abi_method,
        args,
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = get_abi_return(&result.abi_returns, 0)?;

    match &abi_return.return_value {
        ABIValue::Array(returned_array) => {
            assert_eq!(
                returned_array.len(),
                18,
                "Return array should have 18 elements"
            );

            for (i, element) in returned_array.iter().enumerate() {
                match element {
                    ABIValue::Uint(returned_value) => {
                        assert_eq!(
                            *returned_value,
                            BigUint::from((i + 1) as u64),
                            "Element {} should match input value {}",
                            i,
                            i + 1
                        );
                    }
                    _ => return Err(format!("Array element {} should be UInt64", i).into()),
                }
            }
        }
        _ => return Err("Return value should be a dynamic array".into()),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_app_call_validation_errors(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();

    // Test app call with invalid app_id (0)
    let invalid_app_call_params = AppCallParams {
        sender: sender_address.clone(),
        app_id: 0, // Invalid: should be > 0 for app calls (0 is for app creation)
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer
        .add_app_call(invalid_app_call_params)
        .expect("Adding invalid app call should succeed at composer level");

    // The validation should fail when building the transaction group
    let result = composer.build(None).await;

    // The build should return an error due to validation failures
    assert!(
        result.is_err(),
        "Build with invalid app call parameters should fail"
    );

    let error = result.unwrap_err();
    let error_string = error.to_string();

    // Check that the error contains validation-related messages from the transact crate
    assert!(
        error_string.contains("validation")
            || error_string.contains("app_id")
            || error_string.contains("Application")
            || error_string.contains("zero")
            || error_string.contains("0"),
        "Error should contain validation failure details: {}",
        error_string
    );

    Ok(())
}

fn get_abi_method(
    arc56_contract: &Arc56Contract,
    name: &str,
) -> Result<ABIMethod, Box<dyn std::error::Error + Send + Sync>> {
    let method = arc56_contract
        .methods
        .iter()
        .find(|m| m.name == name)
        .ok_or_else(|| format!("Failed to find {} method", name))?
        .try_into()
        .map_err(|e| format!("Failed to convert ARC56 method to ABI method: {}", e))?;
    Ok(method)
}

fn get_abi_return(
    abi_returns: &[Result<Option<ABIReturn>, ComposerError>],
    index: usize,
) -> Result<&ABIReturn, Box<dyn std::error::Error + Send + Sync>> {
    if index >= abi_returns.len() {
        return Err("Index out of range".into());
    }

    let abi_result = &abi_returns[index];
    match abi_result {
        Ok(Some(abi_return)) => Ok(abi_return),
        Ok(None) => Err("ABI result expected".into()),
        Err(e) => Err(format!("Failed to parse ABI result: {}", e).into()),
    }
}

#[rstest]
#[tokio::test]
async fn test_double_nested(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;
    let mut composer = algorand_fixture.algorand_client.new_group();

    let sender_address = algorand_fixture.test_account.account().address();
    let receiver = algorand_fixture.generate_account(None).await?;
    let receiver_address = receiver.account().address();

    let app_id = deploy_nested_app(&algorand_fixture).await?;

    let first_txn_arg = AppCallMethodCallParams {
        sender: sender_address.clone(),
        note: Some("first_txn_arg".as_bytes().to_vec()),
        app_id,
        method: ABIMethod::from_str("txnArg(pay)address")?,
        args: vec![AppMethodCallArg::Payment(PaymentParams {
            sender: sender_address.clone(),
            receiver: receiver_address.clone(),
            amount: 2_500_000u64,
            ..Default::default()
        })],
        ..Default::default()
    };

    let second_txn_arg = AppCallMethodCallParams {
        sender: sender_address.clone(),
        note: Some("second_txn_arg".as_bytes().to_vec()),
        app_id,
        method: ABIMethod::from_str("txnArg(pay)address")?,
        args: vec![AppMethodCallArg::Payment(PaymentParams {
            sender: sender_address.clone(),
            receiver: receiver_address.clone(),
            amount: 1_500_000u64,
            ..Default::default()
        })],
        ..Default::default()
    };

    let method_call_params = AppCallMethodCallParams {
        sender: sender_address.clone(),
        app_id,
        method: ABIMethod::from_str("doubleNestedTxnArg(pay,appl,pay,appl)uint64")?,
        args: vec![
            AppMethodCallArg::AppCallMethodCall(first_txn_arg),
            AppMethodCallArg::AppCallMethodCall(second_txn_arg),
        ],
        ..Default::default()
    };

    composer.add_app_call_method_call(method_call_params)?;
    let result: algokit_utils::SendTransactionComposerResults = composer.send(None).await?;

    let abi_return_0 = get_abi_return(&result.abi_returns, 0)?;
    if let ABIValue::Address(value) = &abi_return_0.return_value {
        assert_eq!(
            *value,
            sender_address.as_str(),
            "Returned address should match with sender address"
        );
    } else {
        return Err("First return value should be an Address".into());
    }

    // Second assertion
    let abi_return_1 = get_abi_return(&result.abi_returns, 1)?;
    if let ABIValue::Address(value) = &abi_return_1.return_value {
        assert_eq!(
            *value,
            sender_address.as_str(),
            "Returned address should match with sender address"
        );
    } else {
        return Err("Second return value should be an Address".into());
    }

    // Third assertion
    let abi_return_2 = get_abi_return(&result.abi_returns, 2)?;
    if let ABIValue::Uint(value) = &abi_return_2.return_value {
        assert_eq!(
            *value,
            BigUint::from(app_id),
            "Returned value should match with app ID"
        );
    } else {
        return Err("Third return value should be a Uint".into());
    }

    Ok(())
}

#[derive(Deserialize)]
struct TealSource {
    approval: String,
    clear: String,
}

#[derive(Deserialize)]
struct Arc32AppSpec {
    source: Option<TealSource>,
}

async fn deploy_nested_app(
    algorand_fixture: &AlgorandFixture,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let app_spec: Arc32AppSpec = serde_json::from_str(nested_contract::APPLICATION)?;
    let teal_source = app_spec.source.unwrap();
    let approval_bytes = BASE64_STANDARD.decode(teal_source.approval)?;
    let clear_state_bytes = BASE64_STANDARD.decode(teal_source.clear)?;

    let approval_compile_result = algorand_fixture
        .algod
        .teal_compile(approval_bytes, None)
        .await?;
    let clear_state_compile_result = algorand_fixture
        .algod
        .teal_compile(clear_state_bytes, None)
        .await?;

    let create_method = ABIMethod::from_str("createApplication()void")?;
    let create_method_selector = create_method.selector()?;

    let app_create_params = AppCreateParams {
        sender: algorand_fixture.test_account.account().address(),
        approval_program: approval_compile_result.result,
        clear_state_program: clear_state_compile_result.result,
        args: Some(vec![create_method_selector]),
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group();
    composer.add_app_create(app_create_params)?;

    let result = composer.send(None).await?;

    result.confirmations[0]
        .app_id
        .ok_or_else(|| "No app id returned".into())
}
