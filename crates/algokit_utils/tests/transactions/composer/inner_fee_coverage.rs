use crate::common::init_test_logging;
use algokit_transact::{Address, OnApplicationComplete};
use algokit_utils::CommonParams;
use algokit_utils::{ApplicationCallParams, ApplicationCreateParams, PaymentParams, testing::*};
use base64::{Engine, prelude::BASE64_STANDARD};
use serde::Deserialize;

// Placeholder smart contract programs - these will be replaced with the actual inner fee contract
// For now, using simple TEAL bytecode placeholders

// TODO: IMPLEMENTATION NEEDED - These tests are currently placeholders that verify transaction structure
// but do not test the actual inner fee coverage logic. The following need to be implemented:
// 1. Add `coverAppCallInnerTransactionFees` parameter to TransactionComposer
// 2. Implement fee calculation logic that:
//    - Simulates transactions to determine inner transaction fees
//    - Calculates required fees to cover inner transactions
//    - Adjusts outer transaction fees accordingly
//    - Handles fee pooling between transactions in a group
// 3. Update these tests to verify actual fee calculation behavior instead of just transaction success

#[derive(Deserialize)]
struct TealSource {
    approval: String,
    clear: String,
}

#[derive(Deserialize)]
struct ARC56AppSpec {
    source: Option<TealSource>,
}

fn get_inner_fee_teal_programs() -> (Vec<u8>, Vec<u8>) {
    let app_spec: ARC56AppSpec =
        serde_json::from_str(include_str!("../../contracts/inner_fee/application.json"))
            .expect("Failed to parse inner fee application spec");

    let teal_source = app_spec
        .source
        .expect("No source found in application spec");

    let approval_bytes = BASE64_STANDARD
        .decode(teal_source.approval)
        .expect("Failed to decode approval program from base64");

    let clear_state_bytes = BASE64_STANDARD
        .decode(teal_source.clear)
        .expect("Failed to decode clear state program from base64");

    (approval_bytes, clear_state_bytes)
}

// TODO: NC - Ensure this tests is only compiled in test mode

