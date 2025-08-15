use algokit_abi::{ABIMethod, ABIType, abi_type::BitSize};
use algokit_transact::{Address, OnApplicationComplete, Transaction};
use algokit_utils::{
    AlgorandClient,
    testing::algorand_fixture,
    transactions::{
        AppCallMethodCallParams, AppCallParams, AppCreateParams, AppDeleteParams, AppUpdateParams,
        AssetClawbackParams, AssetCreateParams, AssetDestroyParams, AssetFreezeParams,
        AssetOptInParams, AssetOptOutParams, AssetTransferParams, CommonParams,
        NonParticipationKeyRegistrationParams, OfflineKeyRegistrationParams,
        OnlineKeyRegistrationParams, PaymentParams,
    },
};
use rstest::*;

#[rstest]
#[case::basic(1_000_000)]
#[case::minimum(1)]
#[case::large(100_000_000)]
#[tokio::test]
async fn payment_transaction(#[case] amount: u64) {
    let mut fixture = algorand_fixture().await.unwrap();
    fixture.new_scope().await.unwrap();
    let sender = fixture.context().unwrap().test_account.clone();
    let receiver = fixture.generate_account(None).await.unwrap();

    let client = AlgorandClient::from_environment();
    let creator = client.create();

    let sender_address = sender.account().unwrap().address();
    let receiver_address = receiver.account().unwrap().address();

    let params = PaymentParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        receiver: receiver_address.clone(),
        amount,
    };

    let tx = creator.payment(params).await.unwrap();

    match &tx {
        Transaction::Payment(payment_fields) => {
            assert_eq!(payment_fields.header.sender, sender_address);
            assert_eq!(payment_fields.receiver, receiver_address);
            assert_eq!(payment_fields.amount, amount);
        }
        _ => panic!("Expected Payment transaction"),
    }
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
async fn asset_operations(#[case] test_case: AssetTestCase) {
    let mut fixture = algorand_fixture().await.unwrap();
    fixture.new_scope().await.unwrap();
    let sender = fixture.context().unwrap().test_account.clone();

    let client = AlgorandClient::from_environment();
    let creator = client.create();

    let sender_address = sender.account().unwrap().address();

    match test_case {
        AssetTestCase::Create => {
            let params = AssetCreateParams {
                common_params: CommonParams {
                    sender: sender_address.clone(),
                    ..Default::default()
                },
                total: 1_000_000,
                asset_name: Some("TestAsset".to_string()),
                unit_name: Some("TST".to_string()),
                ..Default::default()
            };
            let tx = creator.asset_create(params).await.unwrap();
            match &tx {
                Transaction::AssetConfig(asset_fields) => {
                    assert_eq!(asset_fields.header.sender, sender_address);
                    assert_eq!(asset_fields.total, Some(1_000_000));
                }
                _ => panic!("Expected AssetConfig transaction"),
            }
        }
        AssetTestCase::Transfer => {
            let receiver = fixture.generate_account(None).await.unwrap();
            let receiver_address = receiver.account().unwrap().address();
            let params = AssetTransferParams {
                common_params: CommonParams {
                    sender: sender_address.clone(),
                    ..Default::default()
                },
                receiver: receiver_address.clone(),
                asset_id: 1,
                amount: 100,
            };
            let tx = creator.asset_transfer(params).await.unwrap();
            match &tx {
                Transaction::AssetTransfer(transfer_fields) => {
                    assert_eq!(transfer_fields.header.sender, sender_address);
                    assert_eq!(transfer_fields.receiver, receiver_address);
                    assert_eq!(transfer_fields.amount, 100);
                    assert_eq!(transfer_fields.asset_id, 1);
                }
                _ => panic!("Expected AssetTransfer transaction"),
            }
        }
        AssetTestCase::OptIn => {
            let params = AssetOptInParams {
                common_params: CommonParams {
                    sender: sender_address.clone(),
                    ..Default::default()
                },
                asset_id: 1,
            };
            let tx = creator.asset_opt_in(params).await.unwrap();
            match &tx {
                Transaction::AssetTransfer(transfer_fields) => {
                    assert_eq!(transfer_fields.header.sender, sender_address);
                    assert_eq!(transfer_fields.receiver, sender_address);
                    assert_eq!(transfer_fields.amount, 0);
                    assert_eq!(transfer_fields.asset_id, 1);
                }
                _ => panic!("Expected AssetTransfer transaction"),
            }
        }
        AssetTestCase::OptOut => {
            let params = AssetOptOutParams {
                common_params: CommonParams {
                    sender: sender_address.clone(),
                    ..Default::default()
                },
                asset_id: 1,
                close_remainder_to: Some(sender_address.clone()),
            };
            let tx = creator.asset_opt_out(params).await.unwrap();
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
                _ => panic!("Expected AssetTransfer transaction"),
            }
        }
        AssetTestCase::Freeze => {
            let target = fixture.generate_account(None).await.unwrap();
            let target_address = target.account().unwrap().address();
            let params = AssetFreezeParams {
                common_params: CommonParams {
                    sender: sender_address.clone(),
                    ..Default::default()
                },
                asset_id: 1,
                target_address: target_address.clone(),
            };
            let tx = creator.asset_freeze(params).await.unwrap();
            match &tx {
                Transaction::AssetFreeze(freeze_fields) => {
                    assert_eq!(freeze_fields.header.sender, sender_address);
                    assert_eq!(freeze_fields.freeze_target, target_address);
                    assert_eq!(freeze_fields.asset_id, 1);
                    assert_eq!(freeze_fields.frozen, true);
                }
                _ => panic!("Expected AssetFreeze transaction"),
            }
        }
        AssetTestCase::Destroy => {
            let params = AssetDestroyParams {
                common_params: CommonParams {
                    sender: sender_address.clone(),
                    ..Default::default()
                },
                asset_id: 1,
            };
            let tx = creator.asset_destroy(params).await.unwrap();
            match &tx {
                Transaction::AssetConfig(config_fields) => {
                    assert_eq!(config_fields.header.sender, sender_address);
                    assert_eq!(config_fields.asset_id, 1);
                }
                _ => panic!("Expected AssetConfig transaction"),
            }
        }
        AssetTestCase::Clawback => {
            let target = fixture.generate_account(None).await.unwrap();
            let receiver = fixture.generate_account(None).await.unwrap();
            let target_address = target.account().unwrap().address();
            let receiver_address = receiver.account().unwrap().address();
            let params = AssetClawbackParams {
                common_params: CommonParams {
                    sender: sender_address.clone(),
                    ..Default::default()
                },
                asset_id: 1,
                clawback_target: target_address.clone(),
                receiver: receiver_address.clone(),
                amount: 50,
            };
            let tx = creator.asset_clawback(params).await.unwrap();
            match &tx {
                Transaction::AssetTransfer(transfer_fields) => {
                    assert_eq!(transfer_fields.header.sender, sender_address);
                    assert_eq!(transfer_fields.asset_sender, Some(target_address));
                    assert_eq!(transfer_fields.receiver, receiver_address);
                    assert_eq!(transfer_fields.amount, 50);
                    assert_eq!(transfer_fields.asset_id, 1);
                }
                _ => panic!("Expected AssetTransfer transaction"),
            }
        }
    }
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
async fn application_operations(#[case] on_complete: OnApplicationComplete, #[case] app_id: u64) {
    let mut fixture = algorand_fixture().await.unwrap();
    fixture.new_scope().await.unwrap();
    let sender = fixture.context().unwrap().test_account.clone();

    let client = AlgorandClient::from_environment();
    let creator = client.create();

    let sender_address = sender.account().unwrap().address();

    let tx = match on_complete {
        OnApplicationComplete::NoOp if app_id == 0 => creator
            .application_create(AppCreateParams {
                common_params: CommonParams {
                    sender: sender_address.clone(),
                    ..Default::default()
                },
                approval_program: vec![0x06, 0x81, 0x01],
                clear_state_program: vec![0x06, 0x81, 0x01],
                ..Default::default()
            })
            .await
            .unwrap(),
        OnApplicationComplete::NoOp => creator
            .application_call(AppCallParams {
                common_params: CommonParams {
                    sender: sender_address.clone(),
                    ..Default::default()
                },
                app_id,
                on_complete: OnApplicationComplete::NoOp,
                ..Default::default()
            })
            .await
            .unwrap(),
        OnApplicationComplete::UpdateApplication => creator
            .application_update(AppUpdateParams {
                common_params: CommonParams {
                    sender: sender_address.clone(),
                    ..Default::default()
                },
                app_id,
                approval_program: vec![0x06, 0x81, 0x01],
                clear_state_program: vec![0x06, 0x81, 0x01],
                ..Default::default()
            })
            .await
            .unwrap(),
        OnApplicationComplete::DeleteApplication => creator
            .application_delete(AppDeleteParams {
                common_params: CommonParams {
                    sender: sender_address.clone(),
                    ..Default::default()
                },
                app_id,
                ..Default::default()
            })
            .await
            .unwrap(),
        _ => unreachable!(),
    };

    match &tx {
        Transaction::ApplicationCall(app_fields) => {
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
        _ => panic!("Expected ApplicationCall transaction"),
    }
}

#[rstest]
#[tokio::test]
async fn method_call_returns_built_transactions() {
    let mut fixture = algorand_fixture().await.unwrap();
    fixture.new_scope().await.unwrap();
    let sender = fixture.context().unwrap().test_account.clone();

    let client = AlgorandClient::from_environment();
    let creator = client.create();

    let sender_address = sender.account().unwrap().address();

    let method = ABIMethod::new(
        "simple_call".to_string(),
        vec![],
        Some(ABIType::Uint(BitSize::new(64).unwrap())),
        Some("Simple call method".to_string()),
    );

    let params = AppCallMethodCallParams {
        common_params: CommonParams {
            sender: sender_address.clone(),
            ..Default::default()
        },
        app_id: 1,
        method,
        args: vec![],
        on_complete: OnApplicationComplete::NoOp,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let result = creator.application_method_call(params).await.unwrap();

    assert!(!result.transactions.is_empty());
    assert!(!result.signers.is_empty());
    assert_eq!(result.transactions.len(), result.signers.len());

    match &result.transactions[0] {
        Transaction::ApplicationCall(app_fields) => {
            assert_eq!(app_fields.header.sender, sender_address);
            assert_eq!(app_fields.app_id, 1);
        }
        _ => panic!("Expected ApplicationCall transaction"),
    }
}

#[rstest]
#[case::online(KeyRegType::Online)]
#[case::offline(KeyRegType::Offline)]
#[case::nonpart(KeyRegType::NonParticipation)]
#[tokio::test]
async fn key_registration_operations(#[case] key_type: KeyRegType) {
    let mut fixture = algorand_fixture().await.unwrap();
    fixture.new_scope().await.unwrap();
    let sender = fixture.context().unwrap().test_account.clone();

    let client = AlgorandClient::from_environment();
    let creator = client.create();

    let sender_address = sender.account().unwrap().address();

    let tx = match key_type {
        KeyRegType::Online => creator
            .online_key_registration(OnlineKeyRegistrationParams {
                common_params: CommonParams {
                    sender: sender_address.clone(),
                    ..Default::default()
                },
                vote_key: [1u8; 32],
                selection_key: [2u8; 32],
                vote_first: 1000,
                vote_last: 2000,
                state_proof_key: Some([3u8; 64]),
                vote_key_dilution: 10000,
            })
            .await
            .unwrap(),
        KeyRegType::Offline => creator
            .offline_key_registration(OfflineKeyRegistrationParams {
                common_params: CommonParams {
                    sender: sender_address.clone(),
                    ..Default::default()
                },
                non_participation: None,
            })
            .await
            .unwrap(),
        KeyRegType::NonParticipation => creator
            .non_participation_key_registration(NonParticipationKeyRegistrationParams {
                common_params: CommonParams {
                    sender: sender_address.clone(),
                    ..Default::default()
                },
            })
            .await
            .unwrap(),
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
        _ => panic!("Expected KeyRegistration transaction"),
    }
}

#[derive(Debug, Clone)]
enum KeyRegType {
    Online,
    Offline,
    NonParticipation,
}

#[rstest]
#[tokio::test]
async fn transaction_creator_accepts_all_parameters() {
    let mut fixture = algorand_fixture().await.unwrap();
    fixture.new_scope().await.unwrap();

    let client = AlgorandClient::from_environment();
    let creator = client.create();

    // Test that TransactionCreator accepts parameters and creates transactions
    // Validation happens at the sending level, not creation level
    let params = AssetTransferParams {
        common_params: CommonParams::default(),
        asset_id: u64::MAX,           // This is valid for transaction creation
        receiver: Address::default(), // Zero address is also valid for creation
        amount: 1,
    };
    let result = creator.asset_transfer(params).await;

    // TransactionCreator doesn't validate much - it just creates transactions
    // So this should succeed (the validation happens when sending)
    assert!(
        result.is_ok(),
        "TransactionCreator should create transaction successfully"
    );

    // Verify it created the expected transaction structure
    if let Ok(Transaction::AssetTransfer(transfer_fields)) = result {
        assert_eq!(transfer_fields.asset_id, u64::MAX);
        assert_eq!(transfer_fields.amount, 1);
    } else {
        panic!("Expected AssetTransfer transaction");
    }
}

#[rstest]
#[tokio::test]
async fn transaction_has_valid_defaults() {
    let mut fixture = algorand_fixture().await.unwrap();
    fixture.new_scope().await.unwrap();
    let sender = fixture.context().unwrap().test_account.clone();
    let receiver = fixture.generate_account(None).await.unwrap();

    let client = AlgorandClient::from_environment();
    let creator = client.create();

    let sender_address = sender.account().unwrap().address();
    let receiver_address = receiver.account().unwrap().address();

    let params = PaymentParams {
        common_params: CommonParams {
            sender: sender_address,
            ..Default::default()
        },
        receiver: receiver_address,
        amount: 1_000_000,
    };

    let tx = creator.payment(params).await.unwrap();

    let header = tx.header();
    assert!(header.fee.unwrap_or(0) >= 1000);
    assert!(header.first_valid > 0);
    assert!(header.last_valid > header.first_valid);
    assert!(header.genesis_id.is_some());
    assert!(header.genesis_hash.is_some());
}
