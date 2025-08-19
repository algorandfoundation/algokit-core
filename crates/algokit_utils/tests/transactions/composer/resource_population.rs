use crate::common::init_test_logging;
use algokit_abi::{ABIMethod, ABIType, ABIValue};
use algokit_transact::Transaction;
use algokit_transact::{Address, BoxReference, OnApplicationComplete, StateSchema};
use algokit_utils::CommonParams;
use algokit_utils::transactions::composer::{ResourcePopulation, SendParams};
use algokit_utils::{AppCallParams, AppCreateParams, PaymentParams, testing::*};
use base64::{Engine, prelude::BASE64_STANDARD};
use rstest::*;
use std::str::FromStr;
use std::sync::Arc;
use std::vec;

#[fixture]
async fn setup(#[default(8)] avm_version: u8) -> SetupResult {
    init_test_logging();
    let mut fixture = algorand_fixture().await?;
    fixture.new_scope().await?;

    let context = fixture.context()?;
    let sender_address = context.test_account.account()?.address();
    let method_selectors = MethodSelectors {
        create_application: ABIMethod::from_str("createApplication()void")?.selector()?,
        bootstrap: ABIMethod::from_str("bootstrap()void")?.selector()?,
        small_box: ABIMethod::from_str("smallBox()void")?.selector()?,
        medium_box: ABIMethod::from_str("mediumBox()void")?.selector()?,
        external_app_call: ABIMethod::from_str("externalAppCall()void")?.selector()?,
        asset_total: ABIMethod::from_str("assetTotal()void")?.selector()?,
        has_asset: ABIMethod::from_str("hasAsset(address)void")?.selector()?,
        external_local: ABIMethod::from_str("externalLocal(address)void")?.selector()?,
        address_balance: ABIMethod::from_str("addressBalance(address)void")?.selector()?,
        box_with_payment: ABIMethod::from_str("boxWithPayment(pay)void")?.selector()?,
        create_asset: ABIMethod::from_str("createAsset()void")?.selector()?,
        sender_asset_balance: ABIMethod::from_str("senderAssetBalance()void")?.selector()?,
        opt_in_to_application: ABIMethod::from_str("optInToApplication()void")?.selector()?,
        error: ABIMethod::from_str("error()void")?.selector()?,
    };

    let app_id = deploy_resource_population_app(context, &method_selectors, avm_version).await?;
    fund_app_account(context, app_id, 2_334_300).await?;
    bootstrap_resource_population_app(context, &method_selectors, app_id).await?;

    let application_info = &context.algod.get_application_by_id(app_id).await?;
    let external_app_id = application_info
        .params
        .global_state
        .as_ref()
        .unwrap()
        .iter()
        .find(|kv| kv.key == "ZXh0ZXJuYWxBcHBJRA==")
        .ok_or("externalAppID not in global state")?
        .value
        .uint;

    Ok(TestData {
        sender_address,
        app_id,
        external_app_id,
        fixture,
        method_selectors,
        abi_types: ABITypes {
            address: ABIType::Address,
        },
    })
}

#[rstest]
#[case(8)]
#[case(9)]
#[tokio::test]
async fn test_accounts_errors_when_resource_population_disabled(
    #[case] _avm_version: u8,
    #[with(_avm_version)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_id,
        method_selectors,
        mut fixture,
        abi_types,
        ..
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let alice = fixture.generate_account(None).await?.account()?.address();
    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            max_fee: Some(2000),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.address_balance,
            abi_types
                .address
                .encode(&ABIValue::Address(alice.to_string()))?,
        ]),
        ..Default::default()
    })?;

    let result = composer
        .send(Some(SendParams {
            populate_app_call_resources: ResourcePopulation::Disabled,
            cover_app_call_inner_transaction_fees: true, // Ensure the same behaviour when simulating due to inner fee coverage
            ..Default::default()
        }))
        .await;

    assert!(
        result.is_err()
            && result
                .unwrap_err()
                .to_string()
                .contains("invalid Account reference")
    );

    Ok(())
}