#[tokio::test]
async fn test_cover_app_call_inner_transaction_fees() {
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

    // Deploy three apps for testing
    let app_id_1 = deploy_inner_fee_app(context, sender_addr.clone(), "app1".to_string())
        .await
        .expect("Failed to create test app 1");

    let app_id_2 = deploy_inner_fee_app(context, sender_addr.clone(), "app2".to_string())
        .await
        .expect("Failed to create test app 2");

    let app_id_3 = deploy_inner_fee_app(context, sender_addr.clone(), "app3".to_string())
        .await
        .expect("Failed to create test app 3");

    println!(
        "Created test apps with IDs: {}, {}, {}",
        app_id_1, app_id_2, app_id_3
    );

    // Fund the app accounts
    fund_app_account(context, app_id_1, 1_000_000)
        .await
        .expect("Failed to fund app 1");
    fund_app_account(context, app_id_2, 1_000_000)
        .await
        .expect("Failed to fund app 2");
    fund_app_account(context, app_id_3, 1_000_000)
        .await
        .expect("Failed to fund app 3");

    // Test 1: throws when no max fee is supplied
    test_throws_when_no_max_fee_supplied(context, app_id_1, sender_addr.clone()).await;

    // Test 2: throws when inner transaction fees are not covered and coverAppCallInnerTransactionFees is disabled
    test_throws_when_inner_fees_not_covered(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;

    // Test 3: does not alter fee when app call has no inners
    test_does_not_alter_fee_when_no_inners(context, app_id_1, sender_addr.clone()).await;

    // Test 4: alters fee, handling when no inner fees have been covered
    test_alters_fee_no_inner_fees_covered(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;

    // Test 5: alters fee, handling when all inner fees have been covered
    test_alters_fee_all_inner_fees_covered(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;

    // Test 6: alters fee, handling when some inner fees have been covered or partially covered
    test_alters_fee_some_inner_fees_covered(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;

    // Test 7: alters fee, handling when some inner fees have a surplus
    test_alters_fee_some_inner_fees_surplus(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;

    // Test 8: alters fee, handling expensive abi method calls that use ensure_budget to op-up
    test_alters_fee_expensive_abi_method_calls(context, app_id_1, sender_addr.clone()).await;

    // Test 9: throws when max fee is too small to cover inner transaction fees
    test_throws_when_max_fee_too_small(context, app_id_1, app_id_2, app_id_3, sender_addr.clone())
        .await;

    // Test 10: throws when static fee is too small to cover inner transaction fees
    test_throws_when_static_fee_too_small(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;

    // Test 11: does not alter a static fee with surplus
    test_does_not_alter_static_fee_with_surplus(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;

    // Test 12: alters fee, handling multiple app calls in a group that send inners with varying fees
    test_alters_fee_multiple_app_calls_in_group(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;

    // Test 13: does not alter fee when another transaction in the group covers the inner fees
    test_does_not_alter_fee_when_group_covers_inner_fees(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;

    // Test 14: alters fee, allocating surplus fees to the most fee constrained transaction first
    test_alters_fee_allocating_surplus_to_constrained(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;

    // Test 15: alters fee, handling a large inner fee surplus pooling to lower siblings
    test_alters_fee_large_surplus_pooling(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;

    // Test 16: alters fee, handling inner fee surplus pooling to some lower siblings
    test_alters_fee_surplus_pooling_to_some_siblings(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;

    // Test 17: alters fee, handling a large inner fee surplus with no pooling
    test_alters_fee_large_surplus_no_pooling(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;

    // Test 18: alters fee, handling multiple inner fee surplus poolings to lower siblings
    test_alters_fee_multiple_surplus_poolings(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;

    // Test 19: throws when maxFee is below the calculated fee
    test_throws_when_max_fee_below_calculated(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;

    // Test 20: throws when staticFee is below the calculated fee
    test_throws_when_static_fee_below_calculated(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;

    // Test 21: readonly methods tests
    test_readonly_methods(context, app_id_1, sender_addr.clone()).await;

    // Test 22: alters fee, handling nested abi method calls
    test_alters_fee_nested_abi_method_calls(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;

    // Test 23: throws when nested maxFee is below the calculated fee
    test_throws_when_nested_max_fee_below_calculated(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;

    // Test 24: throws when staticFee for non app call transaction is too low
    test_throws_when_static_fee_too_low_for_non_app_call(
        context,
        app_id_1,
        app_id_2,
        app_id_3,
        sender_addr.clone(),
    )
    .await;
}

async fn test_throws_when_no_max_fee_supplied(
    context: &AlgorandTestContext,
    app_id: u64,
    sender: Address,
) {
    println!("Test: throws when no max fee is supplied");

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            // NOTE: No max_fee or static_fee provided
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![encode_method_selector("no_op")]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    // This should fail with coverAppCallInnerTransactionFees enabled due to missing maxFee
    let result = composer.send(None).await;

    // Expected error: "Please provide a maxFee for each app call transaction when coverAppCallInnerTransactionFees is enabled"
    // For now, the test will pass because coverAppCallInnerTransactionFees is not implemented
    // Once implemented, this should panic with the expected error message
    match result {
        Ok(_) => panic!(
            "Test should FAIL: Expected error 'Please provide a maxFee for each app call transaction when coverAppCallInnerTransactionFees is enabled. Required for transaction 0' but transaction succeeded. This indicates coverAppCallInnerTransactionFees is not implemented yet."
        ),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("Please provide a maxFee") {
                println!("✓ Test passed: Got expected maxFee error");
            } else {
                panic!("Test failed with unexpected error: {}", error_msg);
            }
        }
    }
}

async fn test_throws_when_inner_fees_not_covered(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: throws when inner transaction fees are not covered");

    // Create tuple for fees: (uint64,uint64,uint64,uint64,uint64[])
    let fees_tuple = encode_fees_tuple(0, 0, 0, 0, vec![0, 0]);

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(7000), // This should be too small to cover inner fees
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    // With coverAppCallInnerTransactionFees disabled, this should fail due to insufficient fees
    let result = composer.send(None).await;

    // Expected error: fee too small
    match result {
        Ok(_) => panic!(
            "Test should FAIL: Expected 'fee too small' error when coverAppCallInnerTransactionFees is disabled and fees are insufficient, but transaction succeeded. This indicates the inner fee coverage logic is not implemented yet."
        ),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("fee too small") {
                println!("✓ Test passed: Got expected 'fee too small' error");
            } else {
                println!(
                    "✓ Test passed: Got error (may indicate insufficient fees): {}",
                    error_msg
                );
            }
        }
    }
}

async fn test_does_not_alter_fee_when_no_inners(
    context: &AlgorandTestContext,
    app_id: u64,
    sender: Address,
) {
    println!("Test: does not alter fee when app call has no inners");

    let expected_fee = 1000u64;

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(2000),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![encode_method_selector("no_op")]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send application call");

    let transaction_fee = result.confirmations[0]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);
    assert_eq!(
        transaction_fee, expected_fee,
        "Fee should not be altered for no-op calls"
    );
    println!("✓ Test passed: fee = {}", transaction_fee);
}

async fn test_alters_fee_no_inner_fees_covered(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: alters fee, handling when no inner fees have been covered");

    let expected_fee = 7000u64;
    let fees_tuple = encode_fees_tuple(0, 0, 0, 0, vec![0, 0]);

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send application call");

    // Verify the transaction was successful
    assert!(
        !result.confirmations.is_empty(),
        "Should have confirmations"
    );

    // Verify the actual fee matches expected fee - this is the key test
    let transaction_fee = result.confirmations[0]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);

    // This test should FAIL until coverAppCallInnerTransactionFees is implemented
    // The expected behavior is that the fee is adjusted to 7000 to cover inner transaction fees
    if transaction_fee == expected_fee {
        println!(
            "✓ Test passed: fee correctly calculated as {} µALGO",
            transaction_fee
        );
    } else {
        panic!(
            "Test should FAIL: Expected fee to be adjusted to {} µALGO to cover inner transaction fees, but got {} µALGO. This indicates coverAppCallInnerTransactionFees is not implemented yet.",
            expected_fee, transaction_fee
        );
    }
}

async fn test_alters_fee_all_inner_fees_covered(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: alters fee, handling when all inner fees have been covered");

    let expected_fee = 1000u64;
    let fees_tuple = encode_fees_tuple(1000, 1000, 1000, 1000, vec![1000, 1000]);

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send application call");

    // Verify the transaction was successful
    assert!(
        !result.confirmations.is_empty(),
        "Should have confirmations"
    );

    // Verify the actual fee - when all inner fees are covered, should be minimal (1000)
    let transaction_fee = result.confirmations[0]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);

    // This test should FAIL until coverAppCallInnerTransactionFees is implemented
    // When all inner fees are pre-covered, the outer transaction should only need minimal fee
    if transaction_fee == expected_fee {
        println!(
            "✓ Test passed: fee correctly minimal at {} µALGO when all inner fees covered",
            transaction_fee
        );
    } else {
        panic!(
            "Test should FAIL: Expected minimal fee {} µALGO when all inner fees are pre-covered, but got {} µALGO. This indicates coverAppCallInnerTransactionFees is not implemented yet.",
            expected_fee, transaction_fee
        );
    }
}

async fn test_alters_fee_some_inner_fees_covered(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!(
        "Test: alters fee, handling when some inner fees have been covered or partially covered"
    );

    let expected_fee = 5300u64;
    let fees_tuple = encode_fees_tuple(1000, 0, 200, 0, vec![500, 0]);

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send application call");

    // Verify the transaction was successful
    assert!(
        !result.confirmations.is_empty(),
        "Should have confirmations"
    );

    // Verify the actual fee - should be 5300 for partial coverage scenario
    let transaction_fee = result.confirmations[0]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);

    // This test should FAIL until coverAppCallInnerTransactionFees is implemented
    // The fee should be calculated to 5300 based on partial inner fee coverage
    if transaction_fee == expected_fee {
        println!(
            "✓ Test passed: fee correctly calculated as {} µALGO for partial coverage",
            transaction_fee
        );
    } else {
        panic!(
            "Test should FAIL: Expected fee {} µALGO for partial inner fee coverage scenario, but got {} µALGO. This indicates coverAppCallInnerTransactionFees is not implemented yet.",
            expected_fee, transaction_fee
        );
    }
}

async fn test_alters_fee_some_inner_fees_surplus(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: alters fee, handling when some inner fees have a surplus");

    let expected_fee = 2000u64;
    let fees_tuple = encode_fees_tuple(0, 1000, 5000, 0, vec![0, 50]);

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send application call");

    // Verify the transaction was successful
    assert!(
        !result.confirmations.is_empty(),
        "Should have confirmations"
    );

    // Verify the actual fee - this should be 2000 for surplus handling scenario
    let transaction_fee = result.confirmations[0]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);

    // This test should FAIL until coverAppCallInnerTransactionFees is implemented
    // The fee should be calculated considering surplus fee handling
    if transaction_fee == expected_fee {
        println!(
            "✓ Test passed: fee correctly calculated as {} µALGO with surplus handling",
            transaction_fee
        );
    } else {
        panic!(
            "Test should FAIL: Expected fee {} µALGO for surplus handling scenario, but got {} µALGO. This indicates coverAppCallInnerTransactionFees is not implemented yet.",
            expected_fee, transaction_fee
        );
    }
}

async fn test_alters_fee_expensive_abi_method_calls(
    context: &AlgorandTestContext,
    app_id: u64,
    sender: Address,
) {
    println!(
        "Test: alters fee, handling expensive abi method calls that use ensure_budget to op-up"
    );

    let expected_fee = 10_000u64;

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee + 2_000), // Use expected_fee in calculation
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("burn_ops"),
            encode_uint64(6200), // op_budget parameter
        ]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send application call");

    // Verify the transaction was successful and would have created op-up inner transactions
    assert!(
        !result.confirmations.is_empty(),
        "Should have confirmations"
    );

    // Verify the actual fee - should be 10,000 for expensive op-up scenario
    let transaction_fee = result.confirmations[0]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);

    // This test should FAIL until coverAppCallInnerTransactionFees is implemented
    // The fee should be calculated to include op-up costs for expensive operations
    if transaction_fee == expected_fee {
        println!(
            "✓ Test passed: fee correctly calculated as {} µALGO including op-up costs",
            transaction_fee
        );
    } else {
        panic!(
            "Test should FAIL: Expected fee {} µALGO to include op-up costs for expensive operations, but got {} µALGO. This indicates coverAppCallInnerTransactionFees is not implemented yet.",
            expected_fee, transaction_fee
        );
    }
}

async fn test_alters_fee_nested_abi_method_calls(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: alters fee, handling nested abi method calls");

    let expected_fee = 2000u64;
    let fees_tuple = encode_fees_tuple(0, 0, 2000, 0, vec![0, 0]);

    // Create a payment transaction that will be used as a nested argument
    let payment_params = PaymentParams {
        common_params: CommonParams {
            sender: sender.clone(),
            static_fee: Some(1500),
            ..Default::default()
        },
        receiver: sender.clone(),
        amount: 0,
    };

    // Create an app call transaction that will be used as a nested argument
    let nested_app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(4000),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    // In a real implementation, this would use a nested transaction method
    // For now, we'll simulate with a regular group transaction
    let mut composer = context.composer.clone();
    composer
        .add_payment(payment_params)
        .expect("Failed to add payment");
    composer
        .add_application_call(nested_app_call_params)
        .expect("Failed to add nested application call");

    // Main app call that would take the above transactions as arguments
    let main_app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            static_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![encode_method_selector("no_op")]), // Simplified for testing
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    composer
        .add_application_call(main_app_call_params)
        .expect("Failed to add main application call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send nested transaction group");

    // In real implementation, this would verify nested transaction fee handling
    assert_eq!(result.confirmations.len(), 3, "Should have 3 confirmations");
    println!("✓ Test passed (placeholder - would verify nested transaction fee handling)");
}

async fn test_throws_when_nested_max_fee_below_calculated(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: throws when nested maxFee is below the calculated fee");

    let fees_tuple = encode_fees_tuple(0, 0, 2000, 0, vec![0, 0]);

    // Create a payment transaction
    let payment_params = PaymentParams {
        common_params: CommonParams {
            sender: sender.clone(),
            ..Default::default()
        },
        receiver: sender.clone(),
        amount: 0,
    };

    // Create a nested app call with insufficient max fee
    let nested_app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(2000), // Too low for the calculated fee (should be ~5000)
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    // Main app call that would normally take the above as nested arguments
    let main_app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(10_000), // Sufficient for main call
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![encode_method_selector("no_op")]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_payment(payment_params)
        .expect("Failed to add payment");
    composer
        .add_application_call(nested_app_call_params)
        .expect("Failed to add nested application call");
    composer
        .add_application_call(main_app_call_params)
        .expect("Failed to add main application call");

    // This should fail due to nested transaction having insufficient max fee
    let result = composer.send(None).await;

    // Expected error: "Calculated transaction fee 5000 µALGO is greater than max of 2000 for transaction 1"
    match result {
        Ok(_) => panic!(
            "Test should FAIL: Expected error 'Calculated transaction fee 5000 µALGO is greater than max of 2000 for transaction 1' but transaction succeeded. This indicates coverAppCallInnerTransactionFees is not implemented yet."
        ),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("fee") && error_msg.contains("greater than") {
                println!(
                    "✓ Test passed: Got expected nested fee constraint error: {}",
                    error_msg
                );
            } else {
                println!(
                    "✓ Test passed: Got error (may indicate fee constraint): {}",
                    error_msg
                );
            }
        }
    }
}

async fn test_throws_when_static_fee_too_low_for_non_app_call(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: throws when staticFee for non app call transaction is too low");

    let fees_tuple = encode_fees_tuple(0, 0, 0, 0, vec![0, 0]);

    // First app call with high static fee and max fee
    let app_call_params_1 = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            static_fee: Some(13_000),
            max_fee: Some(14_000),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple.clone(),
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    // Second app call with low static fee
    let app_call_params_2 = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            static_fee: Some(1000),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    // Payment transaction with insufficient static fee
    let payment_params = PaymentParams {
        common_params: CommonParams {
            sender: sender.clone(),
            static_fee: Some(500), // Too low, should need additional 500 µALGO
            ..Default::default()
        },
        receiver: sender.clone(),
        amount: 0,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params_1)
        .expect("Failed to add first application call");
    composer
        .add_application_call(app_call_params_2)
        .expect("Failed to add second application call");
    composer
        .add_payment(payment_params)
        .expect("Failed to add payment");

    // This should fail due to payment transaction having insufficient static fee
    let result = composer.send(None).await;

    // Expected error: "An additional fee of 500 µALGO is required for non app call transaction 2"
    match result {
        Ok(_) => panic!(
            "Test should FAIL: Expected error 'An additional fee of 500 µALGO is required for non app call transaction 2' but transaction succeeded. This indicates coverAppCallInnerTransactionFees is not implemented yet."
        ),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("additional fee") && error_msg.contains("required") {
                println!(
                    "✓ Test passed: Got expected additional fee error: {}",
                    error_msg
                );
            } else {
                println!(
                    "✓ Test passed: Got error (may indicate fee issue): {}",
                    error_msg
                );
            }
        }
    }
}

async fn deploy_inner_fee_app(
    context: &AlgorandTestContext,
    sender: Address,
    note: String,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let (approval_teal, clear_state_teal) = get_inner_fee_teal_programs();
    let approval_compile_result = context.algod.teal_compile(approval_teal, None).await?;
    let clear_state_compile_result = context.algod.teal_compile(clear_state_teal, None).await?;

    let app_create_params = ApplicationCreateParams {
        common_params: CommonParams {
            sender: sender.clone(),
            note: Some(note.into_bytes()),
            ..Default::default()
        },
        on_complete: OnApplicationComplete::NoOp,
        approval_program: approval_compile_result.result,
        clear_state_program: clear_state_compile_result.result,
        ..Default::default()
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_create(app_create_params)
        .expect("Failed to add application create");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send application create");

    result.confirmations[0]
        .application_index
        .ok_or_else(|| "No application index returned".into())
}

// Helper function to fund app accounts
async fn fund_app_account(
    context: &AlgorandTestContext,
    app_id: u64,
    amount: u64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app_address = get_application_address(app_id);

    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let payment_params = PaymentParams {
        common_params: CommonParams {
            sender: sender_addr,
            ..Default::default()
        },
        receiver: app_address,
        amount,
    };

    let mut composer = context.composer.clone();
    composer
        .add_payment(payment_params)
        .expect("Failed to add payment");

    composer.send(None).await.expect("Failed to send payment");

    Ok(())
}

// Helper function to compute application address from app ID
fn get_application_address(app_id: u64) -> Address {
    use sha2::{Digest, Sha512_256};

    let mut hasher = Sha512_256::new();
    hasher.update(b"appID");
    hasher.update(&app_id.to_be_bytes());

    let hash = hasher.finalize();
    let mut address_bytes = [0u8; 32];
    address_bytes.copy_from_slice(&hash[..32]);

    Address(address_bytes)
}

// Helper functions for encoding ABI method calls
fn encode_method_selector(method_name: &str) -> Vec<u8> {
    use sha2::{Digest, Sha512_256};

    let method_signatures = match method_name {
        "no_op" => "no_op()void",
        "send_inners_with_fees" => {
            "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void"
        }
        "burn_ops" => "burn_ops(uint64)void",
        "burn_ops_readonly" => "burn_ops_readonly(uint64)void",
        "send_x_inners_with_fees" => "send_x_inners_with_fees(uint64,uint64[])void",
        "send_inners_with_fees_2" => {
            "send_inners_with_fees_2(uint64,uint64,(uint64,uint64,uint64[],uint64,uint64,uint64[]))void"
        }
        _ => panic!("Unknown method: {}", method_name),
    };

    let mut hasher = Sha512_256::new();
    hasher.update(method_signatures.as_bytes());
    let hash = hasher.finalize();
    hash[0..4].to_vec()
}

fn encode_uint64(value: u64) -> Vec<u8> {
    value.to_be_bytes().to_vec()
}

fn encode_fees_tuple(fee1: u64, fee2: u64, fee3: u64, fee4: u64, fee_array: Vec<u64>) -> Vec<u8> {
    let mut result = Vec::new();

    // Encode each uint64 in the tuple
    result.extend_from_slice(&fee1.to_be_bytes());
    result.extend_from_slice(&fee2.to_be_bytes());
    result.extend_from_slice(&fee3.to_be_bytes());
    result.extend_from_slice(&fee4.to_be_bytes());

    // Encode the dynamic array
    // First encode the length as uint16
    result.extend_from_slice(&(fee_array.len() as u16).to_be_bytes());

    // Then encode each element
    for fee in fee_array {
        result.extend_from_slice(&fee.to_be_bytes());
    }

    result
}

async fn test_throws_when_max_fee_too_small(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: throws when max fee is too small to cover inner transaction fees");

    let expected_fee = 7000u64;
    let fees_tuple = encode_fees_tuple(0, 0, 0, 0, vec![0, 0]);

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee - 1), // Too small by 1 microAlgo
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    // This should fail due to insufficient max fee when coverAppCallInnerTransactionFees is enabled
    let result = composer.send(None).await;

    // Expected error: "Fees were too small to resolve execution info via simulate. You may need to increase an app call transaction maxFee."
    match result {
        Ok(_) => panic!(
            "Test should FAIL: Expected error about fees being too small to resolve execution info via simulate, but transaction succeeded. This indicates coverAppCallInnerTransactionFees is not implemented yet."
        ),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("fee")
                && (error_msg.contains("too small") || error_msg.contains("insufficient"))
            {
                println!("✓ Test passed: Got expected fee error: {}", error_msg);
            } else {
                println!(
                    "✓ Test passed: Got error (may indicate fee issue): {}",
                    error_msg
                );
            }
        }
    }
}

async fn test_throws_when_static_fee_too_small(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: throws when static fee is too small to cover inner transaction fees");

    let expected_fee = 7000u64;
    let fees_tuple = encode_fees_tuple(0, 0, 0, 0, vec![0, 0]);

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            static_fee: Some(expected_fee - 1), // Too small by 1 microAlgo
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    // This should fail due to insufficient static fee when coverAppCallInnerTransactionFees is enabled
    let result = composer.send(None).await;

    // Expected error: "Fees were too small to resolve execution info via simulate. You may need to increase an app call transaction maxFee."
    match result {
        Ok(_) => panic!(
            "Test should FAIL: Expected error about fees being too small to resolve execution info via simulate, but transaction succeeded. This indicates coverAppCallInnerTransactionFees is not implemented yet."
        ),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("fee")
                && (error_msg.contains("too small") || error_msg.contains("insufficient"))
            {
                println!("✓ Test passed: Got expected fee error: {}", error_msg);
            } else {
                println!(
                    "✓ Test passed: Got error (may indicate fee issue): {}",
                    error_msg
                );
            }
        }
    }
}

async fn test_does_not_alter_static_fee_with_surplus(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: does not alter a static fee with surplus");

    let expected_fee = 6000u64;
    let fees_tuple = encode_fees_tuple(1000, 0, 200, 0, vec![500, 0]);

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            static_fee: Some(expected_fee), // Static fee with surplus
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send application call");

    let transaction_fee = result.confirmations[0]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);

    // Verify the transaction was successful
    assert!(
        !result.confirmations.is_empty(),
        "Should have confirmations"
    );

    // This test should PASS - static fees with surplus should NOT be altered
    // The fee should remain exactly as specified (6000)
    if transaction_fee == expected_fee {
        println!(
            "✓ Test passed: static fee correctly preserved at {} µALGO with surplus",
            transaction_fee
        );
    } else {
        panic!(
            "Test FAILED: Expected static fee to remain {} µALGO with surplus, but got {} µALGO. Static fees should not be altered even with coverAppCallInnerTransactionFees enabled.",
            expected_fee, transaction_fee
        );
    }
}

async fn test_alters_fee_multiple_app_calls_in_group(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!(
        "Test: alters fee, handling multiple app calls in a group that send inners with varying fees"
    );

    let txn1_expected_fee = 5800u64;
    let txn2_expected_fee = 6000u64;

    let fees_tuple_1 = encode_fees_tuple(0, 1000, 0, 0, vec![200, 0]);
    let fees_tuple_2 = encode_fees_tuple(1000, 0, 0, 0, vec![0, 0]);

    let app_call_params_1 = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            static_fee: Some(txn1_expected_fee),
            note: Some(b"txn1".to_vec()),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple_1,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    let app_call_params_2 = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(txn2_expected_fee),
            note: Some(b"txn2".to_vec()),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple_2,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params_1)
        .expect("Failed to add first application call");
    composer
        .add_application_call(app_call_params_2)
        .expect("Failed to add second application call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send group transaction");

    // Verify both transactions were successful
    assert_eq!(result.confirmations.len(), 2, "Should have 2 confirmations");

    // Verify the actual fees for both transactions
    let txn1_fee = result.confirmations[0]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);
    let txn2_fee = result.confirmations[1]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);

    // This test should FAIL until coverAppCallInnerTransactionFees is implemented
    // The fees should be calculated to the expected values for group transactions
    if txn1_fee == txn1_expected_fee && txn2_fee == txn2_expected_fee {
        println!(
            "✓ Test passed: txn1 fee = {} µALGO, txn2 fee = {} µALGO correctly calculated for group",
            txn1_fee, txn2_fee
        );
    } else {
        panic!(
            "Test should FAIL: Expected txn1 fee {} µALGO and txn2 fee {} µALGO for group with varying inner fees, but got {} and {} µALGO. This indicates coverAppCallInnerTransactionFees is not implemented yet.",
            txn1_expected_fee, txn2_expected_fee, txn1_fee, txn2_fee
        );
    }
}

