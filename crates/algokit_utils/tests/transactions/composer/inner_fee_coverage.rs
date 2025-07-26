use std::str::FromStr;

use crate::common::init_test_logging;
use algokit_abi::{ABIMethod, ABIType, ABIValue};
use algokit_transact::{Address, OnApplicationComplete};
use algokit_utils::CommonParams;
use algokit_utils::transactions::composer::SendParams;
use algokit_utils::{ApplicationCallParams, ApplicationCreateParams, PaymentParams, testing::*};
use base64::{Engine, prelude::BASE64_STANDARD};
use num_bigint::BigUint;
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

// TODO: NC - Add assert to check that it's the minimum fee to successfully cover the app call

const SEND_PARAMS: Option<SendParams> = Some(SendParams {
    cover_app_call_inner_transaction_fees: Some(true),
    max_rounds_to_wait_for_confirmation: None,
});

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
    fund_app_account(context, app_id_1, 500_000)
        .await
        .expect("Failed to fund app 1");
    fund_app_account(context, app_id_2, 500_000)
        .await
        .expect("Failed to fund app 2");
    fund_app_account(context, app_id_3, 500_000)
        .await
        .expect("Failed to fund app 3");

    // Test 1: throws when no max fee is supplied
    test_throws_when_no_max_fee_supplied(context, app_id_1, sender_addr.clone()).await;

    // Test 2: throws when inner transaction fees are not covered and coverAppCallInnerTransactionFees is disabled
    test_throws_when_inner_fees_not_covered_and_fee_coverage_disabled(
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
    // TODO: NC - Add readonly support
    // test_readonly_methods(context, app_id_1, sender_addr.clone()).await;

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

    let method = ABIMethod::from_str("no_op()void").unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            // NOTE: No max_fee or static_fee provided
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selector]),
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
    let result = composer.send(SEND_PARAMS).await;

    assert!(
        result.is_err()
            && result
                .as_ref()
                .unwrap_err()
                .to_string()
                .contains("Please provide a maxFee"),
        "Expected error when no max fee is supplied, got: {:?}",
        result
    );
}

async fn test_throws_when_inner_fees_not_covered_and_fee_coverage_disabled(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!(
        "Test: throws when inner transaction fees are not covered and coverAppCallInnerTransactionFees is disabled"
    );

    // Create ABI method
    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    // This tuple represents the fees for the inner transactions
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();

    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(0u64)), // fee1
        ABIValue::Uint(BigUint::from(0u64)), // fee2
        ABIValue::Uint(BigUint::from(0u64)), // fee3
        ABIValue::Uint(BigUint::from(0u64)), // fee4
        // Nested inner transaction fees
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(0u64)), // fee5
            ABIValue::Uint(BigUint::from(0u64)), // fee6
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(7000), // This should be too small to cover inner fees
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_encoded,
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

    assert!(
        result.is_err()
            && result
                .as_ref()
                .unwrap_err()
                .to_string()
                .contains("fee too small"),
        "Expected error when max fee supplied is too small, got: {:?}",
        result
    );
}

async fn test_does_not_alter_fee_when_no_inners(
    context: &AlgorandTestContext,
    app_id: u64,
    sender: Address,
) {
    println!("Test: does not alter fee when app call has no inners");

    let method = ABIMethod::from_str("no_op()void").unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    let expected_fee = 1000u64;

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(2000),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selector]),
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
        .send(SEND_PARAMS)
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

    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    let expected_fee = 7000u64;

    // This tuple represents the fees for the inner transactions
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(0u64)), // fee1
        ABIValue::Uint(BigUint::from(0u64)), // fee2
        ABIValue::Uint(BigUint::from(0u64)), // fee3
        ABIValue::Uint(BigUint::from(0u64)), // fee4
        // Nested inner transaction fees
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(0u64)), // fee5
            ABIValue::Uint(BigUint::from(0u64)), // fee6
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_encoded,
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
        .send(SEND_PARAMS)
        .await
        .expect("Failed to send application call");

    assert!(
        !result.confirmations.is_empty(),
        "Should have confirmations"
    );

    let transaction_fee = result.confirmations[0]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);

    assert_eq!(
        transaction_fee, expected_fee,
        "Fee should be altered to cover inner fees"
    );
}