#[rstest]
#[case(8)]
#[case(9)]
#[tokio::test]
async fn test_accounts_populated_when_resource_population_enabled(
    #[case] _avm_version: u8,
    #[with(_avm_version)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_id,
        method_selectors,
        mut fixture,
        abi_types,
        ..
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let alice = fixture.generate_account(None).await?.account()?.address();
    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.address_balance,
            abi_types
                .address
                .encode(&ABIValue::Address(alice.to_string()))?,
        ]),
        ..Default::default()
    })?;

    let result = composer.send(POPULATE_RESOURCES_SEND_PARAMS).await?;

    assert!(result.confirmations.len() == 1);
    if let Transaction::ApplicationCall(app_call) = &result.confirmations[0].txn.transaction {
        assert_eq!(app_call.account_references, Some(vec![alice]));
    } else {
        return Err("ApplicationCall transaction expected".into());
    }

    Ok(())
}

#[rstest]
#[case(8, "small")]
#[case(8, "medium")]
#[case(9, "small")]
#[case(9, "medium")]
#[tokio::test]
async fn test_boxes_errors_when_resource_population_disabled(
    #[case] _avm_version: u8,
    #[case] box_size: &str,
    #[with(_avm_version)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_id,
        method_selectors,
        fixture,
        ..
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let method_selector = match box_size {
        "small" => method_selectors.small_box,
        "medium" => method_selectors.medium_box,
        _ => return Err("Invalid box size".into()),
    };
    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selector]),
        ..Default::default()
    })?;

    let result = composer
        .send(Some(SendParams {
            populate_app_call_resources: ResourcePopulation::Disabled,
            ..Default::default()
        }))
        .await;

    assert!(
        result.is_err()
            && result
                .unwrap_err()
                .to_string()
                .contains("invalid Box reference")
    );
    Ok(())
}

#[rstest]
#[case(8, "small")]
#[case(8, "medium")]
#[case(9, "small")]
#[case(9, "medium")]
#[tokio::test]
async fn test_boxes_populated_when_resource_population_enabled(
    #[case] _avm_version: u8,
    #[case] box_size: &str,
    #[with(_avm_version)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_id,
        method_selectors,
        fixture,
        ..
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let (method_selector, box_refs) = match box_size {
        "small" => (
            method_selectors.small_box,
            vec![BoxReference {
                app_id: 0,
                name: vec![115],
            }],
        ),
        "medium" => (
            method_selectors.medium_box,
            vec![
                BoxReference {
                    app_id: 0,
                    name: vec![109],
                },
                BoxReference {
                    app_id: 0,
                    name: vec![],
                },
                BoxReference {
                    app_id: 0,
                    name: vec![],
                },
                BoxReference {
                    app_id: 0,
                    name: vec![],
                },
                BoxReference {
                    app_id: 0,
                    name: vec![],
                },
            ],
        ),
        _ => return Err("Invalid box size".into()),
    };
    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selector]),
        ..Default::default()
    })?;

    let result = composer.send(POPULATE_RESOURCES_SEND_PARAMS).await?;

    assert!(result.confirmations.len() == 1);
    if let Transaction::ApplicationCall(app_call) = &result.confirmations[0].txn.transaction {
        assert_eq!(app_call.box_references, Some(box_refs));
    } else {
        return Err("ApplicationCall transaction expected".into());
    }

    Ok(())
}

#[rstest]
#[case(8)]
#[case(9)]
#[tokio::test]
async fn test_apps_errors_when_resource_population_disabled(
    #[case] _avm_version: u8,
    #[with(_avm_version)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_id,
        method_selectors,
        fixture,
        ..
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            static_fee: Some(2000),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.external_app_call]),
        ..Default::default()
    })?;

    let result = composer
        .send(Some(SendParams {
            populate_app_call_resources: ResourcePopulation::Disabled,
            ..Default::default()
        }))
        .await;

    assert!(result.is_err() && result.unwrap_err().to_string().contains("unavailable App"));
    Ok(())
}

#[rstest]
#[case(8)]
#[case(9)]
#[tokio::test]
async fn test_apps_populated_when_resource_population_enabled(
    #[case] _avm_version: u8,
    #[with(_avm_version)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_id,
        method_selectors,
        fixture,
        ..
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            static_fee: Some(2000),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.external_app_call]),
        ..Default::default()
    })?;

    let result = composer.send(POPULATE_RESOURCES_SEND_PARAMS).await?;

    assert!(result.confirmations.len() == 1);
    if let Transaction::ApplicationCall(app_call) = &result.confirmations[0].txn.transaction {
        assert_eq!(app_call.app_references.as_ref().unwrap().len(), 1);
    } else {
        return Err("ApplicationCall transaction expected".into());
    }

    Ok(())
}

