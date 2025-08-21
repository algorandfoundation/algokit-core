use crate::common::init_test_logging;
use algokit_abi::abi_type::BitSize;
use algokit_abi::{ABIMethod, ABIType, ABIValue};
use algokit_test_artifacts::{inner_fee_contract, nested_contract};
use algokit_transact::{Address, OnApplicationComplete, TransactionId};
use algokit_utils::transactions::composer::{ResourcePopulation, SendParams};
use algokit_utils::{AppCallParams, AppCreateParams, PaymentParams, testing::*};
use algokit_utils::{CommonParams, Composer};
use base64::{Engine, prelude::BASE64_STANDARD};
use rstest::*;
use serde::Deserialize;
use std::str::FromStr;

#[fixture]
async fn setup(
    #[default(3)] inner_fee_app_count: u8,
    #[default(0)] nested_app_count: u8,
) -> SetupResult {
    init_test_logging();
    let mut fixture = algorand_fixture().await?;
    fixture.new_scope().await?;

    let context = fixture.context()?;
    let sender_address = context.test_account.account()?.address();

    let mut app_ids = Vec::new();

    for i in 1..=inner_fee_app_count {
        let app_id = deploy_inner_fee_app(context, &format!("inner_fee_app_{}", i)).await?;
        app_ids.push(app_id);
    }

    for i in 1..=nested_app_count {
        let app_id = deploy_nested_app(context, &format!("nested_app_{}", i)).await?;
        app_ids.push(app_id);
    }

    fund_app_accounts(context, &app_ids, 500_000).await?;

    Ok(TestData {
        sender_address,
        app_ids,
        fixture,
        method_selectors: MethodSelectors {
            no_op: ABIMethod::from_str("no_op()void")?.selector()?,
            send_inners_with_fees: ABIMethod::from_str(
                "send_inners_with_fees(uint64,uint64,(uint64,uint64,uint64,uint64,uint64[]))void",
            )?
            .selector()?,
            send_inners_with_fees_2: ABIMethod::from_str("send_inners_with_fees_2(uint64,uint64,(uint64,uint64,uint64[],uint64,uint64,uint64[]))void")?
                .selector()?,
            nested_txn_arg: ABIMethod::from_str("nestedTxnArg(pay,appl)uint64")?.selector()?,
            burn_ops: ABIMethod::from_str("burn_ops(uint64)void")?.selector()?,
            burn_ops_readonly: ABIMethod::from_str("burn_ops_readonly(uint64)void")?.selector()?,
        },
        abi_types: ABITypes {
            uint64: ABIType::Uint(BitSize::new(64)?),
            fees_tuple: ABIType::from_str("(uint64,uint64,uint64,uint64,uint64[])")?,
            fees_2_tuple: ABIType::from_str("(uint64,uint64,uint64[],uint64,uint64,uint64[])")?,
        },
    })
}

#[rstest]
#[tokio::test]
/// Errors when no max fee is supplied
async fn test_errors_when_no_max_fee_supplied(
    #[with(1)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        fixture,
        method_selectors,
        abi_types: _,
    } = setup.await?;
    let app_id = app_ids[0];
    let mut composer = fixture.context()?.composer.clone();

    let params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.no_op]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    composer.add_app_call(params)?;
    let result = composer.send(COVER_FEES_SEND_PARAMS).await;

    assert!(
        result.is_err()
            && result
                .as_ref()
                .unwrap_err()
                .to_string()
                .contains("Please provide a max fee"),
        "Unexpected result, got: {:?}",
        result
    );

    Ok(())
}

#[rstest]
#[tokio::test]
/// Errors when inner transaction fees are not covered and coverAppCallInnerTransactionFees is disabled
async fn test_errors_when_inner_fees_not_covered_and_fee_coverage_disabled(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);

    let fees_tuple = create_fees_tuple(0, 0, 0, 0, vec![0, 0]);
    let txn_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(7000),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_params)?;

    let result = composer
        .send(Some(SendParams {
            cover_app_call_inner_transaction_fees: false,
            populate_app_call_resources: ResourcePopulation::Enabled {
                use_access_list: false,
            }, // Ensure the same behaviour when simulating due to resource population
            ..Default::default()
        }))
        .await;

    assert!(
        result.is_err()
            && result
                .as_ref()
                .unwrap_err()
                .to_string()
                .contains("fee too small"),
        "Unexpected result, got: {:?}",
        result
    );

    Ok(())
}