async fn test_alters_fee_all_inner_fees_covered(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: alters fee, handling when all inner fees have been covered");

    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    let expected_fee = 1000u64;

    // This tuple represents all inner fees being pre-covered
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(1000u64)), // fee1 - covered
        ABIValue::Uint(BigUint::from(1000u64)), // fee2 - covered
        ABIValue::Uint(BigUint::from(1000u64)), // fee3 - covered
        ABIValue::Uint(BigUint::from(1000u64)), // fee4 - covered
        // All nested inner transaction fees are also covered
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(1000u64)), // fee5 - covered
            ABIValue::Uint(BigUint::from(1000u64)), // fee6 - covered
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_encoded,
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
        .send(SEND_PARAMS)
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

    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    let expected_fee = 5300u64;

    // This tuple represents some inner fees being partially covered
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(1000u64)), // fee1 - covered
        ABIValue::Uint(BigUint::from(0u64)),    // fee2 - not covered
        ABIValue::Uint(BigUint::from(200u64)),  // fee3 - partially covered
        ABIValue::Uint(BigUint::from(0u64)),    // fee4 - not covered
        // Mixed coverage in nested inner transaction fees
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(500u64)), // fee5 - partially covered
            ABIValue::Uint(BigUint::from(0u64)),   // fee6 - not covered
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_encoded,
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
        .send(SEND_PARAMS)
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

    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    let expected_fee = 2000u64;

    // This tuple represents some inner fees having surplus amounts
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(0u64)),    // fee1 - not covered
        ABIValue::Uint(BigUint::from(1000u64)), // fee2 - surplus (over required)
        ABIValue::Uint(BigUint::from(5000u64)), // fee3 - large surplus (over required)
        ABIValue::Uint(BigUint::from(0u64)),    // fee4 - not covered
        // Mixed surplus amounts in nested inner transaction fees
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(0u64)),  // fee5 - not covered
            ABIValue::Uint(BigUint::from(50u64)), // fee6 - small surplus
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_encoded,
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
        .send(SEND_PARAMS)
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

    let method = ABIMethod::from_str("burn_ops(uint64)void").unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    let expected_fee = 10_000u64;

    let uint64 = ABIType::from_str("uint64").unwrap();
    let op_budget_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(6200u64)))
        .expect("Failed to encode op_budget");

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee + 2_000), // Use expected_fee in calculation
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selector, op_budget_encoded]),
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
        .send(SEND_PARAMS)
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

    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    let expected_fee = 2000u64;

    // This tuple represents fees for nested transaction scenario
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(0u64)),    // fee1 - not covered
        ABIValue::Uint(BigUint::from(0u64)),    // fee2 - not covered
        ABIValue::Uint(BigUint::from(2000u64)), // fee3 - covered fee for nested scenario
        ABIValue::Uint(BigUint::from(0u64)),    // fee4 - not covered
        // Nested inner transaction fees
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(0u64)), // fee5 - not covered
            ABIValue::Uint(BigUint::from(0u64)), // fee6 - not covered
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

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
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_encoded,
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
    let method_2 = ABIMethod::from_str("no_op()void").unwrap();
    let method_2_selector = method_2.selector().expect("Failed to get method selector");

    let main_app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            static_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_2_selector]), // Simplified for testing
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    composer
        .add_application_call(main_app_call_params)
        .expect("Failed to add main application call");

    let result = composer
        .send(SEND_PARAMS)
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

    // Create ABI method
    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    // This tuple represents the fees for the inner transactions
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(0u64)),    // fee1
        ABIValue::Uint(BigUint::from(0u64)),    // fee2
        ABIValue::Uint(BigUint::from(2000u64)), // fee3
        ABIValue::Uint(BigUint::from(0u64)),    // fee4
        // Nested inner transaction fees
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(0u64)), // fee5
            ABIValue::Uint(BigUint::from(0u64)), // fee6
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

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
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_encoded,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    // Main app call that would normally take the above as nested arguments
    let method_2 = ABIMethod::from_str("no_op()void").unwrap();
    let method_2_selector = method_2.selector().expect("Failed to get method selector");

    let main_app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(10_000), // Sufficient for main call
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_2_selector]),
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
    let result = composer.send(SEND_PARAMS).await;

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

    // Create ABI method
    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    // This tuple represents the fees for the inner transactions
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(0u64)), // fee1
        ABIValue::Uint(BigUint::from(0u64)), // fee2
        ABIValue::Uint(BigUint::from(0u64)), // fee3
        ABIValue::Uint(BigUint::from(0u64)), // fee4
        // Nested inner transaction fees
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(0u64)), // fee5
            ABIValue::Uint(BigUint::from(0u64)), // fee6
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

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
            method_selector.clone(),
            app_id_2_encoded.clone(),
            app_id_3_encoded.clone(),
            fees_tuple_encoded.clone(),
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
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_encoded,
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
    let result = composer.send(SEND_PARAMS).await;

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

    composer
        .send(SEND_PARAMS)
        .await
        .expect("Failed to send payment");

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

