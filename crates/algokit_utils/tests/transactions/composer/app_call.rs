use std::sync::Arc;

use crate::common::init_test_logging;
use algokit_abi::{ABIReferenceValue, ABIValue, Arc56Contract};
use algokit_test_artifacts::sandbox;
use algokit_transact::{
    Address, OnApplicationComplete, PaymentTransactionFields, StateSchema, Transaction,
    TransactionHeader, TransactionId,
};
use algokit_utils::{AppCallMethodCallParams, AssetCreateParams, CommonParams};
use algokit_utils::{
    AppCallParams, AppCreateParams, AppDeleteParams, AppMethodCallArg, AppUpdateParams,
    PaymentParams, testing::*,
};
use base64::prelude::*;
use num_bigint::BigUint;
use rstest::*;

#[tokio::test]
async fn test_app_call_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let app_id = create_test_app(context, sender_addr.clone())
        .await
        .expect("Failed to create test app");

    println!("Created test app with ID: {}", app_id);

    let app_call_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![b"Call".to_vec()]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_app_call(app_call_params)
        .expect("Failed to add app call");

    let result = composer.send(None).await.expect("Failed to send app call");
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
        }
        _ => panic!("Transaction should be an app call transaction"),
    }
}

#[tokio::test]
async fn test_app_create_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let app_create_params = AppCreateParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        on_complete: OnApplicationComplete::NoOp,
        approval_program: HELLO_WORLD_APPROVAL_PROGRAM.to_vec(),
        clear_state_program: HELLO_WORLD_CLEAR_STATE_PROGRAM.to_vec(),
        global_state_schema: None,
        local_state_schema: None,
        extra_program_pages: None,
        args: Some(vec![b"Create".to_vec()]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_app_create(app_create_params)
        .expect("Failed to add app create");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send app create");
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
        }
        _ => panic!("Transaction should be an app call transaction"),
    }
}

#[tokio::test]
async fn test_app_delete_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let app_id = create_test_app(context, sender_addr.clone())
        .await
        .expect("Failed to create test app");

    let app_delete_params = AppDeleteParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        args: Some(vec![b"Delete".to_vec()]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_app_delete(app_delete_params)
        .expect("Failed to add app delete");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send app delete");
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
        }
        _ => panic!("Transaction should be an app delete transaction"),
    }
}

#[tokio::test]
async fn test_app_update_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let app_id = create_test_app(context, sender_addr.clone())
        .await
        .expect("Failed to create test app");

    let app_update_params = AppUpdateParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        approval_program: HELLO_WORLD_CLEAR_STATE_PROGRAM.to_vec(), // Update the approval program
        clear_state_program: HELLO_WORLD_CLEAR_STATE_PROGRAM.to_vec(),
        args: Some(vec![b"Update".to_vec()]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_app_update(app_update_params)
        .expect("Failed to add app update");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send app update");

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
        }
        _ => panic!("Transaction should be an app update transaction"),
    }
}