#[rstest]
#[tokio::test]
/// Does not alter fee when app call has no inners
async fn test_does_not_alter_fee_when_no_inners(
    #[with(1)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        mut fixture,
        method_selectors,
        abi_types: _,
    } = setup.await?;
    let app_id = app_ids[0];

    let mut composer = fixture.context()?.composer.clone();

    let expected_fee = 1000u64;

    let txn_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(2000),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.no_op]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_params.clone())?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await?;

    assert_eq!(result.confirmations.len(), 1);
    let transaction_fee = result.confirmations[0]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);
    assert_eq!(transaction_fee, expected_fee);
    assert_min_fee(fixture.new_composer()?, &txn_params, transaction_fee).await;

    Ok(())
}

#[rstest]
#[tokio::test]
// Alters fee, handling when no inner fees have been covered
async fn test_alters_fee_no_inner_fees_covered(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        mut fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);
    let expected_fee = 7000u64;

    // Create an app call transaction that has no inner fees covered
    let fees_tuple = create_fees_tuple(0, 0, 0, 0, vec![0, 0]);
    let txn_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_params.clone())?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await?;

    assert_eq!(result.confirmations.len(), 1);
    let transaction_fee = result.confirmations[0]
        .txn
        .transaction
        .header()
        .fee
        .unwrap_or(0);
    assert_eq!(transaction_fee, expected_fee);
    assert_min_fee(fixture.new_composer()?, &txn_params, transaction_fee).await;

    Ok(())
}

#[rstest]
#[tokio::test]
/// Alters fee, handling when all inner fees have been covered
async fn test_alters_fee_all_inner_fees_covered(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        mut fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);

    let expected_fee = 1000u64;

    // Create an app call transaction that has all inner fees covered
    let fees_tuple = create_fees_tuple(1000, 1000, 1000, 1000, vec![1000, 1000]);
    let txn_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_params.clone())?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await?;

    assert_eq!(result.confirmations.len(), 1);
    let actual_fees = result
        .confirmations
        .iter()
        .map(|c| c.txn.transaction.header().fee.unwrap_or(0))
        .collect::<Vec<_>>();
    assert_eq!(actual_fees[0], expected_fee);
    assert_min_fee(fixture.new_composer()?, &txn_params, expected_fee).await;

    Ok(())
}

#[rstest]
#[tokio::test]
/// Alters fee, handling when some inner fees have been covered or partially covered
async fn test_alters_fee_some_inner_fees_covered(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        mut fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);

    let expected_fee = 5300u64;

    // This tuple represents some inner fees being partially covered
    let fees_tuple = create_fees_tuple(1000, 0, 200, 0, vec![500, 0]);

    let txn_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_params.clone())?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await?;

    assert_eq!(result.confirmations.len(), 1);
    let actual_fees = result
        .confirmations
        .iter()
        .map(|c| c.txn.transaction.header().fee.unwrap_or(0))
        .collect::<Vec<_>>();
    assert_eq!(actual_fees[0], expected_fee);
    assert_min_fee(fixture.new_composer()?, &txn_params, expected_fee).await;

    Ok(())
}

#[rstest]
#[tokio::test]
/// Alters fee, handling when some inner fees have a surplus
async fn test_alters_fee_some_inner_fees_surplus(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        mut fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);

    let expected_fee = 2000u64;

    // Create an app call transaction that has some inner fees with surplus
    let fees_tuple = create_fees_tuple(0, 1000, 5000, 0, vec![0, 50]);

    let txn_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_params.clone())?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await?;

    assert_eq!(result.confirmations.len(), 1);
    let actual_fees = result
        .confirmations
        .iter()
        .map(|c| c.txn.transaction.header().fee.unwrap_or(0))
        .collect::<Vec<_>>();
    assert_eq!(actual_fees[0], expected_fee);
    assert_min_fee(fixture.new_composer()?, &txn_params, expected_fee).await;

    Ok(())
}