#[rstest]
#[case(8)]
#[case(9)]
#[tokio::test]
async fn test_assets_errors_when_resource_population_disabled(
    #[case] _avm_version: u8,
    #[with(_avm_version)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_id,
        method_selectors,
        fixture,
        ..
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.asset_total]),
        ..Default::default()
    })?;

    let result = composer
        .send(Some(SendParams {
            populate_app_call_resources: ResourcePopulation::Disabled,
            ..Default::default()
        }))
        .await;

    assert!(
        result.is_err()
            && result
                .unwrap_err()
                .to_string()
                .contains("unavailable Asset")
    );
    Ok(())
}

#[rstest]
#[case(8)]
#[case(9)]
#[tokio::test]
async fn test_assets_populated_when_resource_population_enabled(
    #[case] _avm_version: u8,
    #[with(_avm_version)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_id,
        method_selectors,
        fixture,
        ..
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.asset_total]),
        ..Default::default()
    })?;

    let result = composer.send(POPULATE_RESOURCES_SEND_PARAMS).await?;

    assert!(result.confirmations.len() == 1);
    if let Transaction::ApplicationCall(app_call) = &result.confirmations[0].txn.transaction {
        assert_eq!(app_call.asset_references.as_ref().unwrap().len(), 1);
    } else {
        return Err("ApplicationCall transaction expected".into());
    }

    Ok(())
}

#[rstest]
#[case(8)]
#[case(9)]
#[tokio::test]
async fn test_cross_product_assets_and_accounts_errors_when_resource_population_disabled(
    #[case] avm_version: u8,
    #[with(avm_version)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_id,
        method_selectors,
        mut fixture,
        abi_types,
        ..
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let expected_error = if avm_version == 8 {
        "invalid Account reference"
    } else {
        "unavailable Account"
    };
    let alice = fixture
        .generate_account(None)
        .await?
        .account()?
        .address()
        .to_string();
    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.has_asset,
            abi_types.address.encode(&ABIValue::Address(alice))?,
        ]),
        ..Default::default()
    })?;

    let result = composer
        .send(Some(SendParams {
            populate_app_call_resources: ResourcePopulation::Disabled,
            ..Default::default()
        }))
        .await;

    assert!(result.is_err() && result.unwrap_err().to_string().contains(expected_error));

    Ok(())
}

#[rstest]
#[case(8)]
#[case(9)]
#[tokio::test]
async fn test_cross_product_assets_and_accounts_populated_when_resource_population_enabled(
    #[case] _avm_version: u8,
    #[with(_avm_version)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_id,
        method_selectors,
        mut fixture,
        abi_types,
        ..
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let alice = fixture.generate_account(None).await?.account()?.address();
    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.has_asset,
            abi_types
                .address
                .encode(&ABIValue::Address(alice.to_string()))?,
        ]),
        ..Default::default()
    })?;

    let result = composer.send(POPULATE_RESOURCES_SEND_PARAMS).await?;

    assert!(result.confirmations.len() == 1);
    if let Transaction::ApplicationCall(app_call) = &result.confirmations[0].txn.transaction {
        assert_eq!(app_call.account_references, Some(vec![alice]));
        assert_eq!(app_call.asset_references.as_ref().unwrap().len(), 1);
    } else {
        return Err("ApplicationCall transaction expected".into());
    }

    Ok(())
}

#[rstest]
#[case(8)]
#[case(9)]
#[tokio::test]
async fn test_cross_product_account_app_errors_when_resource_population_disabled(
    #[case] avm_version: u8,
    #[with(avm_version)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_id,
        method_selectors,
        mut fixture,
        abi_types,
        ..
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let expected_error = if avm_version == 8 {
        "invalid Account reference"
    } else {
        "unavailable Account"
    };
    let alice = fixture
        .generate_account(Some(TestAccountConfig {
            initial_funds: 10_000_000,
            ..Default::default()
        }))
        .await?
        .account()?
        .address();
    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.external_local,
            abi_types
                .address
                .encode(&ABIValue::Address(alice.to_string()))?,
        ]),
        ..Default::default()
    })?;

    let result = composer
        .send(Some(SendParams {
            populate_app_call_resources: ResourcePopulation::Disabled,
            ..Default::default()
        }))
        .await;

    assert!(result.is_err() && result.unwrap_err().to_string().contains(expected_error));

    Ok(())
}

