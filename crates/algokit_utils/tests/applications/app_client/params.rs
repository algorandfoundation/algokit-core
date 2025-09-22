use crate::common::TestResult;
use crate::common::app_fixture::{sandbox_app_fixture, testing_app_fixture};
use algokit_abi::ABIValue;
use algokit_transact::BoxReference;
use algokit_utils::applications::app_client::AppClientBareCallParams;
use algokit_utils::applications::app_client::{AppClientMethodCallParams, FundAppAccountParams};
use algokit_utils::{AppMethodCallArg, PaymentParams};
use rstest::*;

#[rstest]
#[tokio::test]
async fn params_build_method_call_and_defaults(
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
                    AppMethodCallArg::ABIValue(ABIValue::from(999u64)),
                    AppMethodCallArg::ABIValue(ABIValue::from(2u64)),
                    AppMethodCallArg::ABIValue(ABIValue::from("seed")),
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
                    AppMethodCallArg::ABIValue(ABIValue::from("bananas")),
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

    let built = client
        .params()
        .call(
            AppClientMethodCallParams {
                method: "default_value_from_local_state".to_string(),
                args: vec![AppMethodCallArg::DefaultValue],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
        )
        .await?;

    assert_eq!(built.method.name, "default_value_from_local_state");
    assert_eq!(built.args.len(), 1);
    match &built.args[0] {
        AppMethodCallArg::ABIValue(ABIValue::String(s)) => assert_eq!(s, "bananas"),
        _ => return Err("expected string arg resolved from local state".into()),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn params_build_includes_foreign_references_from_args(
    #[future] testing_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let mut f = testing_app_fixture.await?;
    let sender = f.sender_address.clone();
    let client = &f.client;
    let extra = f.algorand_fixture.generate_account(None).await?;
    let extra_addr = extra.account().address().to_string();

    let built = client
        .params()
        .call(
            AppClientMethodCallParams {
                method: "call_abi_foreign_refs".to_string(),
                args: vec![],
                sender: Some(sender.to_string()),
                account_references: Some(vec![extra_addr.clone()]),
                app_references: Some(vec![345]),
                asset_references: Some(vec![567]),
                ..Default::default()
            },
            None,
        )
        .await?;

    assert!(!built.account_references.as_ref().unwrap().is_empty());
    assert!(built.app_references.as_ref().unwrap().contains(&345));
    assert!(built.asset_references.as_ref().unwrap().contains(&567));

    Ok(())
}

#[rstest]
#[tokio::test]
async fn params_build_bare_and_fund_payment(
    #[future] sandbox_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let f = sandbox_app_fixture.await?;
    let sender = f.sender_address;
    let client = f.client;

    let bare = client.params().bare().call(
        AppClientBareCallParams {
            args: None,
            sender: Some(sender.to_string()),
            box_references: Some(vec![BoxReference {
                app_id: 0,
                name: b"1".to_vec(),
            }]),
            ..Default::default()
        },
        None,
    )?;
    assert_eq!(bare.box_references.as_ref().unwrap()[0].name, b"1".to_vec());

    let pay: PaymentParams = client.params().fund_app_account(&FundAppAccountParams {
        amount: 200_000,
        sender: Some(sender.to_string()),
        ..Default::default()
    })?;
    assert_eq!(pay.amount, 200_000);
    assert_eq!(pay.receiver, client.app_address());

    Ok(())
}

#[rstest]
#[tokio::test]
async fn params_construct_txn_with_abi_tx_arg_and_return(
    #[future] sandbox_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let f = sandbox_app_fixture.await?;
    let sender = f.sender_address;
    let client = f.client;

    let payment = PaymentParams {
        sender: sender.clone(),
        signer: None,
        rekey_to: None,
        note: None,
        lease: None,
        static_fee: None,
        extra_fee: None,
        max_fee: None,
        validity_window: None,
        first_valid_round: None,
        last_valid_round: None,
        receiver: sender.clone(),
        amount: 123,
    };

    let result = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "get_pay_txn_amount".to_string(),
                args: vec![AppMethodCallArg::Payment(payment)],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    assert_eq!(result.common_params.transactions.len(), 2);
    let abi_ret = result.abi_return.as_ref().expect("abi return expected");
    match &abi_ret.return_value {
        Some(ABIValue::Uint(u)) => assert_eq!(*u, num_bigint::BigUint::from(123u32)),
        _ => return Err("expected uint return".into()),
    }
    Ok(())
}