#[rstest]
#[tokio::test]
/// alters fee, handling expensive abi method calls that use ensure_budget to op-up
async fn test_alters_fee_expensive_abi_method_calls(
    #[with(1)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        mut fixture,
        method_selectors,
        abi_types,
    } = setup.await?;

    let mut composer = fixture.context()?.composer.clone();
    let app_id = app_ids[0];
    let expected_fee = 10_000u64;

    let op_budget_encoded = abi_types.uint64.encode(&ABIValue::from(6200u64))?;

    let params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(expected_fee + 2_000),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.burn_ops, op_budget_encoded]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(params.clone())?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await?;

    assert_eq!(result.confirmations.len(), 1);
    let actual_fees: Vec<u64> = result
        .confirmations
        .iter()
        .map(|c| c.txn.transaction.header().fee.unwrap_or(0))
        .collect();
    assert_eq!(actual_fees[0], expected_fee);

    assert_min_fee(fixture.new_composer()?, &params, expected_fee).await;

    Ok(())
}

#[rstest]
#[tokio::test]
/// Errors when max fee is too small to cover inner transaction fees
async fn test_errors_when_max_fee_too_small(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);
    let expected_fee = 7000u64;

    // Create an app call transaction that has no inner fees covered
    let fees_tuple = create_fees_tuple(0, 0, 0, 0, vec![0, 0]);
    let params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(expected_fee - 1),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(params.clone())?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await;

    assert!(
        result.is_err()
            && result
                .as_ref()
                .unwrap_err()
                .to_string()
                .contains("Fees were too small"),
        "Unexpected result, got: {:?}",
        result
    );

    Ok(())
}

#[rstest]
#[tokio::test]
/// Errors when static fee is too small to cover inner transaction fees
async fn test_errors_when_static_fee_too_small(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);
    let expected_fee = 7000u64;

    // Create an app call transaction that has no inner fees covered
    let fees_tuple = create_fees_tuple(0, 0, 0, 0, vec![0, 0]);
    let params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            static_fee: Some(expected_fee - 1),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(params.clone())?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await;

    assert!(
        result.is_err()
            && result
                .as_ref()
                .unwrap_err()
                .to_string()
                .contains("Fees were too small"),
        "Unexpected result, got: {:?}",
        result
    );

    Ok(())
}

#[rstest]
#[tokio::test]
/// Does not alter a static fee with surplus
async fn test_does_not_alter_static_fee_with_surplus(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);
    let expected_fee = 6000u64;

    // Create an app call transaction that has a static fee with surplus
    let fees_tuple = create_fees_tuple(1000, 0, 200, 0, vec![500, 0]);
    let app_call_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            static_fee: Some(expected_fee), // Static fee with surplus
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };

    composer.add_app_call(app_call_params)?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await?;

    assert_eq!(result.confirmations.len(), 1);
    let actual_fees: Vec<u64> = result
        .confirmations
        .iter()
        .map(|c| c.txn.transaction.header().fee.unwrap_or(0))
        .collect();
    assert_eq!(actual_fees[0], expected_fee);

    Ok(())
}

#[rstest]
#[tokio::test]
/// alters fee, handling multiple app calls in a group that send inners with varying fees
async fn test_alters_fee_multiple_app_calls_in_group(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        mut fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);

    // Create an app call transaction that has varying inner fees
    let txn_1_expected_fee = 5800u64;
    let txn_1_fee_tuple = create_fees_tuple(0, 1000, 0, 0, vec![200, 0]);
    let txn_1_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            static_fee: Some(txn_1_expected_fee),
            note: Some(b"txn1".to_vec()),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees.clone(),
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&txn_1_fee_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_1_params.clone())?;

    // Create an app call transaction that has different varying inner fees
    let txn_2_expected_fee = 6000u64;
    let txn_2_fee_tuple = create_fees_tuple(1000, 0, 0, 0, vec![0, 0]);
    let txn_2_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(txn_2_expected_fee),
            note: Some(b"txn2".to_vec()),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&txn_2_fee_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_2_params.clone())?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await?;

    assert_eq!(result.confirmations.len(), 2);
    let actual_fees: Vec<u64> = result
        .confirmations
        .iter()
        .map(|c| c.txn.transaction.header().fee.unwrap_or(0))
        .collect();
    assert_eq!(actual_fees[0], txn_1_expected_fee);
    assert_eq!(actual_fees[1], txn_2_expected_fee);

    assert_min_fee(fixture.new_composer()?, &txn_1_params, txn_1_expected_fee).await;
    assert_min_fee(fixture.new_composer()?, &txn_2_params, txn_2_expected_fee).await;

    Ok(())
}

