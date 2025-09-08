use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture};
use algokit_abi::{ABIMethod, ABIType, abi_type::BitSize};
use algokit_transact::{Address, OnApplicationComplete, Transaction};
use algokit_utils::transactions::{
    AppCallMethodCallParams, AppCallParams, AppCreateParams, AppDeleteParams, AppUpdateParams,
    AssetClawbackParams, AssetCreateParams, AssetDestroyParams, AssetFreezeParams,
    AssetOptInParams, AssetOptOutParams, AssetTransferParams,
    NonParticipationKeyRegistrationParams, OfflineKeyRegistrationParams,
    OnlineKeyRegistrationParams, PaymentParams, ResourcePopulation, TransactionComposerConfig,
};
use rstest::*;

const GROUP_ANALYSIS_DISABLED: Option<TransactionComposerConfig> =
    Some(TransactionComposerConfig {
        cover_app_call_inner_transaction_fees: false,
        populate_app_call_resources: ResourcePopulation::Disabled,
    });

#[rstest]
#[case::basic(1_000_000)]
#[case::minimum(1)]
#[case::large(100_000_000)]
#[tokio::test]
async fn payment_transaction(
    #[case] amount: u64,
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();
    let receiver = algorand_fixture.generate_account(None).await?;
    let creator = algorand_fixture.algorand_client.create();

    let receiver_address = receiver.account().address();

    let params = PaymentParams {
        sender: sender_address.clone(),
        receiver: receiver_address.clone(),
        amount,
        ..Default::default()
    };

    let tx = creator.payment(params).await?;

    match &tx {
        Transaction::Payment(payment_fields) => {
            assert_eq!(payment_fields.header.sender, sender_address);
            assert_eq!(payment_fields.receiver, receiver_address);
            assert_eq!(payment_fields.amount, amount);
        }
        _ => return Err("Expected Payment transaction".into()),
    }

    Ok(())
}