#[rstest]
#[tokio::test]
async fn test_hello_world_app_method_call(#[future] setup: SetupResult) -> TestResult {
    let TestData {
        sender_address: sender_addr,
        app_id,
        arc56_contract,
        fixture,
    } = setup.await?;

    let context = fixture.context()?;

    let abi_method = arc56_contract
        .methods
        .iter()
        .find(|m| m.name == "hello_world")
        .expect("Failed to find hello_world method")
        .try_into()
        .expect("Failed to convert ARC56 method to ABI method");

    let args = vec![AppMethodCallArg::ABIValue(ABIValue::String(
        "world".to_string(),
    ))];

    let method_call_params = AppCallMethodCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        method: abi_method,
        args,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
    };

    let mut composer = context.composer.clone();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = match &result.abi_returns[0] {
        Ok(Some(abi_return)) => abi_return,
        Ok(None) => panic!("ABI return should be Some"),
        Err(e) => panic!("ABI return should be Ok: {:?}", e),
    };

    match &abi_return.return_value {
        ABIValue::String(value) => {
            assert_eq!(value, "Hello, world",);
        }
        _ => panic!("Invalid return type"),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_add_app_call_method_call(#[future] setup: SetupResult) -> TestResult {
    let TestData {
        sender_address: sender_addr,
        app_id,
        arc56_contract,
        fixture,
    } = setup.await?;

    let context = fixture.context()?;

    let abi_method = arc56_contract
        .methods
        .iter()
        .find(|m| m.name == "add")
        .expect("Failed to find add method")
        .try_into()
        .expect("Failed to convert ARC56 method to ABI method");

    let args = vec![
        AppMethodCallArg::ABIValue(ABIValue::Uint(BigUint::from(1u8))),
        AppMethodCallArg::ABIValue(ABIValue::Uint(BigUint::from(2u8))),
    ];

    let method_call_params = AppCallMethodCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        method: abi_method,
        args,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
    };

    let mut composer = context.composer.clone();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = match &result.abi_returns[0] {
        Ok(Some(abi_return)) => abi_return,
        Ok(None) => panic!("ABI return should be Some"),
        Err(e) => panic!("ABI return should be Ok: {:?}", e),
    };

    match &abi_return.return_value {
        ABIValue::Uint(value) => {
            assert_eq!(*value, BigUint::from(3u8));
        }
        _ => panic!("Invalid return type"),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_echo_byte_app_call_method_call(#[future] setup: SetupResult) -> TestResult {
    let TestData {
        sender_address: sender_addr,
        app_id,
        arc56_contract,
        fixture,
    } = setup.await?;

    let context = fixture.context()?;

    let abi_method = arc56_contract
        .methods
        .iter()
        .find(|m| m.name == "echo_bytes")
        .expect("Failed to find echo_bytes method")
        .try_into()
        .expect("Failed to convert ARC56 method to ABI method");

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
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        method: abi_method,
        args,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
    };

    let mut composer = context.composer.clone();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = match &result.abi_returns[0] {
        Ok(Some(abi_return)) => abi_return,
        Ok(None) => panic!("ABI return should be Some"),
        Err(e) => panic!("ABI return should be Ok: {:?}", e),
    };

    match &abi_return.return_value {
        ABIValue::Array(value) => {
            assert_eq!(*value, test_array);
        }
        _ => panic!("Invalid return type"),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_echo_static_array_app_call_method_call(#[future] setup: SetupResult) -> TestResult {
    let TestData {
        sender_address: sender_addr,
        app_id,
        arc56_contract,
        fixture,
    } = setup.await?;

    let context = fixture.context()?;

    let abi_method = arc56_contract
        .methods
        .iter()
        .find(|m| m.name == "echo_static_array")
        .expect("Failed to find echo_static_array method")
        .try_into()
        .expect("Failed to convert ARC56 method to ABI method");

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
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        method: abi_method,
        args,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
    };

    let mut composer = context.composer.clone();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = match &result.abi_returns[0] {
        Ok(Some(abi_return)) => abi_return,
        Ok(None) => panic!("ABI return should be Some"),
        Err(e) => panic!("ABI return should be Ok: {:?}", e),
    };

    match &abi_return.return_value {
        ABIValue::Array(value) => {
            assert_eq!(*value, test_array);
        }
        _ => panic!("Invalid return type"),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_echo_dynamic_array_app_call_method_call(#[future] setup: SetupResult) -> TestResult {
    let TestData {
        sender_address: sender_addr,
        app_id,
        arc56_contract,
        fixture,
    } = setup.await?;

    let context = fixture.context()?;

    let abi_method = arc56_contract
        .methods
        .iter()
        .find(|m| m.name == "echo_dynamic_array")
        .expect("Failed to find echo_dynamic_array method")
        .try_into()
        .expect("Failed to convert ARC56 method to ABI method");

    let test_array = vec![
        ABIValue::Uint(BigUint::from(10u8)),
        ABIValue::Uint(BigUint::from(20u8)),
        ABIValue::Uint(BigUint::from(30u8)),
    ];
    let args = vec![AppMethodCallArg::ABIValue(ABIValue::Array(
        test_array.clone(),
    ))];

    let method_call_params = AppCallMethodCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        method: abi_method,
        args,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
    };

    let mut composer = context.composer.clone();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = match &result.abi_returns[0] {
        Ok(Some(abi_return)) => abi_return,
        Ok(None) => panic!("ABI return should be Some"),
        Err(e) => panic!("ABI return should be Ok: {:?}", e),
    };

    match &abi_return.return_value {
        ABIValue::Array(value) => {
            assert_eq!(*value, test_array, "Return array should match input array");
        }
        _ => panic!("Return value should be an array"),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_nest_array_and_tuple_app_call_method_call(
    #[future] setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address: sender_addr,
        app_id,
        arc56_contract,
        fixture,
    } = setup.await?;

    let context = fixture.context()?;

    let abi_method = arc56_contract
        .methods
        .iter()
        .find(|m| m.name == "nest_array_and_tuple")
        .expect("Failed to find nest_array_and_tuple method")
        .try_into()
        .expect("Failed to convert ARC56 method to ABI method");

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
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        method: abi_method,
        args,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
    };

    let mut composer = context.composer.clone();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = match &result.abi_returns[0] {
        Ok(Some(abi_return)) => abi_return,
        Ok(None) => panic!("ABI return should be Some"),
        Err(e) => panic!("ABI return should be Ok: {:?}", e),
    };

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
                _ => panic!("First element should be a nested array"),
            }

            match &returned_tuple[1] {
                ABIValue::Array(returned_inner_tuple) => {
                    assert_eq!(
                        *returned_inner_tuple,
                        match &tuple_arg {
                            ABIValue::Array(t) => t.clone(),
                            _ => panic!("tuple_arg should be an array"),
                        },
                        "Returned inner tuple should match input"
                    );
                }
                _ => panic!("Second element should be a tuple"),
            }
        }
        _ => panic!("Return value should be a tuple"),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_get_pay_txn_amount_app_call_method_call(#[future] setup: SetupResult) -> TestResult {
    let TestData {
        sender_address: sender_addr,
        app_id,
        arc56_contract,
        mut fixture,
    } = setup.await?;

    let receiver = fixture
        .generate_account(None)
        .await
        .expect("Failed to create receiver account");
    let receiver_addr = receiver
        .account()
        .expect("Failed to get receiver account")
        .address();

    let context = fixture.context()?;

    let abi_method = arc56_contract
        .methods
        .iter()
        .find(|m| m.name == "get_pay_txn_amount")
        .expect("Failed to find get_pay_txn_amount method")
        .try_into()
        .expect("Failed to convert ARC56 method to ABI method");

    let payment_amount = 1_234_567u64;
    let args = vec![AppMethodCallArg::Payment(PaymentParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        receiver: receiver_addr,
        amount: payment_amount,
    })];

    let method_call_params = AppCallMethodCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        method: abi_method,
        args,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
    };

    let mut composer = context.composer.clone();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = match &result.abi_returns[0] {
        Ok(Some(abi_return)) => abi_return,
        Ok(None) => panic!("ABI return should be Some"),
        Err(e) => panic!("ABI return should be Ok: {:?}", e),
    };

    match &abi_return.return_value {
        ABIValue::Uint(returned_amount) => {
            assert_eq!(
                *returned_amount,
                BigUint::from(payment_amount),
                "Returned amount should match payment amount"
            );
        }
        _ => panic!("Return value should be a UInt"),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_get_pay_txn_amount_app_call_method_call_using_a_different_signer(
    #[future] setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address: sender_addr,
        app_id,
        arc56_contract,
        mut fixture,
    } = setup.await?;

    let receiver = fixture
        .generate_account(None)
        .await
        .expect("Failed to create receiver account");
    let receiver_addr = receiver
        .account()
        .expect("Failed to get receiver account")
        .address();

    let alice = fixture
        .generate_account(None)
        .await
        .expect("Failed to create Alice account");
    let alice_addr = alice
        .account()
        .expect("Failed to get Alice account")
        .address();

    let context = fixture.context()?;

    let abi_method = arc56_contract
        .methods
        .iter()
        .find(|m| m.name == "get_pay_txn_amount")
        .expect("Failed to find get_pay_txn_amount method")
        .try_into()
        .expect("Failed to convert ARC56 method to ABI method");

    let payment_amount = 1_234_567u64;
    let alice_signer = Arc::new(alice.clone());
    let args = vec![AppMethodCallArg::Payment(PaymentParams {
        common_params: CommonParams {
            sender: alice_addr.clone(), // Alice sends and signs the payment
            signer: Some(alice_signer),
            ..Default::default()
        },
        receiver: receiver_addr,
        amount: payment_amount,
    })];

    let method_call_params = AppCallMethodCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(), // Default test account still makes the method call
            ..Default::default()
        },
        app_id,
        method: abi_method,
        args,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
    };

    let mut composer = context.composer.clone();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = match &result.abi_returns[0] {
        Ok(Some(abi_return)) => abi_return,
        Ok(None) => panic!("ABI return should be Some"),
        Err(e) => panic!("ABI return should be Ok: {:?}", e),
    };

    match &abi_return.return_value {
        ABIValue::Uint(returned_amount) => {
            assert_eq!(
                *returned_amount,
                BigUint::from(payment_amount),
                "Returned amount should match payment amount"
            );
        }
        _ => panic!("Return value should be a UInt"),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_get_returned_value_of_app_call_txn_app_call_method_call(
    #[future] setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address: sender_addr,
        app_id,
        arc56_contract,
        mut fixture,
    } = setup.await?;

    let receiver = fixture
        .generate_account(None)
        .await
        .expect("Failed to create receiver account");
    let receiver_addr = receiver
        .account()
        .expect("Failed to get receiver account")
        .address();

    let context = fixture.context()?;

    let get_pay_txn_amount_method = arc56_contract
        .methods
        .iter()
        .find(|m| m.name == "get_pay_txn_amount")
        .expect("Failed to find get_pay_txn_amount method")
        .try_into()
        .expect("Failed to convert ARC56 method to ABI method");

    let get_returned_value_of_app_call_txn_method = arc56_contract
        .methods
        .iter()
        .find(|m| m.name == "get_returned_value_of_app_call_txn")
        .expect("Failed to find get_returned_value_of_app_call_txn method")
        .try_into()
        .expect("Failed to convert ARC56 method to ABI method");

    let payment_amount = 2_500_000u64;
    let mut composer = context.composer.clone();

    let first_method_call_params = AppCallMethodCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        method: get_pay_txn_amount_method,
        args: vec![AppMethodCallArg::Payment(PaymentParams {
            common_params: CommonParams {
                sender: sender_addr.clone(),
                ..Default::default()
            },
            receiver: receiver_addr,
            amount: payment_amount,
        })],
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
    };

    let second_method_call_params = AppCallMethodCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        method: get_returned_value_of_app_call_txn_method,
        args: vec![AppMethodCallArg::AppCallMethodCall(
            first_method_call_params,
        )],
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
    };

    composer.add_app_call_method_call(second_method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = match &result.abi_returns[0] {
        Ok(Some(abi_return)) => abi_return,
        Ok(None) => panic!("ABI return should be Some"),
        Err(e) => panic!("ABI return should be Ok: {:?}", e),
    };

    match &abi_return.return_value {
        ABIValue::Uint(returned_amount) => {
            assert_eq!(
                *returned_amount,
                BigUint::from(payment_amount),
                "Returned amount should match payment amount"
            );
        }
        _ => panic!("Return value should be a UInt"),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_get_returned_value_of_nested_app_call_method_calls(
    #[future] setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address: sender_addr,
        app_id,
        arc56_contract,
        mut fixture,
    } = setup.await?;

    let receiver = fixture
        .generate_account(None)
        .await
        .expect("Failed to create receiver account");
    let receiver_addr = receiver
        .account()
        .expect("Failed to get receiver account")
        .address();

    let context = fixture.context()?;

    let get_pay_txn_amount_method = arc56_contract
        .methods
        .iter()
        .find(|m| m.name == "get_pay_txn_amount")
        .expect("Failed to find get_pay_txn_amount method")
        .try_into()
        .expect("Failed to convert ARC56 method to ABI method");

    let get_pay_txns_amount_sum_method = arc56_contract
        .methods
        .iter()
        .find(|m| m.name == "get_pay_txns_amount_sum")
        .expect("Failed to find get_pay_txns_amount_sum method")
        .try_into()
        .expect("Failed to convert ARC56 method to ABI method");

    let payment_amount = 5_000u64;
    let mut composer = context.composer.clone();

    let get_pay_txn_amount_method_call_params = AppCallMethodCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        method: get_pay_txn_amount_method,
        args: vec![AppMethodCallArg::Payment(PaymentParams {
            common_params: CommonParams {
                sender: sender_addr.clone(),
                ..Default::default()
            },
            receiver: receiver_addr.clone(),
            amount: payment_amount,
        })],
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
    };

    let get_pay_txns_amount_sum_method_method_call_params = AppCallMethodCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        method: get_pay_txns_amount_sum_method,
        args: vec![
            AppMethodCallArg::Payment(PaymentParams {
                common_params: CommonParams {
                    sender: sender_addr.clone(),
                    note: Some("second txn".as_bytes().to_vec()),
                    ..Default::default()
                },
                receiver: receiver_addr.clone(),
                amount: payment_amount,
            }),
            AppMethodCallArg::TransactionPlaceholder,
            AppMethodCallArg::AppCallMethodCall(get_pay_txn_amount_method_call_params),
        ],
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
    };

    composer.add_app_call_method_call(get_pay_txns_amount_sum_method_method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = match &result.abi_returns[1] {
        Ok(Some(abi_return)) => abi_return,
        Ok(None) => panic!("ABI return should be Some"),
        Err(e) => panic!("ABI return should be Ok: {:?}", e),
    };

    let expected_result = BigUint::from(15_000u64);
    match &abi_return.return_value {
        ABIValue::Uint(returned_amount) => {
            assert_eq!(
                *returned_amount, expected_result,
                "Returned amount should match payment amount"
            );
        }
        _ => panic!("Return value should be a UInt"),
    }

    Ok(())
}