#[rstest]
#[tokio::test]
/// Does not alter fee when another transaction in the group covers the inner fees
async fn test_does_not_alter_fee_when_group_covers_inner_fees(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);

    // Create a payment transaction that will cover the inner fees of transaction 2
    let txn_1_expected_fee = 8000u64;
    let txn_1_params = PaymentParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            static_fee: Some(txn_1_expected_fee),
            ..Default::default()
        },
        receiver: sender_address.clone(),
        amount: 0,
    };
    composer.add_payment(txn_1_params)?;

    // Create an app call transaction that has inner fees covered by the above payment
    let fees_tuple = create_fees_tuple(0, 0, 0, 0, vec![0, 0]);
    let txn_2_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(txn_1_expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_2_params)?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await?;

    assert_eq!(result.confirmations.len(), 2);
    let actual_fees: Vec<u64> = result
        .confirmations
        .iter()
        .map(|c| c.txn.transaction.header().fee.unwrap_or(0))
        .collect();
    assert_eq!(actual_fees[0], txn_1_expected_fee);
    // We could technically reduce this to 0, however it adds more complexity and is probably unlikely to be a common use case
    assert_eq!(actual_fees[1], 1000);

    Ok(())
}

#[rstest]
#[tokio::test]
/// Alters fee, handling nested abi method calls
async fn test_alters_fee_nested_abi_method_call(
    #[with(3, 1)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let [app_id_1, app_id_2, app_id_3, app_id_4] = [app_ids[0], app_ids[1], app_ids[2], app_ids[3]];
    let expected_fee = 2000u64;

    // Create a payment transaction that will be used as a nested argument
    let txn_1_params = PaymentParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            static_fee: Some(1500),
            ..Default::default()
        },
        receiver: sender_address.clone(),
        amount: 0,
    };
    composer.add_payment(txn_1_params.clone())?;

    // Create an app call transaction that will be used as a nested argument
    let fees_tuple = create_fees_tuple(0, 0, 2000, 0, vec![0, 0]);
    let txn_2_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(6000),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_2_params.clone())?;

    // Create the app call that will use the nested transaction
    let txn_3_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            static_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_4,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.nested_txn_arg]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_3_params.clone())?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await?;

    assert_eq!(result.confirmations.len(), 3);
    let actual_fees = result
        .confirmations
        .iter()
        .map(|c| c.txn.transaction.header().fee.unwrap_or(0))
        .collect::<Vec<_>>();
    assert_eq!(actual_fees[0], 1500);
    assert_eq!(actual_fees[1], 3500);
    assert_eq!(actual_fees[2], expected_fee);

    Ok(())
}

#[rstest]
#[tokio::test]
/// Errors when nested maxFee is below the calculated fee
async fn test_errors_when_nested_max_fee_below_calculated(
    #[with(3, 1)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let [app_id_1, app_id_2, app_id_3, app_id_4] = [app_ids[0], app_ids[1], app_ids[2], app_ids[3]];

    // Create a payment transaction that will be used as a nested argument
    let txn_1_params = PaymentParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        receiver: sender_address.clone(),
        amount: 0,
    };
    composer.add_payment(txn_1_params)?;

    // Create an app call transaction that will be used as a nested argument
    // This transaction has an insufficient max fee
    let fees_tuple = create_fees_tuple(0, 0, 2000, 0, vec![0, 0]);
    let txn_2_max_fee = 2000; // Too low for the calculated fee
    let txn_2_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(txn_2_max_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_2_params)?;

    // Create an app call transaction that will be used as a nested argument
    let txn_3_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(10_000),
            ..Default::default()
        },
        app_id: app_id_4,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.nested_txn_arg]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_3_params)?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await;

    assert!(
        result.is_err()
            && result.as_ref().unwrap_err().to_string().contains(
                format!(
                    "fee {} µALGO is greater than max of {}",
                    5000, txn_2_max_fee
                )
                .as_str()
            ),
        "Unexpected result, got: {:?}",
        result
    );

    Ok(())
}

