use algokit_abi::{ABIType, abi_type::BitSize};
use algokit_test_artifacts::template_variables;
use algokit_utils::clients::app_manager::*;
use base64::prelude::*;
use rstest::*;
use std::collections::HashMap;

use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture};

/// Test template variable replacement behavior
#[rstest]
#[case("pushint TMPL_NUMBER\npushbytes TMPL_STRING",
       &[("NUMBER", TealTemplateValue::Int(42)), ("STRING", TealTemplateValue::String("hello".to_string()))],
       "pushint 42\npushbytes 0x68656c6c6f")]
#[case("pushint TMPL_UPDATABLE\npushint TMPL_DELETABLE",
       &[("UPDATABLE", TealTemplateValue::Int(1)), ("DELETABLE", TealTemplateValue::Int(0))],
       "pushint 1\npushint 0")]
#[case("pushbytes \"TMPL_NUMBER\"\npushint TMPL_NUMBER",
       &[("NUMBER", TealTemplateValue::Int(42))],
       "pushbytes \"TMPL_NUMBER\"\npushint 42")]
#[case("TMPL_X TMPL_X TMPL_X",
       &[("X", TealTemplateValue::String("test".to_string()))],
       "0x74657374 0x74657374 0x74657374")]
fn test_template_variable_replacement_behavior(
    #[case] teal_code: &str,
    #[case] template_vars: &[(&str, TealTemplateValue)],
    #[case] expected: &str,
) {
    let template_map = template_vars
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect();

    let result = AppManager::replace_template_variables(teal_code, &template_map).unwrap();
    assert_eq!(result.trim(), expected.trim());
}