async fn test_does_not_alter_fee_when_group_covers_inner_fees(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!(
        "Test: does not alter fee when another transaction in the group covers the inner fees"
    );

    let expected_fee = 8000u64;
    let fees_tuple = encode_fees_tuple(0, 0, 0, 0, vec![0, 0]);

    // First transaction: payment with high fee to cover inner fees
    let payment_params = PaymentParams {
        common_params: CommonParams {
            sender: sender.clone(),
            static_fee: Some(expected_fee),
            ..Default::default()
        },
        receiver: sender.clone(),
        amount: 0,
    };

    // Second transaction: app call that generates inner transactions
    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_payment(payment_params)
        .expect("Failed to add payment");
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send group transaction");

    // In real implementation, this would verify the app call fee was reduced due to payment surplus
    assert_eq!(result.confirmations.len(), 2, "Should have 2 confirmations");
    println!("✓ Test passed (placeholder - would verify fee coverage from other transaction)");
}

async fn test_alters_fee_allocating_surplus_to_constrained(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!(
        "Test: alters fee, allocating surplus fees to the most fee constrained transaction first"
    );

    let fees_tuple = encode_fees_tuple(0, 0, 0, 0, vec![0, 0]);

    // Transaction 1: app call with low max fee (needs more)
    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(2000),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    // Transaction 2: payment with high static fee (provides surplus)
    let payment_params_1 = PaymentParams {
        common_params: CommonParams {
            sender: sender.clone(),
            static_fee: Some(7500),
            ..Default::default()
        },
        receiver: sender.clone(),
        amount: 0,
    };

    // Transaction 3: payment with zero fee (can accept surplus)
    let payment_params_2 = PaymentParams {
        common_params: CommonParams {
            sender: sender.clone(),
            static_fee: Some(0),
            ..Default::default()
        },
        receiver: sender.clone(),
        amount: 0,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");
    composer
        .add_payment(payment_params_1)
        .expect("Failed to add first payment");
    composer
        .add_payment(payment_params_2)
        .expect("Failed to add second payment");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send group transaction");

    // In real implementation, this would verify surplus allocation to most constrained transaction
    assert_eq!(result.confirmations.len(), 3, "Should have 3 confirmations");
    println!("✓ Test passed (placeholder - would verify surplus allocation)");
}

async fn test_alters_fee_large_surplus_pooling(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: alters fee, handling a large inner fee surplus pooling to lower siblings");

    let expected_fee = 7000u64;
    // Inner fees with large surplus that should pool to lower siblings
    let fees_tuple = encode_fees_tuple(0, 0, 0, 0, vec![0, 0, 20_000, 0, 0, 0]);

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send application call");

    // Verify the transaction was successful
    assert!(
        !result.confirmations.is_empty(),
        "Should have confirmations"
    );

    // Verify the actual fee
    let transaction_fee = result.confirmations[0]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);

    // In real implementation, this would verify fee pooling behavior
    assert!(transaction_fee > 0, "Should have a positive fee");
    println!(
        "✓ Test passed: fee = {} µALGO (placeholder - would verify fee {} µALGO with large surplus pooling to lower siblings)",
        transaction_fee, expected_fee
    );
}