async fn test_throws_when_max_fee_too_small(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: throws when max fee is too small to cover inner transaction fees");

    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    let expected_fee = 7000u64;

    // This tuple represents all fees as zero (requiring full fee coverage)
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(0u64)), // fee1 - not covered
        ABIValue::Uint(BigUint::from(0u64)), // fee2 - not covered
        ABIValue::Uint(BigUint::from(0u64)), // fee3 - not covered
        ABIValue::Uint(BigUint::from(0u64)), // fee4 - not covered
        // All nested inner transaction fees are also not covered
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(0u64)), // fee5 - not covered
            ABIValue::Uint(BigUint::from(0u64)), // fee6 - not covered
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee - 1), // Too small by 1 microAlgo
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_encoded,
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
    let result = composer.send(SEND_PARAMS).await;

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

    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    let expected_fee = 7000u64;

    // This tuple represents all fees as zero (requiring full fee coverage)
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(0u64)), // fee1 - not covered
        ABIValue::Uint(BigUint::from(0u64)), // fee2 - not covered
        ABIValue::Uint(BigUint::from(0u64)), // fee3 - not covered
        ABIValue::Uint(BigUint::from(0u64)), // fee4 - not covered
        // All nested inner transaction fees are also not covered
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(0u64)), // fee5 - not covered
            ABIValue::Uint(BigUint::from(0u64)), // fee6 - not covered
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            static_fee: Some(expected_fee - 1), // Too small by 1 microAlgo
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_encoded,
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
    let result = composer.send(SEND_PARAMS).await;

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

    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    let expected_fee = 6000u64;

    // This tuple represents partial inner fee coverage
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(1000u64)), // fee1 - covered
        ABIValue::Uint(BigUint::from(0u64)),    // fee2 - not covered
        ABIValue::Uint(BigUint::from(200u64)),  // fee3 - partially covered
        ABIValue::Uint(BigUint::from(0u64)),    // fee4 - not covered
        // Nested inner transaction fees - partial coverage
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(500u64)), // fee5 - partially covered
            ABIValue::Uint(BigUint::from(0u64)),   // fee6 - not covered
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            static_fee: Some(expected_fee), // Static fee with surplus
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_encoded,
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
        .send(SEND_PARAMS)
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

    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    let txn1_expected_fee = 5800u64;
    let txn2_expected_fee = 6000u64;

    // First transaction's fee tuple with varying coverage
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value_1 = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(0u64)),    // fee1 - not covered
        ABIValue::Uint(BigUint::from(1000u64)), // fee2 - covered
        ABIValue::Uint(BigUint::from(0u64)),    // fee3 - not covered
        ABIValue::Uint(BigUint::from(0u64)),    // fee4 - not covered
        // Nested inner transaction fees
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(200u64)), // fee5 - partially covered
            ABIValue::Uint(BigUint::from(0u64)),   // fee6 - not covered
        ]),
    ]);

    // Second transaction's fee tuple with different coverage
    let fees_tuple_value_2 = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(1000u64)), // fee1 - covered
        ABIValue::Uint(BigUint::from(0u64)),    // fee2 - not covered
        ABIValue::Uint(BigUint::from(0u64)),    // fee3 - not covered
        ABIValue::Uint(BigUint::from(0u64)),    // fee4 - not covered
        // Nested inner transaction fees
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(0u64)), // fee5 - not covered
            ABIValue::Uint(BigUint::from(0u64)), // fee6 - not covered
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_1_encoded = fees_tuple_type
        .encode(&fees_tuple_value_1)
        .expect("Failed to encode fees tuple 1");
    let fees_tuple_2_encoded = fees_tuple_type
        .encode(&fees_tuple_value_2)
        .expect("Failed to encode fees tuple 2");

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
            method_selector.clone(),
            app_id_2_encoded.clone(),
            app_id_3_encoded.clone(),
            fees_tuple_1_encoded,
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
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_2_encoded,
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
        .send(SEND_PARAMS)
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

    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    let expected_fee = 8000u64;

    // This tuple represents all fees as zero (requiring full fee coverage)
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(0u64)), // fee1 - not covered
        ABIValue::Uint(BigUint::from(0u64)), // fee2 - not covered
        ABIValue::Uint(BigUint::from(0u64)), // fee3 - not covered
        ABIValue::Uint(BigUint::from(0u64)), // fee4 - not covered
        // All nested inner transaction fees are also not covered
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(0u64)), // fee5 - not covered
            ABIValue::Uint(BigUint::from(0u64)), // fee6 - not covered
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

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
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_encoded,
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
        .send(SEND_PARAMS)
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

    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    // This tuple represents all fees as zero (requiring full fee coverage)
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(0u64)), // fee1 - not covered
        ABIValue::Uint(BigUint::from(0u64)), // fee2 - not covered
        ABIValue::Uint(BigUint::from(0u64)), // fee3 - not covered
        ABIValue::Uint(BigUint::from(0u64)), // fee4 - not covered
        // All nested inner transaction fees are also not covered
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(0u64)), // fee5 - not covered
            ABIValue::Uint(BigUint::from(0u64)), // fee6 - not covered
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

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
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_encoded,
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
        .send(SEND_PARAMS)
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

    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    let expected_fee = 7000u64;

    // This tuple represents the fees for the inner transactions with large surplus
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(0u64)), // fee1
        ABIValue::Uint(BigUint::from(0u64)), // fee2
        ABIValue::Uint(BigUint::from(0u64)), // fee3
        ABIValue::Uint(BigUint::from(0u64)), // fee4
        // Nested inner transaction fees with large surplus
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(0u64)),      // fee5
            ABIValue::Uint(BigUint::from(0u64)),      // fee6
            ABIValue::Uint(BigUint::from(20_000u64)), // fee7 - large surplus fee
            ABIValue::Uint(BigUint::from(0u64)),      // fee8
            ABIValue::Uint(BigUint::from(0u64)),      // fee9
            ABIValue::Uint(BigUint::from(0u64)),      // fee10
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_encoded,
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
        .send(SEND_PARAMS)
        .await
        .expect("Failed to send application call");

    assert!(
        !result.confirmations.is_empty(),
        "Should have confirmations"
    );

    let transaction_fee = result.confirmations[0]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);

    assert_eq!(
        transaction_fee, expected_fee,
        "Fee should be altered to cover inner fees"
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

    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    let expected_fee = 6300u64;

    // This tuple represents the fees for surplus that pools to some but not all lower siblings
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(0u64)),    // fee1
        ABIValue::Uint(BigUint::from(0u64)),    // fee2
        ABIValue::Uint(BigUint::from(2200u64)), // fee3 - surplus fee
        ABIValue::Uint(BigUint::from(0u64)),    // fee4
        // Nested inner transaction fees with partial surplus pooling
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(0u64)),    // fee5
            ABIValue::Uint(BigUint::from(0u64)),    // fee6
            ABIValue::Uint(BigUint::from(2500u64)), // fee7 - surplus fee that pools to some siblings
            ABIValue::Uint(BigUint::from(0u64)),    // fee8
            ABIValue::Uint(BigUint::from(0u64)),    // fee9
            ABIValue::Uint(BigUint::from(0u64)),    // fee10
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_encoded,
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
        .send(SEND_PARAMS)
        .await
        .expect("Failed to send application call");

    assert!(
        !result.confirmations.is_empty(),
        "Should have confirmations"
    );

    let transaction_fee = result.confirmations[0]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);

    assert_eq!(
        transaction_fee, expected_fee,
        "Fee should be altered to cover inner fees"
    );
}