#[rstest]
#[case(8)]
#[case(9)]
#[tokio::test]
async fn test_cross_product_account_app_populated_when_resource_population_enabled(
    #[case] avm_version: u8,
    #[with(avm_version)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_id,
        external_app_id,
        method_selectors,
        mut fixture,
        abi_types,
        ..
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    let alice = fixture
        .generate_account(Some(TestAccountConfig {
            initial_funds: 1_000_000,
            ..Default::default()
        }))
        .await?;
    let alice_address = alice.account()?.address();
    let alice_signer = Arc::new(alice.clone());
    let (expected_account_refs, expected_app_refs) = if avm_version == 8 {
        (
            Some(vec![alice_address.clone()]),
            Some(vec![external_app_id]),
        )
    } else {
        (None, None)
    };

    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: alice_address.clone(),
            signer: Some(alice_signer.clone()),
            ..Default::default()
        },
        app_id: external_app_id,
        on_complete: OnApplicationComplete::OptIn,
        args: Some(vec![method_selectors.opt_in_to_application]),
        ..Default::default()
    })?;

    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.external_local,
            abi_types
                .address
                .encode(&ABIValue::Address(alice_address.to_string()))?,
        ]),
        ..Default::default()
    })?;

    let result = composer.send(POPULATE_RESOURCES_SEND_PARAMS).await?;

    assert!(result.confirmations.len() == 2);
    if let Transaction::ApplicationCall(app_call) = &result.confirmations[1].txn.transaction {
        assert_eq!(app_call.account_references, expected_account_refs);
        assert_eq!(app_call.app_references, expected_app_refs);
    } else {
        return Err("ApplicationCall transaction expected".into());
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_mixed_avm_version_same_account(
    #[with(8)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_id: avm_8_app_id,
        method_selectors,
        mut fixture,
        abi_types,
        ..
    } = setup.await?;
    let context = fixture.context()?;
    let mut composer = context.composer.clone();
    let avm_9_app_id = deploy_resource_population_app(context, &method_selectors, 9).await?;
    let alice = fixture
        .generate_account(Some(TestAccountConfig {
            initial_funds: 1_000_000,
            ..Default::default()
        }))
        .await?
        .account()?
        .address();

    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        app_id: avm_8_app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.address_balance.clone(),
            abi_types
                .address
                .encode(&ABIValue::Address(alice.to_string()))?,
        ]),
        ..Default::default()
    })?;

    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        app_id: avm_9_app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.address_balance,
            abi_types
                .address
                .encode(&ABIValue::Address(alice.to_string()))?,
        ]),
        ..Default::default()
    })?;

    let result = composer.send(POPULATE_RESOURCES_SEND_PARAMS).await?;

    assert!(result.confirmations.len() == 2);
    if let (
        Transaction::ApplicationCall(avm_8_app_call),
        Transaction::ApplicationCall(avm_9_app_call),
    ) = (
        &result.confirmations[0].txn.transaction,
        &result.confirmations[1].txn.transaction,
    ) {
        assert_eq!(avm_8_app_call.account_references, Some(vec![alice]));
        assert_eq!(avm_9_app_call.account_references, None);
    } else {
        return Err("ApplicationCall transactions expected".into());
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_mixed_avm_version_app_account(
    #[with(8)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_id: avm_8_app_id,
        external_app_id,
        method_selectors,
        fixture,
        abi_types,
        ..
    } = setup.await?;
    let context = fixture.context()?;
    let mut composer = context.composer.clone();
    let avm_9_app_id = deploy_resource_population_app(context, &method_selectors, 9).await?;

    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            static_fee: Some(2000),
            ..Default::default()
        },
        app_id: avm_8_app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.external_app_call]),
        ..Default::default()
    })?;

    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        app_id: avm_9_app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![
            method_selectors.address_balance,
            abi_types.address.encode(&ABIValue::Address(
                Address::from_app_id(&external_app_id).to_string(),
            ))?,
        ]),
        ..Default::default()
    })?;

    let result = composer.send(POPULATE_RESOURCES_SEND_PARAMS).await?;

    assert!(result.confirmations.len() == 2);
    if let (
        Transaction::ApplicationCall(avm_8_app_call),
        Transaction::ApplicationCall(avm_9_app_call),
    ) = (
        &result.confirmations[0].txn.transaction,
        &result.confirmations[1].txn.transaction,
    ) {
        assert_eq!(avm_8_app_call.app_references, Some(vec![external_app_id]));
        assert_eq!(avm_9_app_call.account_references, None);
    } else {
        return Err("ApplicationCall transactions expected".into());
    }

    Ok(())
}