#[rstest]
#[tokio::test]
/// Alters fee, allocating surplus fees to the most fee constrained transaction first
async fn test_alters_fee_allocating_surplus_to_most_constrained(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);

    // Create an app call transaction with inners that have no fees
    let fees_tuple_1 = create_fees_tuple(0, 0, 0, 0, vec![0, 0]);
    let txn_1_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(2000),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees.clone(),
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple_1)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_1_params)?;

    // Create a payment transaction with large static fee
    let txn_2_params = PaymentParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            static_fee: Some(7500),
            ..Default::default()
        },
        receiver: sender_address.clone(),
        amount: 0,
    };
    composer.add_payment(txn_2_params)?;

    // Create a payment transaction with static fee of 0
    let txn_3_params = PaymentParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            static_fee: Some(0),
            ..Default::default()
        },
        receiver: sender_address.clone(),
        amount: 0,
    };
    composer.add_payment(txn_3_params)?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await?;

    assert_eq!(result.confirmations.len(), 3);
    let actual_fees = result
        .confirmations
        .iter()
        .map(|c| c.txn.transaction.header().fee.unwrap_or(0))
        .collect::<Vec<_>>();
    assert_eq!(actual_fees[0], 1500);
    assert_eq!(actual_fees[1], 7500);
    assert_eq!(actual_fees[2], 0);

    Ok(())
}

#[rstest]
#[tokio::test]
/// Alters fee, handling a large inner fee surplus pooling to lower siblings
async fn test_alters_fee_large_surplus_pooling_to_lower_siblings(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        mut fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);
    let expected_fee = 7000u64;

    // Create an app call transaction that has a large inner fee surplus pooling to lower siblings
    let fees_tuple = create_fees_tuple(0, 0, 0, 0, vec![0, 0, 20_000, 0, 0, 0]);
    let txn_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_params.clone())?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await?;

    assert_eq!(result.confirmations.len(), 1);
    let actual_fees = result
        .confirmations
        .iter()
        .map(|c| c.txn.transaction.header().fee.unwrap_or(0))
        .collect::<Vec<_>>();
    assert_eq!(actual_fees[0], expected_fee);
    assert_min_fee(fixture.new_composer()?, &txn_params, expected_fee).await;

    Ok(())
}

#[rstest]
#[tokio::test]
/// alters fee, handling a inner fee surplus pooling to some lower siblings
async fn test_alters_fee_surplus_pooling_to_some_siblings(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        mut fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);
    let expected_fee = 6300u64;

    // Create an app call transaction that has a inner fee surplus pooling to some lower siblings
    let fees_tuple = create_fees_tuple(0, 0, 2200, 0, vec![0, 0, 2500, 0, 0, 0]);
    let txn_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_params.clone())?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await?;

    assert_eq!(result.confirmations.len(), 1);
    let actual_fees = result
        .confirmations
        .iter()
        .map(|c| c.txn.transaction.header().fee.unwrap_or(0))
        .collect::<Vec<_>>();
    assert_eq!(actual_fees[0], expected_fee);
    assert_min_fee(fixture.new_composer()?, &txn_params, expected_fee).await;

    Ok(())
}

#[rstest]
#[tokio::test]
/// Alters fee, handling a large inner fee surplus with no pooling
async fn test_alters_fee_large_surplus_no_pooling(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        mut fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);
    let expected_fee = 10_000u64;

    // Create an app call transaction that has a large inner fee surplus with no pooling
    let fees_tuple = create_fees_tuple(0, 0, 0, 0, vec![0, 0, 0, 0, 0, 20_000]);
    let txn_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_params.clone())?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await?;

    assert_eq!(result.confirmations.len(), 1);
    let actual_fees = result
        .confirmations
        .iter()
        .map(|c| c.txn.transaction.header().fee.unwrap_or(0))
        .collect::<Vec<_>>();
    assert_eq!(actual_fees[0], expected_fee);
    assert_min_fee(fixture.new_composer()?, &txn_params, expected_fee).await;

    Ok(())
}

#[rstest]
#[tokio::test]
/// Alters fee, handling multiple inner fee surplus poolings to lower siblings
async fn test_alters_fee_multiple_surplus_poolings(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        mut fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);
    let expected_fee = 7100u64;

    // Create an app call transaction that has multiple inner fee surplus poolings to lower siblings
    let fees_tuple = ABIValue::Array(vec![
        ABIValue::from(0u64),
        ABIValue::from(1200u64),
        ABIValue::from(vec![
            ABIValue::from(0u64),
            ABIValue::from(0u64),
            ABIValue::from(4900u64),
            ABIValue::from(0u64),
            ABIValue::from(0u64),
            ABIValue::from(0u64),
        ]),
        ABIValue::from(200u64),
        ABIValue::from(1100u64),
        ABIValue::from(vec![
            ABIValue::from(0u64),
            ABIValue::from(0u64),
            ABIValue::from(2500u64),
            ABIValue::from(0u64),
            ABIValue::from(0u64),
            ABIValue::from(0u64),
        ]),
    ]);
    let txn_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees_2,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_2_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_params.clone())?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await?;

    assert_eq!(result.confirmations.len(), 1);
    let actual_fees = result
        .confirmations
        .iter()
        .map(|c| c.txn.transaction.header().fee.unwrap_or(0))
        .collect::<Vec<_>>();
    assert_eq!(actual_fees[0], expected_fee);
    assert_min_fee(fixture.new_composer()?, &txn_params, expected_fee).await;

    Ok(())
}