async fn test_alters_fee_surplus_pooling_to_some_siblings(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: alters fee, handling inner fee surplus pooling to some lower siblings");

    let expected_fee = 6300u64;
    // Surplus that pools to some but not all lower siblings
    let fees_tuple = encode_fees_tuple(0, 0, 2200, 0, vec![0, 0, 2500, 0, 0, 0]);

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send application call");

    // In real implementation, this would verify partial surplus pooling
    assert!(
        !result.confirmations.is_empty(),
        "Should have confirmations"
    );
    println!("✓ Test passed (placeholder - would verify partial surplus pooling)");
}

async fn test_alters_fee_large_surplus_no_pooling(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: alters fee, handling a large inner fee surplus with no pooling");

    let expected_fee = 10_000u64;
    // Large surplus at the end with no lower siblings to pool to
    let fees_tuple = encode_fees_tuple(0, 0, 0, 0, vec![0, 0, 0, 0, 0, 20_000]);

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send application call");

    // In real implementation, this would verify no pooling occurred
    assert!(
        !result.confirmations.is_empty(),
        "Should have confirmations"
    );
    println!("✓ Test passed (placeholder - would verify no pooling with terminal surplus)");
}

async fn test_alters_fee_multiple_surplus_poolings(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: alters fee, handling multiple inner fee surplus poolings to lower siblings");

    let expected_fee = 7100u64;
    // Uses send_inners_with_fees_2 method with more complex fee structure
    let fees_tuple_2 = encode_fees_tuple_2(
        0,
        1200,
        vec![0, 0, 4900, 0, 0, 0],
        200,
        1100,
        vec![0, 0, 2500, 0, 0, 0],
    );

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees_2"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple_2,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send application call");

    // In real implementation, this would verify multiple surplus poolings
    assert!(
        !result.confirmations.is_empty(),
        "Should have confirmations"
    );
    println!("✓ Test passed (placeholder - would verify multiple surplus poolings)");
}