#[rstest]
#[case(None)]
#[case(Some(true))]
#[case(Some(false))]
#[tokio::test]
async fn test_error(
    #[case] populate_resources: Option<bool>,
    #[with(9)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        app_id,
        method_selectors,
        fixture,
        ..
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.error]),
        ..Default::default()
    })?;

    let result = composer
        .send(Some(SendParams {
            populate_app_call_resources: match populate_resources.unwrap_or(true) {
                true => ResourcePopulation::Enabled {
                    use_access_list: false,
                },
                false => ResourcePopulation::Disabled,
            }, // Default to enabled
            ..Default::default()
        }))
        .await;

    let error_message = if populate_resources.is_none() || populate_resources.unwrap() {
        // Checks that resource population is enabled by default
        "Error analyzing group requirements via simulate in transaction 0" // Fails on simulate
    } else {
        "400 Bad Request" // Fails on send, as non population occurs
    };
    assert!(
        result.is_err()
            && result
                .as_ref()
                .unwrap_err()
                .to_string()
                .contains("logic eval error: err opcode executed")
            && result
                .as_ref()
                .unwrap_err()
                .to_string()
                .contains(error_message)
    );

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_box_with_txn_arg(
    #[with(9)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        external_app_id,
        method_selectors,
        fixture,
        ..
    } = setup.await?;
    fund_app_account(fixture.context()?, external_app_id, 106_100).await?;
    let mut composer = fixture.context()?.composer.clone();

    composer.add_payment(PaymentParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        amount: 0,
        receiver: sender_address.clone(),
    })?;

    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        app_id: external_app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.box_with_payment]),
        ..Default::default()
    })?;

    let result = composer.send(POPULATE_RESOURCES_SEND_PARAMS).await?;

    assert!(result.confirmations.len() == 2);
    if let Transaction::ApplicationCall(app_call) = &result.confirmations[1].txn.transaction {
        assert_eq!(
            app_call.box_references,
            Some(vec![BoxReference {
                app_id: 0,
                name: vec![98, 111, 120, 75, 101, 121],
            }])
        );
    } else {
        return Err("ApplicationCall transaction expected".into());
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_sender_asset_holding(
    #[with(9)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        external_app_id,
        method_selectors,
        fixture,
        ..
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    fund_app_account(fixture.context()?, external_app_id, 200_000).await?;

    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            static_fee: Some(2000),
            ..Default::default()
        },
        app_id: external_app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.create_asset]),
        ..Default::default()
    })?;

    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        app_id: external_app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.sender_asset_balance]),
        ..Default::default()
    })?;

    let result = composer.send(POPULATE_RESOURCES_SEND_PARAMS).await?;

    assert!(result.confirmations.len() == 2);
    if let Transaction::ApplicationCall(app_call) = &result.confirmations[1].txn.transaction {
        assert_eq!(app_call.account_references, None);
    } else {
        return Err("ApplicationCall transaction expected".into());
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_rekeyed_account(
    #[with(9)]
    #[future]
    setup: SetupResult,
) -> TestResult {
    let TestData {
        sender_address,
        external_app_id,
        method_selectors,
        mut fixture,
        ..
    } = setup.await?;
    let mut composer = fixture.context()?.composer.clone();
    fund_app_account(fixture.context()?, external_app_id, 200_001).await?;
    let auth_account = fixture
        .generate_account(Some(TestAccountConfig {
            initial_funds: 1_000_000,
            ..Default::default()
        }))
        .await?;
    let auth_address = auth_account.account()?.address();
    let auth_signer = Arc::new(auth_account.clone());

    // Rekey the account
    composer.add_payment(PaymentParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            rekey_to: Some(auth_address.clone()),
            ..Default::default()
        },
        amount: 0,
        receiver: sender_address.clone(),
    })?;

    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            signer: Some(auth_signer.clone()),
            static_fee: Some(2001),
            ..Default::default()
        },
        app_id: external_app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.create_asset]),
        ..Default::default()
    })?;

    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            signer: Some(auth_signer.clone()),
            ..Default::default()
        },
        app_id: external_app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.sender_asset_balance]),
        ..Default::default()
    })?;

    let result = composer.send(POPULATE_RESOURCES_SEND_PARAMS).await?;

    assert!(result.confirmations.len() == 3);
    if let Transaction::ApplicationCall(app_call) = &result.confirmations[2].txn.transaction {
        assert_eq!(app_call.account_references, None);
    } else {
        return Err("ApplicationCall transaction expected".into());
    }

    Ok(())
}