#[rstest]
#[tokio::test]
/// Errors when maxFee is below the calculated fee
async fn test_errors_when_max_fee_below_calculated(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);

    // Create an app call transaction that has no inner fees covered
    let fees_tuple = create_fees_tuple(0, 0, 0, 0, vec![0, 0]);
    let txn_1_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(1200),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_1_params)?;

    // Create an app call transaction that has large max fee,
    // without it the simulate call to get the execution info would fail
    let txn_2_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(10_000),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.no_op]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_2_params)?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await;

    assert!(
        result.is_err()
            && result
                .as_ref()
                .unwrap_err()
                .to_string()
                .contains("fee 7000 µALGO is greater than max of 1200"),
        "Unexpected result, got: {:?}",
        result
    );

    Ok(())
}

#[rstest]
#[tokio::test]
/// Errors when staticFee is below the calculated fee
async fn test_errors_when_static_fee_below_calculated(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);

    // Create an app call transaction that has no inner fees covered
    let fees_tuple = create_fees_tuple(0, 0, 0, 0, vec![0, 0]);
    let params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            static_fee: Some(5000),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(params)?;

    // Create an app call transaction that has large max fee,
    // without it the simulate call to get the execution info would fail
    let txn_2_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(10_000),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.no_op]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_2_params)?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await;

    assert!(
        result.is_err()
            && result
                .as_ref()
                .unwrap_err()
                .to_string()
                .contains("fee 7000 µALGO is greater than max of 5000"),
        "Unexpected result, got: {:?}",
        result
    );

    Ok(())
}

#[rstest]
#[tokio::test]
/// Errors when staticFee for non app call transaction is too low
async fn test_errors_when_static_fee_too_low_for_non_app_call(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_ids,
        fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);
    let fees_tuple = create_fees_tuple(0, 0, 0, 0, vec![0, 0]);

    // Create an app call transaction with both high static and max fee
    let txn_1_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            static_fee: Some(13_000),
            max_fee: Some(14_000),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees.clone(),
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_1_params)?;

    // Create an app call transaction with low static
    let txn_2_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            static_fee: Some(1000),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_2_params)?;

    // Payment transaction with insufficient static fee
    let txn_3_params = PaymentParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            static_fee: Some(500),
            ..Default::default()
        },
        receiver: sender_address.clone(),
        amount: 0,
    };
    composer.add_payment(txn_3_params)?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await;

    assert!(
        result.is_err()
            && result
                .as_ref()
                .unwrap_err()
                .to_string()
                .contains("fee of 500 µALGO is required for non app call transaction"),
        "Unexpected result, got: {:?}",
        result
    );

    Ok(())
}

#[rstest]
#[case(true)]
#[case(false)]
#[tokio::test]
#[ignore = "Readonly method support not yet implemented"]
/// Uses fixed opcode budget without op-up inner transactions
async fn test_readonly_fixed_opcode_budget(
    #[with(1)]
    #[future]
    setup: SetupResult,
    #[case] cover_inner_fees: bool,
) -> TestResult {
    // This test verifies that readonly calls use the fixed max opcode budget and don't require inner transactions for op-ups,
    // regardless of coverAppCallInnerTransactionFees setting.
    let TestData {
        sender_address,
        app_ids,
        fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let app_id = app_ids[0];

    let op_budget_encoded = abi_types.uint64.encode(&ABIValue::from(6200u64))?; // This would normally require op-ups via inner transactions
    let txn_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.burn_ops_readonly, op_budget_encoded]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_params)?;

    let result = composer
        .send(Some(SendParams {
            cover_app_call_inner_transaction_fees: cover_inner_fees,
            ..Default::default()
        }))
        .await?;

    assert_eq!(result.confirmations.len(), 1);
    let actual_fees = result
        .confirmations
        .iter()
        .map(|c| c.txn.transaction.header().fee.unwrap_or(0))
        .collect::<Vec<_>>();
    assert_eq!(actual_fees[0], 1000);
    assert!(result.confirmations[0].inner_txns.is_none()); // No op-up inner transactions needed

    Ok(())
}

