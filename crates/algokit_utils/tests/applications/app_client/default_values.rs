use crate::common::TestResult;
use crate::common::app_fixture::testing_app_fixture;
use algokit_abi::ABIValue;
use algokit_utils::AppMethodCallArg;
use algokit_utils::applications::app_client::AppClientMethodCallParams;
use num_bigint::BigUint;
use rstest::*;

#[rstest]
#[tokio::test]
async fn test_default_value_from_literal(
    #[future] testing_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let f = testing_app_fixture.await?;
    let client = f.client;
    let sender = f.sender_address;

    let defined = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "default_value".to_string(),
                args: vec![AppMethodCallArg::ABIValue(ABIValue::from("defined value"))],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;
    let defined_ret = defined
        .abi_return
        .and_then(|r| r.return_value)
        .expect("Expected ABI return value");
    match defined_ret {
        ABIValue::String(s) => assert_eq!(s, "defined value"),
        _ => return Err("Expected string return".into()),
    }

    let defaulted = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "default_value".to_string(),
                args: vec![AppMethodCallArg::DefaultValue],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;
    let default_ret = defaulted
        .abi_return
        .and_then(|r| r.return_value)
        .expect("Expected ABI return value");
    match default_ret {
        ABIValue::String(s) => assert_eq!(s, "default value"),
        _ => return Err("Expected string return".into()),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_default_value_from_method(
    #[future] testing_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let f = testing_app_fixture.await?;
    let client = f.client;
    let sender = f.sender_address;

    let defined = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "default_value_from_abi".to_string(),
                args: vec![AppMethodCallArg::ABIValue(ABIValue::from("defined value"))],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;
    let defined_ret = defined
        .abi_return
        .and_then(|r| r.return_value)
        .expect("Expected ABI return value");
    match defined_ret {
        ABIValue::String(s) => assert_eq!(s, "ABI, defined value"),
        _ => return Err("Expected string return".into()),
    }

    let defaulted = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "default_value_from_abi".to_string(),
                args: vec![AppMethodCallArg::DefaultValue],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;
    let default_ret = defaulted
        .abi_return
        .and_then(|r| r.return_value)
        .expect("Expected ABI return value");
    match default_ret {
        ABIValue::String(s) => assert_eq!(s, "ABI, default value"),
        _ => return Err("Expected string return".into()),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_default_value_from_global_state(
    #[future] testing_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let f = testing_app_fixture.await?;
    let client = f.client;
    let sender = f.sender_address;

    client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "set_global".to_string(),
                args: vec![
                    AppMethodCallArg::ABIValue(ABIValue::from(456u64)),
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

    let defined = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "default_value_from_global_state".to_string(),
                args: vec![AppMethodCallArg::ABIValue(ABIValue::from(123u64))],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;
    let defined_ret = defined
        .abi_return
        .and_then(|r| r.return_value)
        .expect("Expected ABI return value");
    match defined_ret {
        ABIValue::Uint(v) => assert_eq!(v, BigUint::from(123u64)),
        _ => return Err("Expected uint return".into()),
    }

    let defaulted = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "default_value_from_global_state".to_string(),
                args: vec![AppMethodCallArg::DefaultValue],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;
    let default_ret = defaulted
        .abi_return
        .and_then(|r| r.return_value)
        .expect("Expected ABI return value");
    match default_ret {
        ABIValue::Uint(v) => assert_eq!(v, BigUint::from(456u64)),
        _ => return Err("Expected uint return".into()),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_default_value_from_local_state(
    #[future] testing_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let f = testing_app_fixture.await?;
    let client = f.client;
    let sender = f.sender_address;

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
                    AppMethodCallArg::ABIValue(ABIValue::from("banana")),
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

    let defined = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "default_value_from_local_state".to_string(),
                args: vec![AppMethodCallArg::ABIValue(ABIValue::from("defined value"))],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;
    let defined_ret = defined
        .abi_return
        .and_then(|r| r.return_value)
        .expect("Expected ABI return value");
    match defined_ret {
        ABIValue::String(s) => assert_eq!(s, "Local state, defined value"),
        _ => return Err("Expected string return".into()),
    }

    let defaulted = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "default_value_from_local_state".to_string(),
                args: vec![AppMethodCallArg::DefaultValue],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;
    let default_ret = defaulted
        .abi_return
        .and_then(|r| r.return_value)
        .expect("Expected ABI return value");
    match default_ret {
        ABIValue::String(s) => assert_eq!(s, "Local state, banana"),
        _ => return Err("Expected string return".into()),
    }

    Ok(())
}