struct TestData {
    sender_address: Address,
    app_id: u64,
    external_app_id: u64,
    fixture: AlgorandFixture,
    method_selectors: MethodSelectors,
    abi_types: ABITypes,
}

type SetupResult = Result<TestData, Box<dyn std::error::Error + Send + Sync>>;
type TestResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

struct MethodSelectors {
    create_application: Vec<u8>,
    bootstrap: Vec<u8>,
    small_box: Vec<u8>,
    medium_box: Vec<u8>,
    external_app_call: Vec<u8>,
    asset_total: Vec<u8>,
    has_asset: Vec<u8>,
    external_local: Vec<u8>,
    address_balance: Vec<u8>,
    box_with_payment: Vec<u8>,
    create_asset: Vec<u8>,
    sender_asset_balance: Vec<u8>,
    opt_in_to_application: Vec<u8>,
    error: Vec<u8>,
}

struct ABITypes {
    address: ABIType,
}

// TODO: This should be shared when we have ARC56 support
#[derive(serde::Deserialize)]
struct ARC32AppSpec {
    source: Option<TealSource>,
}

#[derive(serde::Deserialize)]
struct TealSource {
    approval: String,
    clear: String,
}

const POPULATE_RESOURCES_SEND_PARAMS: Option<SendParams> = Some(SendParams {
    populate_app_call_resources: ResourcePopulation::Enabled {
        use_access_list: false,
    },
    cover_app_call_inner_transaction_fees: false,
    max_rounds_to_wait_for_confirmation: None,
});

async fn deploy_resource_population_app(
    context: &AlgorandTestContext,
    method_selectors: &MethodSelectors,
    version: u8,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let (approval_teal, clear_state_teal) = get_resource_population_programs(version).await?;
    let approval_compile_result = context.algod.teal_compile(approval_teal, None).await?;
    let clear_state_compile_result = context.algod.teal_compile(clear_state_teal, None).await?;

    let mut composer = context.composer.clone();
    composer.add_app_create(AppCreateParams {
        common_params: CommonParams {
            sender: context.test_account.account()?.address(),
            ..Default::default()
        },
        on_complete: OnApplicationComplete::NoOp,
        approval_program: approval_compile_result.result.clone(),
        clear_state_program: clear_state_compile_result.result,
        global_state_schema: Some(StateSchema {
            num_uints: 2,
            num_byte_slices: 0,
        }),
        local_state_schema: Some(StateSchema {
            num_uints: 0,
            num_byte_slices: 0,
        }),
        args: Some(vec![method_selectors.create_application.clone()]),
        ..Default::default()
    })?;
    let result = composer.send(None).await?;

    result.confirmations[0]
        .app_id
        .ok_or_else(|| "No app id returned".into())
}

async fn bootstrap_resource_population_app(
    context: &AlgorandTestContext,
    method_selectors: &MethodSelectors,
    app_id: u64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut composer = context.composer.clone();

    composer.add_app_call(AppCallParams {
        common_params: CommonParams {
            sender: context.test_account.account()?.address(),
            static_fee: Some(3000),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![method_selectors.bootstrap.clone()]),
        ..Default::default()
    })?;
    composer.send(None).await?;

    Ok(())
}

async fn fund_app_account(
    context: &AlgorandTestContext,
    app_id: u64,
    amount: u64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut dispenser = LocalNetDispenser::new(context.algod.clone());
    let app_address = Address::from_app_id(&app_id);
    dispenser
        .fund_account(&app_address.to_string(), amount)
        .await?;
    Ok(())
}

async fn get_resource_population_programs(
    version: u8,
) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error + Send + Sync>> {
    let app_spec_path = if version == 8 {
        include_str!("../../contracts/resource_population/ResourcePackerv8.arc32.json")
    } else {
        include_str!("../../contracts/resource_population/ResourcePackerv9.arc32.json")
    };

    let app_spec: ARC32AppSpec = serde_json::from_str(app_spec_path)?;
    let teal_source = app_spec.source.unwrap();
    let approval_bytes = BASE64_STANDARD.decode(teal_source.approval)?;
    let clear_state_bytes = BASE64_STANDARD.decode(teal_source.clear)?;
    Ok((approval_bytes, clear_state_bytes))
}