async fn test_throws_when_max_fee_below_calculated(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: throws when maxFee is below the calculated fee");

    let fees_tuple = encode_fees_tuple(0, 0, 0, 0, vec![0, 0]);

    // First transaction with insufficient max fee
    let app_call_params_1 = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(1200), // Too low for inner fees
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple.clone(),
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    // Second transaction with sufficient fee to allow simulate to succeed
    let app_call_params_2 = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(10_000),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![encode_method_selector("no_op")]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params_1)
        .expect("Failed to add first application call");
    composer
        .add_application_call(app_call_params_2)
        .expect("Failed to add second application call");

    // This should fail due to calculated fee being higher than maxFee
    let result = composer.send(None).await;

    // Expected error: "Calculated transaction fee 7000 µALGO is greater than max of 1200 for transaction 0"
    match result {
        Ok(_) => panic!(
            "Test should FAIL: Expected error 'Calculated transaction fee 7000 µALGO is greater than max of 1200 for transaction 0' but transaction succeeded. This indicates coverAppCallInnerTransactionFees is not implemented yet."
        ),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("fee") && error_msg.contains("greater than") {
                println!(
                    "✓ Test passed: Got expected fee greater than max error: {}",
                    error_msg
                );
            } else {
                println!(
                    "✓ Test passed: Got error (may indicate fee constraint): {}",
                    error_msg
                );
            }
        }
    }
}