#[rstest]
#[case::create(AssetTestCase::Create)]
#[case::transfer(AssetTestCase::Transfer)]
#[case::opt_in(AssetTestCase::OptIn)]
#[case::opt_out(AssetTestCase::OptOut)]
#[case::freeze(AssetTestCase::Freeze)]
#[case::destroy(AssetTestCase::Destroy)]
#[case::clawback(AssetTestCase::Clawback)]
#[tokio::test]
async fn asset_operations(
    #[case] test_case: AssetTestCase,
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();

    match test_case {
        AssetTestCase::Create => {
            let params = AssetCreateParams {
                sender: sender_address.clone(),
                total: 1_000_000,
                asset_name: Some("TestAsset".to_string()),
                unit_name: Some("TST".to_string()),
                ..Default::default()
            };
            let tx = algorand_fixture
                .algorand_client
                .create()
                .asset_create(params)
                .await?;
            match &tx {
                Transaction::AssetConfig(asset_fields) => {
                    assert_eq!(asset_fields.header.sender, sender_address);
                    assert_eq!(asset_fields.total, Some(1_000_000));
                }
                _ => return Err("Expected AssetConfig transaction".into()),
            }
        }
        AssetTestCase::Transfer => {
            let receiver = algorand_fixture.generate_account(None).await?;
            let receiver_address = receiver.account().address();
            let params = AssetTransferParams {
                sender: sender_address.clone(),
                receiver: receiver_address.clone(),
                asset_id: 1,
                amount: 100,
                ..Default::default()
            };
            let tx = algorand_fixture
                .algorand_client
                .create()
                .asset_transfer(params)
                .await?;
            match &tx {
                Transaction::AssetTransfer(transfer_fields) => {
                    assert_eq!(transfer_fields.header.sender, sender_address);
                    assert_eq!(transfer_fields.receiver, receiver_address);
                    assert_eq!(transfer_fields.amount, 100);
                    assert_eq!(transfer_fields.asset_id, 1);
                }
                _ => return Err("Expected AssetTransfer transaction".into()),
            }
        }
        AssetTestCase::OptIn => {
            let params = AssetOptInParams {
                sender: sender_address.clone(),
                asset_id: 1,
                ..Default::default()
            };
            let tx = algorand_fixture
                .algorand_client
                .create()
                .asset_opt_in(params)
                .await?;
            match &tx {
                Transaction::AssetTransfer(transfer_fields) => {
                    assert_eq!(transfer_fields.header.sender, sender_address);
                    assert_eq!(transfer_fields.receiver, sender_address);
                    assert_eq!(transfer_fields.amount, 0);
                    assert_eq!(transfer_fields.asset_id, 1);
                }
                _ => return Err("Expected AssetTransfer transaction".into()),
            }
        }
        AssetTestCase::OptOut => {
            let params = AssetOptOutParams {
                sender: sender_address.clone(),
                asset_id: 1,
                close_remainder_to: Some(sender_address.clone()),
                ..Default::default()
            };
            let tx = algorand_fixture
                .algorand_client
                .create()
                .asset_opt_out(params)
                .await?;
            match &tx {
                Transaction::AssetTransfer(transfer_fields) => {
                    assert_eq!(transfer_fields.header.sender, sender_address);
                    assert_eq!(transfer_fields.receiver, sender_address); // Opts out by sending to self
                    assert_eq!(
                        transfer_fields.close_remainder_to,
                        Some(sender_address.clone())
                    );
                    assert_eq!(transfer_fields.asset_id, 1);
                    assert_eq!(transfer_fields.amount, 0); // Opt-out sends 0 amount
                }
                _ => return Err("Expected AssetTransfer transaction".into()),
            }
        }
        AssetTestCase::Freeze => {
            let target = algorand_fixture.generate_account(None).await?;
            let target_address = target.account().address();
            let params = AssetFreezeParams {
                sender: sender_address.clone(),
                asset_id: 1,
                target_address: target_address.clone(),
                ..Default::default()
            };
            let tx = algorand_fixture
                .algorand_client
                .create()
                .asset_freeze(params)
                .await?;
            match &tx {
                Transaction::AssetFreeze(freeze_fields) => {
                    assert_eq!(freeze_fields.header.sender, sender_address);
                    assert_eq!(freeze_fields.freeze_target, target_address);
                    assert_eq!(freeze_fields.asset_id, 1);
                    assert!(freeze_fields.frozen);
                }
                _ => return Err("Expected AssetFreeze transaction".into()),
            }
        }
        AssetTestCase::Destroy => {
            let params = AssetDestroyParams {
                sender: sender_address.clone(),
                asset_id: 1,
                ..Default::default()
            };
            let tx = algorand_fixture
                .algorand_client
                .create()
                .asset_destroy(params)
                .await?;
            match &tx {
                Transaction::AssetConfig(config_fields) => {
                    assert_eq!(config_fields.header.sender, sender_address);
                    assert_eq!(config_fields.asset_id, 1);
                }
                _ => return Err("Expected AssetConfig transaction".into()),
            }
        }
        AssetTestCase::Clawback => {
            let target = algorand_fixture.generate_account(None).await?;
            let receiver = algorand_fixture.generate_account(None).await?;
            let target_address = target.account().address();
            let receiver_address = receiver.account().address();
            let params = AssetClawbackParams {
                sender: sender_address.clone(),
                asset_id: 1,
                clawback_target: target_address.clone(),
                receiver: receiver_address.clone(),
                amount: 50,
                ..Default::default()
            };
            let tx = algorand_fixture
                .algorand_client
                .create()
                .asset_clawback(params)
                .await?;
            match &tx {
                Transaction::AssetTransfer(transfer_fields) => {
                    assert_eq!(transfer_fields.header.sender, sender_address);
                    assert_eq!(transfer_fields.asset_sender, Some(target_address));
                    assert_eq!(transfer_fields.receiver, receiver_address);
                    assert_eq!(transfer_fields.amount, 50);
                    assert_eq!(transfer_fields.asset_id, 1);
                }
                _ => return Err("Expected AssetTransfer transaction".into()),
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
enum AssetTestCase {
    Create,
    Transfer,
    OptIn,
    OptOut,
    Freeze,
    Destroy,
    Clawback,
}

#[rstest]
#[case::call(OnApplicationComplete::NoOp, 1)]
#[case::create(OnApplicationComplete::NoOp, 0)]
#[case::update(OnApplicationComplete::UpdateApplication, 1)]
#[case::delete(OnApplicationComplete::DeleteApplication, 1)]
#[tokio::test]
async fn application_operations(
    #[case] on_complete: OnApplicationComplete,
    #[case] app_id: u64,
    #[with(GROUP_ANALYSIS_DISABLED)]
    #[future]
    algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();

    let tx = match on_complete {
        OnApplicationComplete::NoOp if app_id == 0 => algorand_fixture
            .algorand_client
            .create()
            .app_create(AppCreateParams {
                sender: sender_address.clone(),
                approval_program: vec![0x06, 0x81, 0x01],
                clear_state_program: vec![0x06, 0x81, 0x01],
                ..Default::default()
            })
            .await
            .unwrap(),
        OnApplicationComplete::NoOp => algorand_fixture
            .algorand_client
            .create()
            .app_call(AppCallParams {
                sender: sender_address.clone(),
                app_id,
                on_complete: OnApplicationComplete::NoOp,
                ..Default::default()
            })
            .await
            .unwrap(),
        OnApplicationComplete::UpdateApplication => algorand_fixture
            .algorand_client
            .create()
            .app_update(AppUpdateParams {
                sender: sender_address.clone(),
                app_id,
                approval_program: vec![0x06, 0x81, 0x01],
                clear_state_program: vec![0x06, 0x81, 0x01],
                ..Default::default()
            })
            .await
            .unwrap(),
        OnApplicationComplete::DeleteApplication => algorand_fixture
            .algorand_client
            .create()
            .app_delete(AppDeleteParams {
                sender: sender_address.clone(),
                app_id,
                ..Default::default()
            })
            .await
            .unwrap(),
        _ => unreachable!(),
    };

    match &tx {
        Transaction::AppCall(app_fields) => {
            assert_eq!(app_fields.header.sender, sender_address);
            assert_eq!(app_fields.on_complete, on_complete);
            if app_id == 0 {
                assert_eq!(app_fields.app_id, 0);
                assert!(app_fields.approval_program.is_some());
                assert!(app_fields.clear_state_program.is_some());
            } else {
                assert_eq!(app_fields.app_id, app_id);
            }
        }
        _ => return Err("Expected AppCall transaction".into()),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn method_call_returns_built_transactions(
    #[with(GROUP_ANALYSIS_DISABLED)]
    #[future]
    algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();

    let creator = algorand_fixture.algorand_client.create();

    let method = ABIMethod::new(
        "simple_call".to_string(),
        vec![],
        Some(ABIType::Uint(BitSize::new(64)?)),
        Some("Simple call method".to_string()),
    );

    let params = AppCallMethodCallParams {
        sender: sender_address.clone(),
        app_id: 1,
        method,
        args: vec![],
        on_complete: OnApplicationComplete::NoOp,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        ..Default::default()
    };

    let result = creator.app_call_method_call(params).await?;

    assert!(!result.transactions.is_empty());
    assert!(!result.signers.is_empty());
    assert_eq!(result.transactions.len(), result.signers.len());

    match &result.transactions[0] {
        Transaction::AppCall(app_fields) => {
            assert_eq!(app_fields.header.sender, sender_address);
            assert_eq!(app_fields.app_id, 1);
        }
        _ => return Err("Expected AppCall transaction".into()),
    }

    Ok(())
}

#[rstest]
#[case::online(KeyRegType::Online)]
#[case::offline(KeyRegType::Offline)]
#[case::nonpart(KeyRegType::NonParticipation)]
#[tokio::test]
async fn key_registration_operations(
    #[case] key_type: KeyRegType,
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();

    let tx = match key_type {
        KeyRegType::Online => {
            algorand_fixture
                .algorand_client
                .create()
                .online_key_registration(OnlineKeyRegistrationParams {
                    sender: sender_address.clone(),
                    vote_key: [1u8; 32],
                    selection_key: [2u8; 32],
                    vote_first: 1000,
                    vote_last: 2000,
                    state_proof_key: Some([3u8; 64]),
                    vote_key_dilution: 10000,
                    ..Default::default()
                })
                .await?
        }
        KeyRegType::Offline => {
            algorand_fixture
                .algorand_client
                .create()
                .offline_key_registration(OfflineKeyRegistrationParams {
                    sender: sender_address.clone(),
                    ..Default::default()
                })
                .await?
        }
        KeyRegType::NonParticipation => {
            algorand_fixture
                .algorand_client
                .create()
                .non_participation_key_registration(NonParticipationKeyRegistrationParams {
                    sender: sender_address.clone(),
                    ..Default::default()
                })
                .await?
        }
    };

    match &tx {
        Transaction::KeyRegistration(key_fields) => {
            assert_eq!(key_fields.header.sender, sender_address);

            match key_type {
                KeyRegType::Online => {
                    assert_eq!(key_fields.vote_key, Some([1u8; 32]));
                    assert_eq!(key_fields.selection_key, Some([2u8; 32]));
                    assert_eq!(key_fields.vote_first, Some(1000));
                    assert_eq!(key_fields.vote_last, Some(2000));
                    assert_eq!(key_fields.vote_key_dilution, Some(10000));
                }
                KeyRegType::Offline => {
                    assert!(key_fields.vote_key.is_none());
                    assert!(key_fields.selection_key.is_none());
                }
                KeyRegType::NonParticipation => {
                    assert_eq!(key_fields.non_participation, Some(true));
                }
            }
        }
        _ => return Err("Expected KeyRegistration transaction".into()),
    }

    Ok(())
}

#[derive(Debug, Clone)]
enum KeyRegType {
    Online,
    Offline,
    NonParticipation,
}

#[rstest]
#[tokio::test]
async fn transaction_creator_accepts_all_parameters(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();

    // Test that TransactionCreator accepts parameters and creates transactions
    // Validation happens at the sending level, not creation level
    let params = AssetTransferParams {
        sender: sender_address,
        asset_id: u64::MAX,           // This is valid for transaction creation
        receiver: Address::default(), // Zero address is also valid for creation
        amount: 1,
        ..Default::default()
    };
    let result = algorand_fixture
        .algorand_client
        .create()
        .asset_transfer(params)
        .await?;

    // Verify it created the expected transaction structure
    if let Transaction::AssetTransfer(transfer_fields) = result {
        assert_eq!(transfer_fields.asset_id, u64::MAX);
        assert_eq!(transfer_fields.amount, 1);
    } else {
        return Err("Expected AssetTransfer transaction".into());
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn transaction_has_valid_defaults(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;
    let sender_address = algorand_fixture.test_account.account().address();
    let receiver = algorand_fixture.generate_account(None).await?;

    let creator = algorand_fixture.algorand_client.create();

    let receiver_address = receiver.account().address();

    let params = PaymentParams {
        sender: sender_address,
        receiver: receiver_address,
        amount: 1_000_000,
        ..Default::default()
    };

    let tx = creator.payment(params).await?;

    let header = tx.header();
    assert!(header.fee.unwrap_or(0) >= 1000);
    assert!(header.first_valid > 0);
    assert!(header.last_valid > header.first_valid);
    assert!(header.genesis_id.is_some());
    assert!(header.genesis_hash.is_some());

    Ok(())
}