struct TestData {
    sender_address: Address,
    app_id: u64,
    arc56_contract: Arc56Contract,
    fixture: AlgorandFixture,
}

type SetupResult = Result<TestData, Box<dyn std::error::Error + Send + Sync>>;
type TestResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[fixture]
async fn setup() -> SetupResult {
    init_test_logging();
    let mut fixture = algorand_fixture().await?;
    fixture.new_scope().await?;

    let context = fixture.context()?;
    let sender_address = context.test_account.account()?.address();

    let arc56_contract: Arc56Contract = serde_json::from_str(sandbox::APPLICATION_ARC56)?;
    let app_id = deploy_app(context, sender_address.clone(), arc56_contract.clone()).await;

    Ok(TestData {
        sender_address,
        app_id,
        arc56_contract,
        fixture,
    })
}

// Raw (non ABI) hello world approval program
const HELLO_WORLD_APPROVAL_PROGRAM: [u8; 18] = [
    10, 128, 7, 72, 101, 108, 108, 111, 44, 32, 54, 26, 0, 80, 176, 129, 1, 67,
];
// Raw (non ABI) hello world clear state program
const HELLO_WORLD_CLEAR_STATE_PROGRAM: [u8; 4] = [10, 129, 1, 67];

async fn create_test_app(context: &AlgorandTestContext, sender: Address) -> Option<u64> {
    let app_create_params = AppCreateParams {
        common_params: CommonParams {
            sender: sender.clone(),
            ..Default::default()
        },
        on_complete: OnApplicationComplete::NoOp,
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
        extra_program_pages: None,
        args: Some(vec![b"Create".to_vec()]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();

    composer
        .add_app_create(app_create_params)
        .expect("Failed to add app create");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send app create");

    result.confirmations[0].app_id
}

async fn deploy_app(
    context: &AlgorandTestContext,
    sender: Address,
    arc56_contract: Arc56Contract,
) -> u64 {
    let teal_source = arc56_contract.source.expect("No source found in app spec");

    let approval_bytes = BASE64_STANDARD
        .decode(teal_source.approval)
        .expect("Failed to decode approval program from base64");

    let clear_state_bytes = BASE64_STANDARD
        .decode(teal_source.clear)
        .expect("Failed to decode clear state program from base64");

    let approval_compile_result = context
        .algod
        .teal_compile(approval_bytes, None)
        .await
        .expect("Failed to compile approval program");
    let clear_state_compile_result = context
        .algod
        .teal_compile(clear_state_bytes, None)
        .await
        .expect("Failed to compile clear state program");

    let app_create_params = AppCreateParams {
        common_params: CommonParams {
            sender: sender.clone(),
            ..Default::default()
        },
        on_complete: OnApplicationComplete::NoOp,
        approval_program: approval_compile_result.result,
        clear_state_program: clear_state_compile_result.result,
        global_state_schema: None,
        local_state_schema: None,
        extra_program_pages: None,
        args: None,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_app_create(app_create_params)
        .expect("Failed to add app create");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send app create");

    result.confirmations[0].app_id.expect("No app ID returned")
}

#[rstest]
#[tokio::test]
async fn test_more_than_15_args_with_ref_types_app_call_method_call(
    #[future] setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address: sender_addr,
        app_id,
        arc56_contract,
        mut fixture,
    } = setup.await?;

    let receiver = fixture
        .generate_account(Some(TestAccountConfig {
            initial_funds: 0u64,
            ..Default::default()
        }))
        .await
        .expect("Failed to create receiver account");
    let receiver_addr = receiver
        .account()
        .expect("Failed to get receiver account")
        .address();

    let context = fixture.context()?;

    let asset_create_params = AssetCreateParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        total: 1_000_000,
        decimals: Some(2),
        default_frozen: Some(false),
        asset_name: Some("Test Asset".to_string()),
        unit_name: Some("TEST".to_string()),
        url: Some("https://example.com".to_string()),
        metadata_hash: None,
        manager: Some(sender_addr.clone()),
        reserve: Some(sender_addr.clone()),
        freeze: Some(sender_addr.clone()),
        clawback: Some(sender_addr.clone()),
    };

    let mut asset_composer = context.composer.clone();
    asset_composer
        .add_asset_create(asset_create_params)
        .expect("Failed to add asset create");

    let asset_result = asset_composer
        .send(None)
        .await
        .expect("Failed to send asset create");
    let asset_id = asset_result.confirmations[0]
        .asset_id
        .expect("No asset ID returned");

    let abi_method = arc56_contract
        .methods
        .iter()
        .find(|m| m.name == "more_than_15_args_with_ref_types")
        .expect("Failed to find method")
        .try_into()
        .expect("Failed to convert ARC56 method to ABI method");

    let tx_params = context
        .algod
        .transaction_params()
        .await
        .expect("Failed to get transaction params");

    let payment_amount = 200_000u64;
    let genesis_hash: Option<[u8; 32]> = tx_params.genesis_hash.try_into().ok();
    let payment_transaction = Transaction::Payment(PaymentTransactionFields {
        header: TransactionHeader {
            sender: sender_addr.clone(),
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
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        method: abi_method,
        args,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
    };

    let mut composer = context.composer.clone();
    composer
        .add_app_call_method_call(method_call_params)
        .expect("Failed to add method call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send transaction");

    let abi_return = match &result.abi_returns[0] {
        Ok(Some(abi_return)) => abi_return,
        Ok(None) => panic!("ABI return should be Some"),
        Err(e) => panic!("ABI return should be Ok: {:?}", e),
    };

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
                _ => panic!("First element should be asset ID"),
            }

            match &returned_tuple[1] {
                ABIValue::Uint(returned_app_id) => {
                    assert_eq!(
                        *returned_app_id,
                        BigUint::from(app_id),
                        "Returned app ID should match deployed app"
                    );
                }
                _ => panic!("Second element should be app ID"),
            }

            match &returned_tuple[2] {
                ABIValue::Uint(returned_balance) => {
                    assert_eq!(*returned_balance, BigUint::from(payment_amount),);
                }
                _ => panic!("Third element should be account balance"),
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
                            ABIValue::Byte(b) => *b,
                            _ => panic!("Transaction ID bytes should be ABIValue::Byte"),
                        })
                        .collect();

                    assert_eq!(
                        returned_txn_id, actual_txn_id,
                        "Returned transaction ID should match actual transaction ID"
                    );
                }
                _ => panic!("Fourth element should be transaction ID bytes"),
            }
        }
        _ => panic!("Return value should be a tuple"),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_more_than_15_args_app_call_method_call(#[future] setup: SetupResult) -> TestResult {
    let TestData {
        sender_address: sender_addr,
        app_id,
        arc56_contract,
        fixture,
    } = setup.await?;

    let context = fixture.context()?;

    let abi_method = arc56_contract
        .methods
        .iter()
        .find(|m| m.name == "more_than_15_args")
        .expect("Failed to find method")
        .try_into()
        .expect("Failed to convert ARC56 method to ABI method");

    let mut args = vec![];
    for i in 1..=18 {
        args.push(AppMethodCallArg::ABIValue(ABIValue::Uint(BigUint::from(
            i as u64,
        ))));
    }

    let method_call_params = AppCallMethodCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        method: abi_method,
        args,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: OnApplicationComplete::NoOp,
    };

    let mut composer = context.composer.clone();
    composer.add_app_call_method_call(method_call_params)?;

    let result = composer.send(None).await?;

    let abi_return = match &result.abi_returns[0] {
        Ok(Some(abi_return)) => abi_return,
        Ok(None) => panic!("ABI return should be Some"),
        Err(e) => panic!("ABI return should be Ok: {:?}", e),
    };

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
                    _ => panic!("Array element {} should be UInt64", i),
                }
            }
        }
        _ => panic!("Return value should be a dynamic array"),
    }

    Ok(())
}

#[tokio::test]
async fn test_app_call_validation_errors() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");
    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");
    let context = fixture.context().expect("Failed to get context");
    let sender_addr: Address = context
        .test_account
        .account()
        .expect("Failed to get sender address")
        .into();

    // Test app call with invalid app_id (0)
    let invalid_app_call_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id: 0, // Invalid: should be > 0 for app calls (0 is for app creation)
        on_complete: OnApplicationComplete::NoOp,
        args: None,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
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
}