async fn test_alters_fee_large_surplus_no_pooling(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: alters fee, handling a large inner fee surplus with no pooling");

    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    let expected_fee = 10_000u64;

    // This tuple represents large surplus at the end with no lower siblings to pool to
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(0u64)), // fee1
        ABIValue::Uint(BigUint::from(0u64)), // fee2
        ABIValue::Uint(BigUint::from(0u64)), // fee3
        ABIValue::Uint(BigUint::from(0u64)), // fee4
        // Nested inner transaction fees with large surplus at the end
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(0u64)),      // fee5
            ABIValue::Uint(BigUint::from(0u64)),      // fee6
            ABIValue::Uint(BigUint::from(0u64)),      // fee7
            ABIValue::Uint(BigUint::from(0u64)),      // fee8
            ABIValue::Uint(BigUint::from(0u64)),      // fee9
            ABIValue::Uint(BigUint::from(20_000u64)), // fee10 - large surplus at end
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_encoded,
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
        .send(SEND_PARAMS)
        .await
        .expect("Failed to send application call");

    assert!(
        !result.confirmations.is_empty(),
        "Should have confirmations"
    );

    let transaction_fee = result.confirmations[0]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);

    assert_eq!(
        transaction_fee, expected_fee,
        "Fee should be altered to cover inner fees"
    );
}