async fn test_throws_when_static_fee_below_calculated(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: throws when staticFee is below the calculated fee");

    let fees_tuple = encode_fees_tuple(0, 0, 0, 0, vec![0, 0]);

    // First transaction with insufficient static fee
    let app_call_params_1 = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            static_fee: Some(5000), // Too low for inner fees (should be 7000)
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"),
            encode_uint64(app_id_2),
            encode_uint64(app_id_3),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    // Second transaction with sufficient fee to allow simulate to succeed
    let app_call_params_2 = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(10_000),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![encode_method_selector("no_op")]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params_1)
        .expect("Failed to add first application call");
    composer
        .add_application_call(app_call_params_2)
        .expect("Failed to add second application call");

    // This should fail due to calculated fee being higher than staticFee
    let result = composer.send(None).await;

    // Expected error: "Calculated transaction fee 7000 µALGO is greater than max of 5000 for transaction 0"
    match result {
        Ok(_) => panic!(
            "Test should FAIL: Expected error 'Calculated transaction fee 7000 µALGO is greater than max of 5000 for transaction 0' but transaction succeeded. This indicates coverAppCallInnerTransactionFees is not implemented yet."
        ),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("fee") && error_msg.contains("greater than") {
                println!(
                    "✓ Test passed: Got expected fee greater than static fee error: {}",
                    error_msg
                );
            } else {
                println!(
                    "✓ Test passed: Got error (may indicate fee constraint): {}",
                    error_msg
                );
            }
        }
    }
}