#[rstest]
#[tokio::test]
#[ignore = "Readonly method support not yet implemented"]
/// Readonly method alters fee when handling inner transactions
async fn test_readonly_alters_fee_handling_inners(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    // TODO: When readonly support is added, some code will be need to force `send_inners_with_fees` to be marked as readonly.
    let TestData {
        sender_address,
        app_ids,
        fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);
    let expected_fee = 12_000u64;

    // The expected_fee differs to non readonly method call, as we don't want to run simulate twice (once for resolving the minimum fee and once for the actual transaction result).
    // Because no fees are actually paid with readonly calls, we simply use the max_fee value (if set) and skip any minimum fee calculations.
    // If this method is running in a non readonly context, the minimum fee would be calculated as 5300.
    let fees_tuple = create_fees_tuple(1000, 0, 200, 0, vec![500, 0]);
    let txn_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(expected_fee),
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_params)?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await?;

    assert_eq!(result.confirmations.len(), 1);
    let actual_fees = result
        .confirmations
        .iter()
        .map(|c| c.txn.transaction.header().fee.unwrap_or(0))
        .collect::<Vec<_>>();
    assert_eq!(actual_fees[0], expected_fee);
    println!("TxnId: {}", result.confirmations[0].txn.transaction.id()?);
    assert_eq!(
        result.confirmations[0].inner_txns.as_ref().unwrap().len(),
        4
    );

    Ok(())
}

#[rstest]
#[tokio::test]
#[ignore = "Readonly method support not yet implemented"]
/// Readonly Errors when max fee is too small to cover inner transaction fees
async fn test_readonly_errors_when_max_fee_too_small(
    #[with(3)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    // TODO: When readonly support is added, some code will be need to force `send_inners_with_fees` to be marked as readonly.
    let TestData {
        sender_address,
        app_ids,
        fixture,
        method_selectors,
        abi_types,
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (app_id_1, app_id_2, app_id_3) = (app_ids[0], app_ids[1], app_ids[2]);

    // This tuple represents partial inner fee coverage for readonly context
    let fees_tuple = create_fees_tuple(1000, 0, 200, 0, vec![500, 0]);
    let txn_params = AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(2000), // Too small for the inner fees
            ..Default::default()
        },
        app_id: app_id_1,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.send_inners_with_fees,
            abi_types.uint64.encode(&app_id_2.into())?,
            abi_types.uint64.encode(&app_id_3.into())?,
            abi_types.fees_tuple.encode(&fees_tuple)?,
        ]),
        account_references: None,
        app_references: Some(vec![app_id_2, app_id_3]),
        asset_references: None,
        box_references: None,
    };
    composer.add_app_call(txn_params)?;

    let result = composer.send(COVER_FEES_SEND_PARAMS).await;

    assert!(
        result.is_err()
            && result
                .as_ref()
                .unwrap_err()
                .to_string()
                .contains("fees too small"),
        "Unexpected result, got: {:?}",
        result
    );

    Ok(())
}

struct TestData {
    sender_address: Address,
    app_ids: Vec<u64>,
    fixture: AlgorandFixture,
    method_selectors: MethodSelectors,
    abi_types: ABITypes,
}

type SetupResult = Result<TestData, Box<dyn std::error::Error + Send + Sync>>;
type TestResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

struct MethodSelectors {
    no_op: Vec<u8>,
    send_inners_with_fees: Vec<u8>,
    send_inners_with_fees_2: Vec<u8>,
    nested_txn_arg: Vec<u8>,
    burn_ops: Vec<u8>,
    burn_ops_readonly: Vec<u8>,
}

struct ABITypes {
    uint64: ABIType,
    fees_tuple: ABIType,
    fees_2_tuple: ABIType,
}

#[derive(Deserialize)]
struct TealSource {
    approval: String,
    clear: String,
}

#[derive(Deserialize)]
struct Arc56AppSpec {
    source: Option<TealSource>,
}

#[derive(Deserialize)]
struct Arc32AppSpec {
    source: Option<TealSource>,
}

const COVER_FEES_SEND_PARAMS: Option<SendParams> = Some(SendParams {
    cover_app_call_inner_transaction_fees: true,
    max_rounds_to_wait_for_confirmation: None,
    populate_app_call_resources: ResourcePopulation::Disabled,
});

