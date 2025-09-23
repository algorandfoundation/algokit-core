use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture, deploy_arc56_contract};
use algokit_abi::{ABIValue, Arc56Contract};
use algokit_utils::applications::app_client::AppClientMethodCallParams;
use algokit_utils::transactions::TransactionComposerConfig;
use algokit_utils::{AlgorandClient as RootAlgorandClient, AppMethodCallArg, ResourcePopulation};
use rstest::*;
use std::collections::HashMap;
use std::sync::Arc;

fn get_nested_struct_spec() -> Arc56Contract {
    let json = algokit_test_artifacts::nested_struct_storage::APPLICATION_ARC56;
    Arc56Contract::from_json(json).expect("valid arc56")
}

fn get_nested_struct_create_application_args() -> Vec<Vec<u8>> {
    vec![vec![184u8, 68u8, 123u8, 54u8]]
}

#[rstest]
#[tokio::test]
async fn test_nested_structs_described_by_structure(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

    let spec = get_nested_struct_spec();
    let app_id = deploy_arc56_contract(
        &fixture,
        &sender,
        &spec,
        None,
        None,
        Some(get_nested_struct_create_application_args()),
    )
    .await?;

    let mut algorand = RootAlgorandClient::default_localnet(None);
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let app_client = algokit_utils::applications::app_client::AppClient::new(
        algokit_utils::applications::app_client::AppClientParams {
            app_id,
            app_spec: spec,
            algorand: algorand.into(),
            app_name: None,
            default_sender: Some(sender.to_string()),
            default_signer: None,
            source_maps: None,
            transaction_composer_config: Some(TransactionComposerConfig {
                populate_app_call_resources: ResourcePopulation::Enabled {
                    use_access_list: false,
                },
                ..Default::default()
            }),
        },
    );

    app_client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "setValue".to_string(),
                args: vec![
                    AppMethodCallArg::ABIValue(ABIValue::from(1u64)),
                    AppMethodCallArg::ABIValue(ABIValue::from("hello")),
                ],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    let result = app_client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "getValue".to_string(),
                args: vec![AppMethodCallArg::ABIValue(ABIValue::from(1u64))],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    let abi_ret = result.abi_return.expect("abi return");
    let value = abi_ret.return_value.expect("decoded value");
    match value {
        ABIValue::Struct(ref outer) => {
            let x = match outer.get("x").expect("x") {
                ABIValue::Struct(m) => m,
                _ => return Err("x should be a struct".into()),
            };
            match x.get("a").expect("a") {
                ABIValue::String(s) => assert_eq!(s, "hello"),
                _ => return Err("a should be string".into()),
            }
        }
        _ => return Err("expected struct return".into()),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_nested_structs_referenced_by_name(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

    let mut spec = get_nested_struct_spec();
    spec.structs = HashMap::from([
        (
            "Struct1".to_string(),
            vec![algokit_abi::arc56_contract::StructField {
                name: "a".to_string(),
                field_type: algokit_abi::arc56_contract::StructFieldType::Value(
                    "string".to_string(),
                ),
            }],
        ),
        (
            "Struct2".to_string(),
            vec![algokit_abi::arc56_contract::StructField {
                name: "x".to_string(),
                field_type: algokit_abi::arc56_contract::StructFieldType::Value(
                    "Struct1".to_string(),
                ),
            }],
        ),
    ]);

    let app_id = deploy_arc56_contract(
        &fixture,
        &sender,
        &spec,
        None,
        None,
        Some(get_nested_struct_create_application_args()),
    )
    .await?;

    let mut algorand = RootAlgorandClient::default_localnet(None);
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let app_client = algokit_utils::applications::app_client::AppClient::new(
        algokit_utils::applications::app_client::AppClientParams {
            app_id,
            app_spec: spec,
            algorand: algorand.into(),
            app_name: None,
            default_sender: Some(sender.to_string()),
            default_signer: None,
            source_maps: None,
            transaction_composer_config: None,
        },
    );

    app_client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "setValue".to_string(),
                args: vec![
                    AppMethodCallArg::ABIValue(ABIValue::from(1u64)),
                    AppMethodCallArg::ABIValue(ABIValue::from("hello")),
                ],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    let result = app_client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "getValue".to_string(),
                args: vec![AppMethodCallArg::ABIValue(ABIValue::from(1u64))],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    let abi_ret = result.abi_return.expect("abi return");
    let value = abi_ret.return_value.expect("decoded value");
    match value {
        ABIValue::Struct(ref outer) => {
            let x = match outer.get("x").expect("x") {
                ABIValue::Struct(m) => m,
                _ => return Err("x should be a struct".into()),
            };
            match x.get("a").expect("a") {
                ABIValue::String(s) => assert_eq!(s, "hello"),
                _ => return Err("a should be string".into()),
            }
        }
        _ => return Err("expected struct return".into()),
    }

    Ok(())
}