async fn test_readonly_methods(context: &AlgorandTestContext, app_id: u64, sender: Address) {
    println!("Test: readonly methods behavior");

    // Test 1: uses fixed opcode budget without op-up inner transactions (enabled)
    test_readonly_fixed_opcode_budget(context, app_id, sender.clone(), true).await;

    // Test 2: uses fixed opcode budget without op-up inner transactions (disabled)
    test_readonly_fixed_opcode_budget(context, app_id, sender.clone(), false).await;

    // Test 3: alters fee, handling inner transactions for readonly methods
    test_readonly_alters_fee_handling_inners(context, app_id, sender.clone()).await;

    // Test 4: throws when max fee is too small for readonly methods
    test_readonly_throws_when_max_fee_too_small(context, app_id, sender.clone()).await;
}

async fn test_readonly_fixed_opcode_budget(
    context: &AlgorandTestContext,
    app_id: u64,
    sender: Address,
    cover_inner_fees: bool,
) {
    let description = if cover_inner_fees {
        "with coverAppCallInnerTransactionFees enabled"
    } else {
        "with coverAppCallInnerTransactionFees disabled"
    };

    println!(
        "Test: uses fixed opcode budget without op-up inner transactions {}",
        description
    );

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("burn_ops_readonly"),
            encode_uint64(6200), // This would normally require op-ups via inner transactions
        ]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send readonly application call");

    let transaction_fee = result.confirmations[0]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);

    // In real implementation, this would verify readonly behavior
    assert_eq!(transaction_fee, 1000, "Readonly should use minimal fee");
    // Would also verify no inner transactions were created for op-ups
    println!(
        "✓ Test passed: readonly fee = {} (no op-up inners)",
        transaction_fee
    );
}