fn get_inner_fee_teal_programs()
-> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error + Send + Sync>> {
    let app_spec: Arc56AppSpec = serde_json::from_str(inner_fee_contract::APPLICATION)?;
    let teal_source = app_spec.source.unwrap();
    let approval_bytes = BASE64_STANDARD.decode(teal_source.approval)?;
    let clear_state_bytes = BASE64_STANDARD.decode(teal_source.clear)?;
    Ok((approval_bytes, clear_state_bytes))
}

fn get_nested_app_teal_programs()
-> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error + Send + Sync>> {
    let app_spec: Arc32AppSpec = serde_json::from_str(nested_contract::APPLICATION)?;
    let teal_source = app_spec.source.unwrap();
    let approval_bytes = BASE64_STANDARD.decode(teal_source.approval)?;
    let clear_state_bytes = BASE64_STANDARD.decode(teal_source.clear)?;
    Ok((approval_bytes, clear_state_bytes))
}

async fn deploy_inner_fee_app(
    context: &AlgorandTestContext,
    note: &str,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let (approval_teal, clear_state_teal) = get_inner_fee_teal_programs()?;
    let approval_compile_result = context.algod.teal_compile(approval_teal, None).await?;
    let clear_state_compile_result = context.algod.teal_compile(clear_state_teal, None).await?;

    deploy_app(
        context,
        approval_compile_result.result,
        clear_state_compile_result.result,
        None,
        note,
    )
    .await
}

async fn deploy_nested_app(
    context: &AlgorandTestContext,
    note: &str,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let (approval_teal, clear_state_teal) = get_nested_app_teal_programs()?;
    let approval_compile_result = context.algod.teal_compile(approval_teal, None).await?;
    let clear_state_compile_result = context.algod.teal_compile(clear_state_teal, None).await?;

    let create_method = ABIMethod::from_str("createApplication()void")?;
    let create_method_selector = create_method.selector()?;

    deploy_app(
        context,
        approval_compile_result.result,
        clear_state_compile_result.result,
        Some(vec![create_method_selector]),
        note,
    )
    .await
}

async fn deploy_app(
    context: &AlgorandTestContext,
    approval_program: Vec<u8>,
    clear_state_program: Vec<u8>,
    args: Option<Vec<Vec<u8>>>,
    note: &str,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let app_create_params = AppCreateParams {
        common_params: CommonParams {
            sender: context.test_account.account()?.address(),
            note: Some(note.as_bytes().to_vec()),
            ..Default::default()
        },
        on_complete: OnApplicationComplete::NoOp,
        approval_program,
        clear_state_program,
        args,
        ..Default::default()
    };

    let mut composer = context.composer.clone();
    composer.add_app_create(app_create_params)?;

    let result = composer.send(None).await?;

    result.confirmations[0]
        .app_id
        .ok_or_else(|| "No app id returned".into())
}

// Helper function to fund app accounts
async fn fund_app_accounts(
    context: &AlgorandTestContext,
    app_ids: &Vec<u64>,
    amount: u64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut dispenser = LocalNetDispenser::new(context.algod.clone());

    for app_id in app_ids {
        let app_address = Address::from_app_id(app_id);
        dispenser
            .fund_account(&app_address.to_string(), amount)
            .await?;
    }

    Ok(())
}

async fn assert_min_fee(mut composer: Composer, params: &AppCallParams, fee: u64) {
    if fee == 1000 {
        return;
    }

    let params = AppCallParams {
        common_params: CommonParams {
            static_fee: Some(fee - 1),
            ..params.common_params.clone()
        },
        ..params.clone()
    };

    composer
        .add_app_call(params)
        .expect("Failed to add app call");

    let result = composer
        .send(Some(SendParams {
            cover_app_call_inner_transaction_fees: false,
            ..Default::default()
        }))
        .await;

    assert!(
        result.is_err()
            && result
                .as_ref()
                .unwrap_err()
                .to_string()
                .contains("fee too small"),
        "Unexpected result, got: {:?}",
        result
    );
}

fn create_fees_tuple(
    fee1: u64,
    fee2: u64,
    fee3: u64,
    fee4: u64,
    nested_fees: Vec<u64>,
) -> ABIValue {
    ABIValue::from(vec![
        ABIValue::from(fee1),
        ABIValue::from(fee2),
        ABIValue::from(fee3),
        ABIValue::from(fee4),
        ABIValue::Array(nested_fees.into_iter().map(ABIValue::from).collect()),
    ])
}