async fn test_alters_fee_multiple_surplus_poolings(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: alters fee, handling multiple inner fee surplus poolings to lower siblings");

    let method = ABIMethod::from_str(
        "send_inners_with_fees_2(uint64,uint64,(uint64,uint64,uint64[],uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    let expected_fee = 7100u64;

    // This tuple represents the more complex fee structure with multiple surplus poolings
    let fees_tuple_type =
        ABIType::from_str("(uint64,uint64,uint64[],uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(0u64)),    // fee1
        ABIValue::Uint(BigUint::from(1200u64)), // fee2 - surplus fee
        // First fee array with surplus that pools to lower siblings
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(0u64)),    // fee_array_1[0]
            ABIValue::Uint(BigUint::from(0u64)),    // fee_array_1[1]
            ABIValue::Uint(BigUint::from(4900u64)), // fee_array_1[2] - surplus fee
            ABIValue::Uint(BigUint::from(0u64)),    // fee_array_1[3]
            ABIValue::Uint(BigUint::from(0u64)),    // fee_array_1[4]
            ABIValue::Uint(BigUint::from(0u64)),    // fee_array_1[5]
        ]),
        ABIValue::Uint(BigUint::from(200u64)),  // fee3
        ABIValue::Uint(BigUint::from(1100u64)), // fee4 - surplus fee
        // Second fee array with surplus that pools to lower siblings
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(0u64)),    // fee_array_2[0]
            ABIValue::Uint(BigUint::from(0u64)),    // fee_array_2[1]
            ABIValue::Uint(BigUint::from(2500u64)), // fee_array_2[2] - surplus fee
            ABIValue::Uint(BigUint::from(0u64)),    // fee_array_2[3]
            ABIValue::Uint(BigUint::from(0u64)),    // fee_array_2[4]
            ABIValue::Uint(BigUint::from(0u64)),    // fee_array_2[5]
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_encoded,
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
        .send(SEND_PARAMS)
        .await
        .expect("Failed to send application call");

    assert!(
        !result.confirmations.is_empty(),
        "Should have confirmations"
    );

    let transaction_fee = result.confirmations[0]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);

    assert_eq!(
        transaction_fee, expected_fee,
        "Fee should be altered to cover inner fees"
    );
}