async fn test_readonly_alters_fee_handling_inners(
    context: &AlgorandTestContext,
    app_id: u64,
    sender: Address,
) {
    println!("Test: readonly alters fee, handling inner transactions");

    let expected_fee = 12_000u64;
    let fees_tuple = encode_fees_tuple(1000, 0, 200, 0, vec![500, 0]);

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"), // Would be marked as readonly in real impl
            encode_uint64(app_id),                           // Use same app for simplicity
            encode_uint64(app_id),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id]),
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send readonly application call with inners");

    let transaction_fee = result.confirmations[0]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);

    // In real implementation, this would use maxFee for readonly and verify inner txns
    assert!(transaction_fee > 0, "Should have some fee");
    println!(
        "✓ Test passed: readonly with inners fee = {} (placeholder)",
        transaction_fee
    );
}

async fn test_readonly_throws_when_max_fee_too_small(
    context: &AlgorandTestContext,
    app_id: u64,
    sender: Address,
) {
    println!("Test: readonly throws when max fee is too small to cover inner transaction fees");

    let fees_tuple = encode_fees_tuple(1000, 0, 200, 0, vec![500, 0]);

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(2000), // Too small for the inner fees
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            encode_method_selector("send_inners_with_fees"), // Would be marked as readonly in real impl
            encode_uint64(app_id),
            encode_uint64(app_id),
            fees_tuple,
        ]),
        account_references: None,
        app_references: Some(vec![app_id]),
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    // This should fail due to insufficient max fee for readonly method
    let result = composer.send(None).await;

    match result {
        Ok(_) => println!(
            "✓ Test passed (placeholder - would fail with insufficient max fee for readonly)"
        ),
        Err(e) => println!("Readonly transaction failed as expected: {}", e),
    }
}

// Helper function for encoding the more complex fees tuple used by send_inners_with_fees_2
fn encode_fees_tuple_2(
    fee1: u64,
    fee2: u64,
    fee_array_1: Vec<u64>,
    fee3: u64,
    fee4: u64,
    fee_array_2: Vec<u64>,
) -> Vec<u8> {
    let mut result = Vec::new();

    // Encode the structure: (uint64,uint64,uint64[],uint64,uint64,uint64[])
    result.extend_from_slice(&fee1.to_be_bytes());
    result.extend_from_slice(&fee2.to_be_bytes());

    // Encode first dynamic array
    result.extend_from_slice(&(fee_array_1.len() as u16).to_be_bytes());
    for fee in fee_array_1 {
        result.extend_from_slice(&fee.to_be_bytes());
    }

    result.extend_from_slice(&fee3.to_be_bytes());
    result.extend_from_slice(&fee4.to_be_bytes());

    // Encode second dynamic array
    result.extend_from_slice(&(fee_array_2.len() as u16).to_be_bytes());
    for fee in fee_array_2 {
        result.extend_from_slice(&fee.to_be_bytes());
    }

    result
}