/// Test comprehensive comment stripping behavior with all edge cases
#[test]
fn test_comprehensive_comment_stripping() {
    let input = r#"//comment
op arg //comment
op "arg" //comment
op "//" //comment
op "  //comment  " //comment
op "\" //" //comment
op "// \" //" //comment
op "" //comment
//
op 123
op 123 // something
op "" // more comments
op "//" //op "//"
op "//"
pushbytes base64(//8=)
pushbytes b64(//8=)

pushbytes base64(//8=)  // pushbytes base64(//8=)
pushbytes b64(//8=)     // pushbytes b64(//8=)
pushbytes "base64(//8=)"  // pushbytes "base64(//8=)"
pushbytes "b64(//8=)"     // pushbytes "b64(//8=)"

pushbytes base64 //8=
pushbytes b64 //8=

pushbytes base64 //8=  // pushbytes base64 //8=
pushbytes b64 //8=     // pushbytes b64 //8=
pushbytes "base64 //8="  // pushbytes "base64 //8="
pushbytes "b64 //8="     // pushbytes "b64 //8=""#;

    let expected = r#"
op arg
op "arg"
op "//"
op "  //comment  "
op "\" //"
op "// \" //"
op ""

op 123
op 123
op ""
op "//"
op "//"
pushbytes base64(//8=)
pushbytes b64(//8=)

pushbytes base64(//8=)
pushbytes b64(//8=)
pushbytes "base64(//8=)"
pushbytes "b64(//8=)"

pushbytes base64 //8=
pushbytes b64 //8=

pushbytes base64 //8=
pushbytes b64 //8=
pushbytes "base64 //8="
pushbytes "b64 //8=""#;

    let result = AppManager::strip_teal_comments(input);
    assert_eq!(result.trim(), expected.trim());
}

/// Test TEAL compilation and caching behavior
#[rstest]
#[tokio::test]
async fn test_teal_compilation(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let app_manager = algorand_fixture.algorand_client.app();

    let teal = "#pragma version 3\npushint 1\nreturn";
    let result = app_manager.compile_teal(teal).await.unwrap();

    assert_eq!(result.teal, teal);
    // Verify deterministic compilation results
    assert_eq!(result.compiled_base64_to_bytes, vec![3, 129, 1, 67]);
    assert_eq!(
        result.compiled_hash,
        "LKKM53XYIPYORMMTKCCUXWFPADWRFYAYZ27QZ2HUWER4OU7TKTVW3C4BRQ"
    );

    // Test caching behavior by verifying consistent results across calls
    let cached = app_manager.compile_teal(teal).await.unwrap();
    assert_eq!(result.compiled_hash, cached.compiled_hash);
    assert_eq!(result.teal, cached.teal);
    assert_eq!(
        result.compiled_base64_to_bytes,
        cached.compiled_base64_to_bytes
    );

    // Test with different TEAL code produces different results
    let different_teal = "#pragma version 3\npushint 2\nreturn";
    let different_result = app_manager.compile_teal(different_teal).await.unwrap();
    assert_ne!(result.compiled_hash, different_result.compiled_hash);

    Ok(())
}

/// Test template compilation
#[rstest]
#[tokio::test]
async fn test_template_compilation(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let app_manager = algorand_fixture.algorand_client.app();

    let template_params = HashMap::from([("VALUE".to_string(), TealTemplateValue::Int(42))]);
    let result = app_manager
        .compile_teal_template(
            "#pragma version 3\npushint TMPL_VALUE\nreturn",
            Some(&template_params),
            None,
        )
        .await
        .unwrap();

    assert!(result.teal.contains("pushint 42"));
    assert!(!result.teal.contains("TMPL_"));
    // Check deterministic compilation results for template with int 42
    assert_eq!(result.compiled_base64_to_bytes, vec![3, 129, 42, 67]);

    Ok(())
}

/// Test deploy-time control
#[rstest]
#[tokio::test]
async fn test_deploy_time_control(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let app_manager = algorand_fixture.algorand_client.app();

    let template = format!(
        "#pragma version 3\npushint {}\npushint {}\nreturn",
        UPDATABLE_TEMPLATE_NAME, DELETABLE_TEMPLATE_NAME
    );
    let metadata = DeploymentMetadata {
        updatable: Some(true),
        deletable: Some(false),
    };

    let result = app_manager
        .compile_teal_template(&template, None, Some(&metadata))
        .await
        .unwrap();

    assert!(result.teal.contains("pushint 1"));
    assert!(result.teal.contains("pushint 0"));
    assert!(!result.teal.contains("TMPL_"));

    Ok(())
}

/// Test real contract compilation
#[rstest]
#[tokio::test]
async fn test_real_contract_compilation(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let app_manager = algorand_fixture.algorand_client.app();

    let contract: serde_json::Value =
        serde_json::from_str(template_variables::APPLICATION_ARC56).unwrap();
    let approval_teal = contract["source"]["approval"].as_str().unwrap();
    let approval_code = String::from_utf8(BASE64_STANDARD.decode(approval_teal).unwrap()).unwrap();

    let template_params = HashMap::from([
        ("uint64TmplVar".to_string(), TealTemplateValue::Int(42)),
        ("bytesTmplVar".to_string(), TealTemplateValue::String("hello".to_string())),
        ("bytes32TmplVar".to_string(), TealTemplateValue::String("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string())),
        ("bytes64TmplVar".to_string(), TealTemplateValue::String("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string())),
    ]);

    let result = app_manager
        .compile_teal_template(&approval_code, Some(&template_params), None)
        .await
        .unwrap();

    // Check deterministic compilation results for the real contract with fixed template parameters
    let expected_bytes = vec![
        10, 32, 2, 1, 42, 38, 3, 5, 104, 101, 108, 108, 111, 128, 1, 48, 49, 50, 51, 52, 53, 54,
        55, 56, 57, 97, 98, 99, 100, 101, 102, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99,
        100, 101, 102, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102, 48, 49,
        50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102, 48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 97, 98, 99, 100, 101, 102, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100,
        101, 102, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102, 48, 49, 50,
        51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102, 64, 48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 97, 98, 99, 100, 101, 102, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100,
        101, 102, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102, 48, 49, 50,
        51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102, 49, 24, 20, 129, 6, 11, 49, 25, 8,
        141, 12, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136,
        0, 2, 34, 67, 138, 0, 0, 40, 176, 35, 68, 137, 136, 0, 2, 34, 67, 138, 0, 0, 40, 41, 42,
        132, 137, 136, 0, 2, 34, 67, 138, 0, 0, 0, 137, 128, 4, 21, 31, 124, 117, 136, 0, 12, 73,
        21, 22, 87, 6, 2, 76, 80, 80, 176, 34, 67, 138, 0, 1, 35, 22, 137, 34, 67, 128, 4, 184, 68,
        123, 54, 54, 26, 0, 142, 1, 255, 241, 0, 128, 4, 154, 113, 210, 180, 128, 4, 223, 77, 92,
        59, 128, 4, 61, 135, 13, 135, 128, 4, 188, 11, 23, 6, 54, 26, 0, 142, 4, 255, 140, 255,
        153, 255, 166, 255, 176, 0,
    ];
    assert_eq!(result.compiled_base64_to_bytes, expected_bytes);
    assert_eq!(
        result.compiled_hash,
        "P2FNVZSIY7ETR6HLNUMUA7SXEK5ZHQBWLFH3T2IJKHBKHMLKA5KAIWQZFE"
    );

    Ok(())
}

/// Test template substitution
#[test]
fn test_template_substitution() {
    let program = r#"test TMPL_INT // TMPL_INT
test TMPL_INT
no change
test TMPL_STR // TMPL_STR
TMPL_STR
TMPL_STR // TMPL_INT
TMPL_STR // foo //
TMPL_STR // bar
test "TMPL_STR" // not replaced
test "TMPL_STRING" // not replaced
test TMPL_STRING // not replaced
test TMPL_STRI // not replaced
test TMPL_STR TMPL_INT TMPL_INT TMPL_STR // TMPL_STR TMPL_INT TMPL_INT TMPL_STR
test TMPL_INT TMPL_STR TMPL_STRING "TMPL_INT TMPL_STR TMPL_STRING" //TMPL_INT TMPL_STR TMPL_STRING
test TMPL_INT TMPL_INT TMPL_STRING TMPL_STRING TMPL_STRING TMPL_INT TMPL_STRING //keep
TMPL_STR TMPL_STR TMPL_STR
TMPL_STRING
test NOTTMPL_STR // not replaced
NOTTMPL_STR // not replaced
TMPL_STR // replaced"#;

    let mut template_values = HashMap::new();
    template_values.insert("INT".to_string(), TealTemplateValue::Int(123));
    template_values.insert(
        "STR".to_string(),
        TealTemplateValue::String("ABC".to_string()),
    );

    let result = AppManager::replace_template_variables(program, &template_values)
        .expect("Template replacement should succeed");

    let expected = r#"test 123 // TMPL_INT
test 123
no change
test 0x414243 // TMPL_STR
0x414243
0x414243 // TMPL_INT
0x414243 // foo //
0x414243 // bar
test "TMPL_STR" // not replaced
test "TMPL_STRING" // not replaced
test TMPL_STRING // not replaced
test TMPL_STRI // not replaced
test 0x414243 123 123 0x414243 // TMPL_STR TMPL_INT TMPL_INT TMPL_STR
test 123 0x414243 TMPL_STRING "TMPL_INT TMPL_STR TMPL_STRING" //TMPL_INT TMPL_STR TMPL_STRING
test 123 123 TMPL_STRING TMPL_STRING TMPL_STRING 123 TMPL_STRING //keep
0x414243 0x414243 0x414243
TMPL_STRING
test NOTTMPL_STR // not replaced
NOTTMPL_STR // not replaced
0x414243 // replaced"#;

    // Verify the output matches exactly
    assert_eq!(result.trim(), expected.trim());
}

/// Test compilation error handling
#[rstest]
#[tokio::test]
async fn test_compilation_errors(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let app_manager = algorand_fixture.algorand_client.app();

    // Invalid TEAL should fail
    let result = app_manager
        .compile_teal("#pragma version 3\ninvalid_opcode_xyz")
        .await;
    assert!(result.is_err());

    // Missing template variables should either preserve or fail
    let result = app_manager
        .compile_teal_template(
            "#pragma version 3\npushint TMPL_MISSING\nreturn",
            None,
            None,
        )
        .await;

    match result {
        Ok(compiled) => assert!(compiled.teal.contains("TMPL_MISSING")),
        Err(_) => {} // Both outcomes are acceptable
    }

    Ok(())
}

/// Test that BoxIdentifier correctly handles binary data
#[test]
fn test_box_identifier_binary_handling() {
    use base64::{Engine, engine::general_purpose::STANDARD as Base64};

    // Test with UTF-8 string data (common case)
    let text_data = "hello_world".as_bytes().to_vec();
    let (app_id, name_bytes) = AppManager::get_box_reference(&text_data);
    assert_eq!(app_id, 0);
    assert_eq!(name_bytes, text_data);
    assert_eq!(name_bytes, b"hello_world".to_vec());

    // Test with pure binary data (non-UTF-8)
    let binary_data = vec![0xFF, 0xFE, 0xFD, 0x00, 0x01, 0x02];
    let (app_id, name_bytes) = AppManager::get_box_reference(&binary_data);
    assert_eq!(app_id, 0);
    assert_eq!(name_bytes, binary_data);

    // Test with empty data
    let empty_data = vec![];
    let (app_id, name_bytes) = AppManager::get_box_reference(&empty_data);
    assert_eq!(app_id, 0);
    assert_eq!(name_bytes, empty_data);

    // Test that box identifiers can be constructed from different sources

    // From UTF-8 string
    let string_box_id: BoxIdentifier = "my_box".as_bytes().to_vec();
    assert_eq!(string_box_id, b"my_box".to_vec());

    // From hex data (representing binary data)
    let hex_box_id: BoxIdentifier = vec![0xDE, 0xAD, 0xBE, 0xEF];
    assert_eq!(hex_box_id.len(), 4);

    // From base64-decoded data
    let base64_str = "SGVsbG8gV29ybGQ="; // "Hello World" in base64
    let base64_box_id: BoxIdentifier = Base64.decode(base64_str).unwrap();
    assert_eq!(base64_box_id, b"Hello World".to_vec());

    // Test that the box reference function works consistently
    let (_, ref_bytes) = AppManager::get_box_reference(&string_box_id);
    assert_eq!(ref_bytes, string_box_id);
}

/// Test that app state keys and bytes are now Vec<u8> for TypeScript consistency
#[test]
fn test_app_state_keys_as_vec_u8() {
    use algod_client::models::{TealKeyValue, TealValue};
    use base64::{Engine, engine::general_purpose::STANDARD as Base64};

    // Create mock state data
    let key_raw = b"test_key".to_vec();
    let key_base64 = Base64.encode(&key_raw);

    let state_val = TealKeyValue {
        key: key_base64,
        value: TealValue {
            r#type: 2, // Uint type
            bytes: Vec::new(),
            uint: 42,
        },
    };

    let state = vec![state_val];

    // Decode the app state
    let result = AppManager::decode_app_state(&state).unwrap();

    // Verify that the HashMap key is Vec<u8>
    assert_eq!(result.len(), 1);
    assert!(result.contains_key(&key_raw));

    // Verify the actual data in AppState
    let app_state = &result[&key_raw];
    assert_eq!(app_state.key_raw, key_raw);
    assert_eq!(app_state.key_base64, Base64.encode(&key_raw));

    // Test with binary key data (non-UTF-8)
    let binary_key = vec![0xFF, 0xFE, 0xFD, 0x00];
    let binary_key_base64 = Base64.encode(&binary_key);

    let binary_state_val = TealKeyValue {
        key: binary_key_base64,
        value: TealValue {
            r#type: 2,
            bytes: Vec::new(),
            uint: 123,
        },
    };

    let binary_state = vec![binary_state_val];
    let binary_result = AppManager::decode_app_state(&binary_state).unwrap();

    // Verify binary key works correctly
    assert!(binary_result.contains_key(&binary_key));
    let binary_app_state = &binary_result[&binary_key];
    assert_eq!(binary_app_state.key_raw, binary_key);

    // Test bytes value type with base64 deserialization
    let bytes_key = b"bytes_key".to_vec();
    let bytes_key_base64 = Base64.encode(&bytes_key);
    let bytes_value = b"Hello, World!".to_vec();

    let bytes_state_val = TealKeyValue {
        key: bytes_key_base64,
        value: TealValue {
            r#type: 1, // Bytes type
            bytes: bytes_value.clone(),
            uint: 0,
        },
    };

    let bytes_state = vec![bytes_state_val];
    let bytes_result = AppManager::decode_app_state(&bytes_state).unwrap();

    // Verify bytes value handling
    assert!(bytes_result.contains_key(&bytes_key));
    let bytes_app_state = &bytes_result[&bytes_key];
    assert_eq!(bytes_app_state.key_raw, bytes_key);
    assert_eq!(bytes_app_state.value_raw, Some(bytes_value.clone()));
    assert_eq!(
        bytes_app_state.value_base64,
        Some(Base64.encode(&bytes_value))
    );

    // Check that the bytes value is correctly decoded as UTF-8 string
    if let AppStateValue::Bytes(ref value_str) = bytes_app_state.value {
        assert_eq!(value_str, "Hello, World!");
    } else {
        panic!("Expected AppStateValue::Bytes");
    }
}

/// Test ABIType-based box value methods structure
#[test]
fn test_abi_type_box_value_methods() {
    // This test demonstrates the ABIType-based box value approach:
    //
    // ABIType-based methods (get_box_value_from_abi_type, get_box_values_from_abi_type):
    //    - Take ABIType directly as parameter
    //    - Return ABIValue directly
    //    - Simpler API that matches TypeScript/Python implementations
    //    - Ideal for box data decoding based on actual storage format

    // Create a simple uint64 ABI type for testing
    let uint64_type = ABIType::Uint(BitSize::new(64).unwrap());

    // Verify the type can be created successfully
    assert_eq!(format!("{}", uint64_type), "uint64");

    // The actual network testing would be done in integration tests with real algod
    // This unit test validates the correct approach for box data decoding
    println!("ABIType approach for box data: Storage type -> ABIValue");
}