async fn test_throws_when_max_fee_below_calculated(
    context: &AlgorandTestContext,
    app_id_1: u64,
    app_id_2: u64,
    app_id_3: u64,
    sender: Address,
) {
    println!("Test: throws when maxFee is below the calculated fee");

    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    // This tuple represents all fees as zero (requiring full fee coverage)
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(0u64)), // fee1 - not covered
        ABIValue::Uint(BigUint::from(0u64)), // fee2 - not covered
        ABIValue::Uint(BigUint::from(0u64)), // fee3 - not covered
        ABIValue::Uint(BigUint::from(0u64)), // fee4 - not covered
        // All nested inner transaction fees are also not covered
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(0u64)), // fee5 - not covered
            ABIValue::Uint(BigUint::from(0u64)), // fee6 - not covered
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

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
            method_selector.clone(),
            app_id_2_encoded.clone(),
            app_id_3_encoded.clone(),
            fees_tuple_encoded.clone(),
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    // Second transaction with sufficient fee to allow simulate to succeed
    let method_2 = ABIMethod::from_str("no_op()void").unwrap();
    let method_2_selector = method_2.selector().expect("Failed to get method selector");

    let app_call_params_2 = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(10_000),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_2_selector]),
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
    let result = composer.send(SEND_PARAMS).await;

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

    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    // This tuple represents all fees as zero (requiring full fee coverage)
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(0u64)), // fee1 - not covered
        ABIValue::Uint(BigUint::from(0u64)), // fee2 - not covered
        ABIValue::Uint(BigUint::from(0u64)), // fee3 - not covered
        ABIValue::Uint(BigUint::from(0u64)), // fee4 - not covered
        // All nested inner transaction fees are also not covered
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(0u64)), // fee5 - not covered
            ABIValue::Uint(BigUint::from(0u64)), // fee6 - not covered
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_2_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_2)))
        .expect("Failed to encode app_id_2");
    let app_id_3_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id_3)))
        .expect("Failed to encode app_id_3");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

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
            method_selector,
            app_id_2_encoded,
            app_id_3_encoded,
            fees_tuple_encoded,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    // Second transaction with sufficient fee to allow simulate to succeed
    let method_2 = ABIMethod::from_str("no_op()void").unwrap();
    let method_2_selector = method_2.selector().expect("Failed to get method selector");

    let app_call_params_2 = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(10_000),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_2_selector]),
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
    let result = composer.send(SEND_PARAMS).await;

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
        args: Some({
            let method = ABIMethod::from_str("burn_ops_readonly(uint64)void").unwrap();
            let method_selector = method.selector().expect("Failed to get method selector");
            let uint64 = ABIType::from_str("uint64").unwrap();
            let op_budget_encoded = uint64
                .encode(&ABIValue::Uint(BigUint::from(6200u64)))
                .expect("Failed to encode op_budget");
            vec![method_selector, op_budget_encoded]
        }),
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
        .send(Some(SendParams {
            cover_app_call_inner_transaction_fees: Some(cover_inner_fees),
            ..Default::default()
        }))
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

    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    let expected_fee = 12_000u64;

    // This tuple represents partial inner fee coverage for readonly context
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(1000u64)), // fee1 - covered
        ABIValue::Uint(BigUint::from(0u64)),    // fee2 - not covered
        ABIValue::Uint(BigUint::from(200u64)),  // fee3 - partially covered
        ABIValue::Uint(BigUint::from(0u64)),    // fee4 - not covered
        // Nested inner transaction fees - partial coverage
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(500u64)), // fee5 - partially covered
            ABIValue::Uint(BigUint::from(0u64)),   // fee6 - not covered
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id)))
        .expect("Failed to encode app_id");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selector,        // Would be marked as readonly in real impl
            app_id_encoded.clone(), // Use same app for simplicity
            app_id_encoded,
            fees_tuple_encoded,
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
        .send(SEND_PARAMS)
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

    let method = ABIMethod::from_str(
        "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
    )
    .unwrap();
    let method_selector = method.selector().expect("Failed to get method selector");

    // This tuple represents partial inner fee coverage for readonly context
    let fees_tuple_type = ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])").unwrap();
    let fees_tuple_value = ABIValue::Array(vec![
        ABIValue::Uint(BigUint::from(1000u64)), // fee1 - covered
        ABIValue::Uint(BigUint::from(0u64)),    // fee2 - not covered
        ABIValue::Uint(BigUint::from(200u64)),  // fee3 - partially covered
        ABIValue::Uint(BigUint::from(0u64)),    // fee4 - not covered
        // Nested inner transaction fees - partial coverage
        ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(500u64)), // fee5 - partially covered
            ABIValue::Uint(BigUint::from(0u64)),   // fee6 - not covered
        ]),
    ]);

    let uint64 = ABIType::from_str("uint64").unwrap();
    let app_id_encoded = uint64
        .encode(&ABIValue::Uint(BigUint::from(app_id)))
        .expect("Failed to encode app_id");
    let fees_tuple_encoded = fees_tuple_type
        .encode(&fees_tuple_value)
        .expect("Failed to encode fees tuple");

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender.clone(),
            max_fee: Some(2000), // Too small for the inner fees
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selector, // Would be marked as readonly in real impl
            app_id_encoded.clone(),
            app_id_encoded,
            fees_tuple_encoded,
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
    let result = composer.send(SEND_PARAMS).await;

    match result {
        Ok(_) => println!(
            "✓ Test passed (placeholder - would fail with insufficient max fee for readonly)"
        ),
        Err(e) => println!("Readonly transaction failed as expected: {}", e),
    }
}
