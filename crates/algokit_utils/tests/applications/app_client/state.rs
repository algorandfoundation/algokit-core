use crate::common::TestResult;
use crate::common::app_fixture::{
    boxmap_app_fixture, testing_app_fixture, testing_app_puya_fixture,
};
use algokit_abi::{ABIType, ABIValue};
use algokit_transact::BoxReference;
// client params not needed with fixtures
use algokit_utils::AppMethodCallArg;
use algokit_utils::applications::app_client::{AppClientMethodCallParams, FundAppAccountParams};
use algokit_utils::clients::app_manager::{AppState, BoxName};
use base64::{Engine, engine::general_purpose::STANDARD as Base64};
use num_bigint::BigUint;
use rstest::*;
use std::collections::HashMap;
use std::str::FromStr;

#[rstest]
#[tokio::test]
async fn test_global_state_retrieval(
    #[future] testing_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let f = testing_app_fixture.await?;
    let sender = f.sender_address;
    let client = f.client;

    client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "set_global".to_string(),
                args: vec![
                    AppMethodCallArg::ABIValue(ABIValue::from(1u64)),
                    AppMethodCallArg::ABIValue(ABIValue::from(2u64)),
                    AppMethodCallArg::ABIValue(ABIValue::from("asdf")),
                    AppMethodCallArg::ABIValue(ABIValue::Array(vec![
                        ABIValue::from_byte(1),
                        ABIValue::from_byte(2),
                        ABIValue::from_byte(3),
                        ABIValue::from_byte(4),
                    ])),
                ],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    let global_state = client.get_global_state().await?;

    assert!(global_state.contains_key("int1".as_bytes()));
    assert!(global_state.contains_key("int2".as_bytes()));
    assert!(global_state.contains_key("bytes1".as_bytes()));
    assert!(global_state.contains_key("bytes2".as_bytes()));

    let mut keys: Vec<String> = global_state
        .keys()
        .map(|k| String::from_utf8_lossy(k).to_string())
        .collect();
    keys.sort();
    assert_eq!(keys, vec!["bytes1", "bytes2", "int1", "int2", "value"]);

    match global_state.get("int1".as_bytes()).unwrap() {
        AppState::Uint(state) => assert_eq!(state.value, 1),
        _ => return Err("Expected uint state".into()),
    }

    match global_state.get("int2".as_bytes()).unwrap() {
        AppState::Uint(state) => assert_eq!(state.value, 2),
        _ => return Err("Expected uint state".into()),
    }

    match global_state.get("bytes1".as_bytes()).unwrap() {
        AppState::Bytes(state) => {
            assert_eq!(String::from_utf8(state.value_raw.clone()).unwrap(), "asdf");
        }
        _ => return Err("Expected bytes state".into()),
    }

    match global_state.get("bytes2".as_bytes()).unwrap() {
        AppState::Bytes(state) => {
            assert_eq!(state.value_raw, vec![1, 2, 3, 4]);
        }
        _ => return Err("Expected bytes state".into()),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_local_state_retrieval(
    #[future] testing_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let f = testing_app_fixture.await?;
    let sender = f.sender_address;
    let client = f.client;

    client
        .send()
        .opt_in(
            AppClientMethodCallParams {
                method: "opt_in".to_string(),
                args: vec![],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
        )
        .await?;

    client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "set_local".to_string(),
                args: vec![
                    AppMethodCallArg::ABIValue(ABIValue::from(1u64)),
                    AppMethodCallArg::ABIValue(ABIValue::from(2u64)),
                    AppMethodCallArg::ABIValue(ABIValue::from("asdf")),
                    AppMethodCallArg::ABIValue(ABIValue::Array(vec![
                        ABIValue::from_byte(1),
                        ABIValue::from_byte(2),
                        ABIValue::from_byte(3),
                        ABIValue::from_byte(4),
                    ])),
                ],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    let local_state = client.get_local_state(&sender.to_string()).await?;

    assert!(local_state.contains_key("local_int1".as_bytes()));
    assert!(local_state.contains_key("local_int2".as_bytes()));
    assert!(local_state.contains_key("local_bytes1".as_bytes()));
    assert!(local_state.contains_key("local_bytes2".as_bytes()));

    let mut keys: Vec<String> = local_state
        .keys()
        .map(|k| String::from_utf8_lossy(k).to_string())
        .collect();
    keys.sort();
    assert_eq!(
        keys,
        vec!["local_bytes1", "local_bytes2", "local_int1", "local_int2"]
    );

    match local_state.get("local_int1".as_bytes()).unwrap() {
        AppState::Uint(state) => assert_eq!(state.value, 1),
        _ => return Err("Expected uint state".into()),
    }

    match local_state.get("local_int2".as_bytes()).unwrap() {
        AppState::Uint(state) => assert_eq!(state.value, 2),
        _ => return Err("Expected uint state".into()),
    }

    match local_state.get("local_bytes1".as_bytes()).unwrap() {
        AppState::Bytes(state) => {
            assert_eq!(String::from_utf8(state.value_raw.clone()).unwrap(), "asdf");
        }
        _ => return Err("Expected bytes state".into()),
    }

    match local_state.get("local_bytes2".as_bytes()).unwrap() {
        AppState::Bytes(state) => {
            assert_eq!(state.value_raw, vec![1, 2, 3, 4]);
        }
        _ => return Err("Expected bytes state".into()),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_box_retrieval(
    #[future] testing_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let f = testing_app_fixture.await?;
    let sender = f.sender_address;
    let client = f.client;

    let box_name1: Vec<u8> = vec![0, 0, 0, 1];
    let box_name2: Vec<u8> = vec![0, 0, 0, 2];

    client
        .fund_app_account(
            FundAppAccountParams {
                amount: 1_000_000,
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
        )
        .await?;

    client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "set_box".to_string(),
                args: vec![
                    AppMethodCallArg::ABIValue(ABIValue::Array(
                        box_name1.iter().copied().map(ABIValue::from_byte).collect(),
                    )),
                    AppMethodCallArg::ABIValue(ABIValue::from("value1")),
                ],
                sender: Some(sender.to_string()),
                box_references: Some(vec![BoxReference {
                    app_id: 0,
                    name: box_name1.clone(),
                }]),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "set_box".to_string(),
                args: vec![
                    AppMethodCallArg::ABIValue(ABIValue::Array(
                        box_name2.iter().copied().map(ABIValue::from_byte).collect(),
                    )),
                    AppMethodCallArg::ABIValue(ABIValue::from("value2")),
                ],
                sender: Some(sender.to_string()),
                box_references: Some(vec![BoxReference {
                    app_id: 0,
                    name: box_name2.clone(),
                }]),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    let box_names = client.get_box_names().await?;
    let names: Vec<Vec<u8>> = box_names.iter().map(|n| n.name_raw.clone()).collect();
    assert!(names.contains(&box_name1));
    assert!(names.contains(&box_name2));

    let box_values = client.get_box_values().await?;
    let box1_value = client.get_box_value(&box_name1).await?;

    let box_name1_base64 = Base64.encode(&box_name1);
    let box_name2_base64 = Base64.encode(&box_name2);

    let mut box_names_base64: Vec<_> = box_values.iter().map(|b| &b.name.name_base64).collect();
    box_names_base64.sort();
    let mut expected_names = vec![&box_name1_base64, &box_name2_base64];
    expected_names.sort();
    assert_eq!(box_names_base64, expected_names);

    let box1 = box_values
        .iter()
        .find(|b| b.name.name_base64 == box_name1_base64)
        .expect("box1 should exist");
    assert_eq!(box1.value, b"value1");
    assert_eq!(box1_value, box1.value);

    let box2 = box_values
        .iter()
        .find(|b| b.name.name_base64 == box_name2_base64)
        .expect("box2 should exist");
    assert_eq!(box2.value, b"value2");

    let expected_value_decoded = "1234524352";
    let expected_value = format!("\x00\n{}", expected_value_decoded);

    client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "set_box".to_string(),
                args: vec![
                    AppMethodCallArg::ABIValue(ABIValue::Array(
                        box_name1.iter().copied().map(ABIValue::from_byte).collect(),
                    )),
                    AppMethodCallArg::ABIValue(ABIValue::from(expected_value.as_str())),
                ],
                sender: Some(sender.to_string()),
                box_references: Some(vec![BoxReference {
                    app_id: 0,
                    name: box_name1.clone(),
                }]),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    let abi_string_type = "string".parse::<ABIType>().unwrap();
    let box_name1_base64_for_filter = box_name1_base64.clone();
    let boxes_abi = client
        .get_box_values_from_abi_type(
            &abi_string_type,
            Some(Box::new(move |name: &BoxName| {
                name.name_base64 == box_name1_base64_for_filter
            })),
        )
        .await?;

    let box1_abi_value = client
        .get_box_value_from_abi_type(&box_name1, &abi_string_type)
        .await?;

    assert_eq!(boxes_abi.len(), 1);
    if let ABIValue::String(decoded_str) = &boxes_abi[0].value {
        assert_eq!(decoded_str, expected_value_decoded);
    } else {
        return Err("Expected string ABIValue".into());
    }

    if let ABIValue::String(decoded_str) = &box1_abi_value {
        assert_eq!(decoded_str, expected_value_decoded);
    } else {
        return Err("Expected string ABIValue".into());
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_box_maps(
    #[future] boxmap_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let f = boxmap_app_fixture.await?;
    let sender = f.sender_address;
    let app_client = f.client;

    app_client
        .fund_app_account(
            FundAppAccountParams {
                amount: 1_000_000,
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
        )
        .await?;

    let _result = app_client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "setValue".to_string(),
                args: vec![
                    AppMethodCallArg::ABIValue(ABIValue::Uint(BigUint::from(1u64))),
                    AppMethodCallArg::ABIValue(ABIValue::String("foo".to_string())),
                ],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await;

    let box_map = app_client.state().box_storage().get_map("bMap").await?;
    assert_eq!(box_map.len(), 1);

    let key = ABIValue::Uint(BigUint::from(1u64));
    let expected_value = ABIValue::String("foo".to_string());
    assert_eq!(box_map.get(&key), Some(&expected_value));

    let box_map_value = app_client
        .state()
        .box_storage()
        .get_map_value("bMap", &ABIValue::Uint(BigUint::from(1u64)))
        .await?;
    assert_eq!(box_map_value, Some(expected_value));

    Ok(())
}

#[rstest]
#[tokio::test]
async fn box_methods_with_manually_encoded_abi_args(
    #[future] testing_app_puya_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let f = testing_app_puya_fixture.await?;
    let sender = f.sender_address;
    let client = f.client;

    client
        .fund_app_account(
            FundAppAccountParams {
                amount: 1_000_000,
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
        )
        .await?;

    let box_prefix = b"box_bytes".to_vec();
    let name_type = ABIType::String;
    let box_name = "asdf";
    let box_name_encoded = name_type.encode(&ABIValue::from(box_name)).unwrap();
    let box_identifier = {
        let mut v = box_prefix.clone();
        v.extend_from_slice(&box_name_encoded);
        v
    };

    let value_type = ABIType::DynamicArray(Box::new(ABIType::Byte));
    let encoded = value_type
        .encode(&ABIValue::from(vec![
            ABIValue::from_byte(116),
            ABIValue::from_byte(101),
            ABIValue::from_byte(115),
            ABIValue::from_byte(116),
            ABIValue::from_byte(95),
            ABIValue::from_byte(98),
            ABIValue::from_byte(121),
            ABIValue::from_byte(116),
            ABIValue::from_byte(101),
            ABIValue::from_byte(115),
        ]))
        .unwrap();

    client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "set_box_bytes".to_string(),
                args: vec![
                    AppMethodCallArg::ABIValue(ABIValue::from("asdf")),
                    AppMethodCallArg::ABIValue(ABIValue::Array(
                        encoded.into_iter().map(ABIValue::from_byte).collect(),
                    )),
                ],
                sender: Some(sender.to_string()),
                box_references: Some(vec![BoxReference {
                    app_id: 0,
                    name: box_identifier.clone(),
                }]),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    let retrieved = client
        .algorand()
        .app()
        .get_box_value_from_abi_type(client.app_id(), &box_identifier, &value_type)
        .await?;
    assert_eq!(
        retrieved,
        ABIValue::Array(vec![
            ABIValue::from_byte(116),
            ABIValue::from_byte(101),
            ABIValue::from_byte(115),
            ABIValue::from_byte(116),
            ABIValue::from_byte(95),
            ABIValue::from_byte(98),
            ABIValue::from_byte(121),
            ABIValue::from_byte(116),
            ABIValue::from_byte(101),
            ABIValue::from_byte(115),
        ])
    );

    Ok(())
}

#[rstest]
#[tokio::test]
async fn box_methods_with_arc4_returns_parametrized(
    #[future] testing_app_puya_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let f = testing_app_puya_fixture.await?;
    let sender = f.sender_address;
    let client = f.client;

    client
        .fund_app_account(
            FundAppAccountParams {
                amount: 1_000_000,
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
        )
        .await?;

    let mut big = num_bigint::BigUint::from(1u64);
    big <<= 256u32;
    let cases: Vec<(Vec<u8>, &str, &str, ABIValue)> = vec![
        (
            b"box_str".to_vec(),
            "set_box_str",
            "string",
            ABIValue::from("string"),
        ),
        (
            b"box_int".to_vec(),
            "set_box_int",
            "uint32",
            ABIValue::from(123u32),
        ),
        (
            b"box_int512".to_vec(),
            "set_box_int512",
            "uint512",
            ABIValue::from(big),
        ),
        (
            b"box_static".to_vec(),
            "set_box_static",
            "byte[4]",
            ABIValue::Array(vec![
                ABIValue::from_byte(1),
                ABIValue::from_byte(2),
                ABIValue::from_byte(3),
                ABIValue::from_byte(4),
            ]),
        ),
        (
            b"".to_vec(),
            "set_struct",
            "(string,uint64)",
            ABIValue::Array(vec![ABIValue::from("box1"), ABIValue::from(123u64)]),
        ),
    ];

    for (box_prefix, method_sig, value_type_str, arg_val) in cases {
        let name_type = ABIType::String;
        let name_encoded = name_type.encode(&ABIValue::from("box1")).unwrap();
        let mut box_reference = box_prefix.clone();
        box_reference.extend_from_slice(&name_encoded);

        let method_arg_val = if method_sig == "set_struct" {
            ABIValue::Struct(HashMap::from([
                ("name".to_string(), ABIValue::from("box1")),
                ("id".to_string(), ABIValue::from(123u64)),
            ]))
        } else {
            arg_val.clone()
        };

        client
            .send()
            .call(
                AppClientMethodCallParams {
                    method: method_sig.to_string(),
                    args: vec![
                        AppMethodCallArg::ABIValue(ABIValue::from("box1")),
                        AppMethodCallArg::ABIValue(method_arg_val),
                    ],
                    sender: Some(sender.to_string()),
                    box_references: Some(vec![BoxReference {
                        app_id: 0,
                        name: box_reference.clone(),
                    }]),
                    ..Default::default()
                },
                None,
                None,
            )
            .await?;

        let expected_raw = algokit_abi::ABIType::from_str(value_type_str)
            .unwrap()
            .encode(&arg_val)
            .unwrap();
        let actual_raw = client.get_box_value(&box_reference).await?;
        assert_eq!(actual_raw, expected_raw);

        let decoded = client
            .get_box_value_from_abi_type(
                &box_reference,
                &ABIType::from_str(value_type_str).unwrap(),
            )
            .await?;
        assert_eq!(decoded, arg_val);

        let box_name_for_filter = box_reference.clone();
        let values = client
            .get_box_values_from_abi_type(
                &ABIType::from_str(value_type_str).unwrap(),
                Some(Box::new(move |name: &BoxName| {
                    name.name_raw == box_name_for_filter
                })),
            )
            .await?;
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].value, decoded);
    }

    Ok(())
}
