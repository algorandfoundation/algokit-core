use algokit_abi::ABIMethod;
use algokit_test_artifacts::{abi_create_and_delete, testing_app};
use algokit_transact::{Address, OnApplicationComplete};
use algokit_utils::applications::{
    AppDeployMetadata, AppDeployParams, AppDeployResult, AppDeployer, AppProgram, CreateParams,
    DeleteParams, DeployAppCreateMethodCallParams, DeployAppCreateParams,
    DeployAppDeleteMethodCallParams, DeployAppDeleteParams, DeployAppUpdateParams, OnSchemaBreak,
    OnUpdate, UpdateParams,
};
use algokit_utils::clients::app_manager::{AppManager, DeploymentMetadata, TealTemplateValue};
use algokit_utils::{AppCreateParams, AppMethodCallArg, PaymentParams, TransactionSender};
use algokit_utils::{AssetManager, SendParams};
use base64::{Engine, prelude::BASE64_STANDARD};
use rstest::*;
use serde_json;
use std::collections::HashMap;
use std::str::FromStr;

use crate::common::{AlgorandFixture, AlgorandFixtureResult, TestResult, algorand_fixture};

#[fixture]
async fn fixture(#[future] algorand_fixture: AlgorandFixtureResult) -> FixtureResult {
    let algorand_fixture = algorand_fixture.await?;
    let test_account = algorand_fixture.test_account.account().address();
    let algod_client = algorand_fixture.algod.clone();
    let indexer_client = algorand_fixture.indexer.clone();

    let composer = algorand_fixture.algorand_client.new_group(None);
    let asset_manager = AssetManager::new(algod_client.clone(), {
        let new_composer = composer.clone();
        move |_params| new_composer.clone()
    });
    let app_manager = AppManager::new(algod_client.clone());

    let transaction_sender = TransactionSender::new(
        {
            let new_composer = composer.clone();
            move |_params| new_composer.clone()
        },
        asset_manager,
    );

    let app_deployer = AppDeployer::new(
        app_manager.clone(),
        transaction_sender.clone(),
        Some(indexer_client.clone()),
    );

    Ok(Fixture {
        test_account,
        algorand_fixture,
        app_manager,
        transaction_sender,
        app_deployer,
    })
}

#[rstest]
#[tokio::test]
async fn test_created_app_is_retrieved_by_name_with_deployment_metadata(
    #[future] fixture: FixtureResult,
) -> TestResult {
    let Fixture {
        test_account,
        app_manager,
        transaction_sender,
        algorand_fixture,
        mut app_deployer,
        ..
    } = fixture.await?;

    let creation_metadata = get_metadata(AppDeployMetadataParams {
        name: Some(String::from("MY_APP")),
        version: Some(String::from("1.0")),
        updatable: Some(true),
        ..Default::default()
    });

    let create_params =
        get_testing_app_create_params(&app_manager, &test_account, &creation_metadata).await?;

    let result = transaction_sender.app_create(create_params, None).await?;

    algorand_fixture
        .wait_for_indexer_transaction(&result.transaction_id)
        .await?;

    let apps = app_deployer
        .get_creator_apps_by_name(&test_account, None)
        .await?;

    assert_eq!(apps.creator, test_account);
    assert_eq!(apps.apps.len(), 1);
    assert!(apps.apps.contains_key("MY_APP"));
    let app = &apps.apps["MY_APP"];
    assert_eq!(app.app_id, result.app_id);
    assert_eq!(app.app_address, result.app_address);
    assert_eq!(
        app.created_round,
        result.confirmation.confirmed_round.unwrap()
    );
    assert_eq!(app.created_metadata, creation_metadata);
    assert_eq!(app.updated_round, app.created_round);
    assert_eq!(app.name, creation_metadata.name);
    assert_eq!(app.version, creation_metadata.version);
    assert_eq!(app.updatable, creation_metadata.updatable);
    assert_eq!(app.deletable, creation_metadata.deletable);
    assert!(!app.deleted);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_latest_created_app_is_retrieved(#[future] fixture: FixtureResult) -> TestResult {
    let Fixture {
        test_account,
        app_manager,
        transaction_sender,
        algorand_fixture,
        mut app_deployer,
        ..
    } = fixture.await?;

    let creation_metadata = get_metadata(AppDeployMetadataParams {
        name: Some(String::from("MY_APP")),
        version: Some(String::from("1.0")),
        updatable: Some(true),
        deletable: Some(false),
    });

    let mut create_params_1 =
        get_testing_app_create_params(&app_manager, &test_account, &creation_metadata).await?;
    create_params_1.lease = Some([1u8; 32]);
    transaction_sender.app_create(create_params_1, None).await?;

    let mut create_params_2 =
        get_testing_app_create_params(&app_manager, &test_account, &creation_metadata).await?;
    create_params_2.lease = Some([2u8; 32]);
    transaction_sender.app_create(create_params_2, None).await?;

    let mut create_params_3 =
        get_testing_app_create_params(&app_manager, &test_account, &creation_metadata).await?;
    create_params_3.lease = Some([3u8; 32]);
    let result_3 = transaction_sender.app_create(create_params_3, None).await?;

    algorand_fixture
        .wait_for_indexer_transaction(&result_3.transaction_id)
        .await?;

    let apps = app_deployer
        .get_creator_apps_by_name(&test_account, None)
        .await?;

    assert_eq!(apps.apps["MY_APP"].app_id, result_3.app_id);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_created_updated_and_deleted_apps_are_retrieved_by_name_with_deployment_metadata(
    #[future] fixture: FixtureResult,
) -> TestResult {
    let Fixture {
        test_account,
        app_manager,
        transaction_sender,
        algorand_fixture,
        mut app_deployer,
        ..
    } = fixture.await?;

    let creation_metadata = get_metadata(AppDeployMetadataParams {
        name: Some(String::from("MY_APP")),
        version: Some(String::from("1.0")),
        updatable: Some(true),
        deletable: Some(true),
    });
    let create_params_1 =
        get_testing_app_create_params(&app_manager, &test_account, &creation_metadata).await?;
    let result_1 = transaction_sender.app_create(create_params_1, None).await?;

    let creation_metadata_2 = AppDeployMetadata {
        name: String::from("APP_2"),
        ..creation_metadata.clone()
    };
    let create_params_2 =
        get_testing_app_create_params(&app_manager, &test_account, &creation_metadata_2).await?;
    let result_2 = transaction_sender.app_create(create_params_2, None).await?;

    let creation_metadata_3 = AppDeployMetadata {
        name: String::from("APP_3"),
        ..creation_metadata.clone()
    };
    let create_params_3 =
        get_testing_app_create_params(&app_manager, &test_account, &creation_metadata_3).await?;
    let result_3 = transaction_sender.app_create(create_params_3, None).await?;

    // Update app 1
    let update_metadata = AppDeployMetadata {
        version: String::from("2.0"),
        ..creation_metadata.clone()
    };
    let update_create_params =
        get_testing_app_create_params(&app_manager, &test_account, &update_metadata).await?;
    let update_params = algokit_utils::AppUpdateParams {
        sender: update_create_params.sender.clone(),
        signer: update_create_params.signer.clone(),
        rekey_to: update_create_params.rekey_to.clone(),
        note: update_create_params.note.clone(),
        lease: update_create_params.lease,
        static_fee: update_create_params.static_fee,
        extra_fee: update_create_params.extra_fee,
        max_fee: update_create_params.max_fee,
        validity_window: update_create_params.validity_window,
        first_valid_round: update_create_params.first_valid_round,
        last_valid_round: update_create_params.last_valid_round,
        app_id: result_1.app_id,
        approval_program: update_create_params.approval_program,
        clear_state_program: update_create_params.clear_state_program,
        args: update_create_params.args,
        account_references: update_create_params.account_references,
        app_references: update_create_params.app_references,
        asset_references: update_create_params.asset_references,
        box_references: update_create_params.box_references,
    };
    let update_result = transaction_sender.app_update(update_params, None).await?;

    // Delete app 3
    let delete_params = algokit_utils::AppDeleteParams {
        sender: test_account.clone(),
        app_id: result_3.app_id,
        args: None,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        ..Default::default()
    };
    let delete_result = transaction_sender.app_delete(delete_params, None).await?;

    algorand_fixture
        .wait_for_indexer_transaction(&delete_result.transaction_id)
        .await?;

    let apps = app_deployer
        .get_creator_apps_by_name(&test_account, None)
        .await?;

    assert_eq!(apps.creator, test_account);
    let mut app_names: Vec<String> = apps.apps.keys().cloned().collect();
    app_names.sort();
    let mut expected_names = vec![
        "MY_APP".to_string(),
        "APP_2".to_string(),
        "APP_3".to_string(),
    ];
    expected_names.sort();
    assert_eq!(app_names, expected_names);

    // Check app 1 was updated
    let app_1_data = &apps.apps["MY_APP"];
    assert_eq!(app_1_data.app_id, result_1.app_id);
    assert_eq!(app_1_data.app_address, result_1.app_address);
    assert_eq!(
        app_1_data.created_round,
        result_1.confirmation.confirmed_round.unwrap()
    );
    assert_eq!(app_1_data.created_metadata, creation_metadata);
    assert_ne!(app_1_data.created_round, app_1_data.updated_round);
    assert_eq!(
        app_1_data.updated_round,
        update_result.confirmation.confirmed_round.unwrap()
    );
    assert_eq!(app_1_data.name, update_metadata.name);
    assert_eq!(app_1_data.updatable, update_metadata.updatable);
    assert_eq!(app_1_data.deletable, update_metadata.deletable);
    assert_eq!(app_1_data.version, update_metadata.version);
    assert!(!app_1_data.deleted);

    // Check app 2 is unchanged
    let app2_data = &apps.apps["APP_2"];
    assert_eq!(app2_data.app_id, result_2.app_id);
    assert_eq!(app2_data.created_round, app2_data.updated_round);
    assert!(!app2_data.deleted);

    // Check app 3 is deleted
    let app3_data = &apps.apps["APP_3"];
    assert_eq!(app3_data.app_id, result_3.app_id);
    assert!(app3_data.deleted);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_deploy_new_app(#[future] fixture: FixtureResult) -> TestResult {
    let Fixture {
        test_account,
        mut app_deployer,
        ..
    } = fixture.await?;

    let metadata = get_metadata(AppDeployMetadataParams {
        ..Default::default()
    });
    let deployment =
        get_testing_app_deploy_params(&test_account, &metadata, None, None, None, None).await?;

    let result = app_deployer.deploy(deployment).await?;
    let (app, create_result) = match &result {
        AppDeployResult::Create { app, result } => (app, result),
        _ => return Err("Expected Create result".into()),
    };

    assert_eq!(app.app_id, create_result.confirmations[0].app_id.unwrap());
    assert_eq!(app.app_address, Address::from_app_id(&app.app_id));
    assert_eq!(app.created_metadata, metadata);
    assert_eq!(
        app.created_round,
        create_result.confirmations[0].confirmed_round.unwrap()
    );
    assert_eq!(app.updated_round, app.created_round);
    assert_eq!(app.name, metadata.name);
    assert_eq!(app.version, metadata.version);
    assert_eq!(app.updatable, metadata.updatable);
    assert_eq!(app.deletable, metadata.deletable);
    assert!(!app.deleted);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_fail_to_deploy_immutable_app_without_tmpl_updatable(
    #[future] fixture: FixtureResult,
) -> TestResult {
    let Fixture {
        test_account,
        mut app_deployer,
        ..
    } = fixture.await?;

    let metadata = get_metadata(AppDeployMetadataParams {
        updatable: Some(true),
        ..Default::default()
    });
    let mut deployment =
        get_testing_app_deploy_params(&test_account, &metadata, None, None, None, None).await?;

    // Remove TMPL_UPDATABLE from the approval program to simulate the validation failure
    if let CreateParams::AppCreateCall(ref mut create_params) = deployment.create_params {
        if let AppProgram::Teal(ref mut approval_program) = create_params.approval_program {
            *approval_program = approval_program.replace("TMPL_UPDATABLE", "0");
        }
    }

    let result = app_deployer.deploy(deployment).await;

    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    assert!(error_message.contains(
        "Deploy-time updatability control requested, but TMPL_UPDATABLE not present in TEAL code"
    ));

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_fail_to_deploy_permanent_app_without_tmpl_deletable(
    #[future] fixture: FixtureResult,
) -> TestResult {
    let Fixture {
        test_account,
        mut app_deployer,
        ..
    } = fixture.await?;

    let metadata = get_metadata(AppDeployMetadataParams {
        deletable: Some(true),
        ..Default::default()
    });
    let mut deployment =
        get_testing_app_deploy_params(&test_account, &metadata, None, None, None, None).await?;

    // Remove TMPL_DELETABLE from the approval program to simulate the validation failure
    if let CreateParams::AppCreateCall(ref mut create_params) = deployment.create_params {
        if let AppProgram::Teal(ref mut approval_program) = create_params.approval_program {
            *approval_program = approval_program.replace("TMPL_DELETABLE", "0");
        }
    }

    let result = app_deployer.deploy(deployment).await;

    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    assert!(error_message.contains(
        "Deploy-time deletability control requested, but TMPL_DELETABLE not present in TEAL code"
    ));

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_deploy_update_to_updatable_app(#[future] fixture: FixtureResult) -> TestResult {
    let Fixture {
        test_account,
        mut app_deployer,
        algorand_fixture,
        ..
    } = fixture.await?;

    // Deploy initial app with updatable=true
    let metadata = get_metadata(AppDeployMetadataParams {
        updatable: Some(true),
        ..Default::default()
    });
    let deployment_1 =
        get_testing_app_deploy_params(&test_account, &metadata, None, None, None, None).await?;
    let result_1 = app_deployer.deploy(deployment_1).await?;
    let (app_1, app_1_id, tx_id) = match &result_1 {
        AppDeployResult::Create { app, result } => {
            (app, app.app_id, result.transaction_ids[0].clone())
        }
        _ => return Err("Expected Create result".into()),
    };

    algorand_fixture
        .wait_for_indexer_transaction(&tx_id)
        .await?;

    // Deploy update with same metadata but different version
    let metadata_2 = AppDeployMetadata {
        version: String::from("2.0"),
        ..metadata.clone()
    };
    let deployment_2 = get_testing_app_deploy_params(
        &test_account,
        &metadata_2,
        Some(2),
        None,
        Some(OnUpdate::Update),
        None,
    )
    .await?;

    let result_2 = app_deployer.deploy(deployment_2).await?;
    let (app_2, update_result) = match result_2 {
        AppDeployResult::Update { app, result } => (app, result),
        _ => return Err("Expected Update result".into()),
    };

    assert_eq!(app_2.app_id, app_1_id);
    assert_eq!(app_2.created_metadata, metadata);
    assert_eq!(app_2.created_round, app_1.created_round);
    assert_ne!(app_2.updated_round, app_2.created_round);
    assert_eq!(
        app_2.updated_round,
        update_result.confirmations[0].confirmed_round.unwrap()
    );
    assert_eq!(app_2.name, metadata_2.name);
    assert_eq!(app_2.version, metadata_2.version);
    assert_eq!(app_2.updatable, metadata_2.updatable);
    assert_eq!(app_2.deletable, metadata_2.deletable);
    assert!(!app_2.deleted);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_deploy_update_to_immutable_app_fails(#[future] fixture: FixtureResult) -> TestResult {
    let Fixture {
        test_account,
        mut app_deployer,
        algorand_fixture,
        ..
    } = fixture.await?;

    // Deploy initial app (immutable)
    let metadata = get_metadata(AppDeployMetadataParams {
        updatable: Some(false),
        ..Default::default()
    });
    let deployment_1 =
        get_testing_app_deploy_params(&test_account, &metadata, None, None, None, None).await?;

    let result_1 = app_deployer.deploy(deployment_1).await?;

    algorand_fixture
        .wait_for_indexer_transaction(&match result_1 {
            AppDeployResult::Create { result, .. } => result.transaction_ids[0].clone(),
            _ => return Err("Expected Create result".into()),
        })
        .await?;

    // Attempt to update (should fail because app is immutable)
    let metadata_2 = get_metadata(AppDeployMetadataParams {
        version: Some(String::from("2.0")),
        updatable: Some(false),
        ..Default::default()
    });
    let deployment_2 = get_testing_app_deploy_params(
        &test_account,
        &metadata_2,
        Some(2),
        None,
        Some(OnUpdate::Update),
        None,
    )
    .await?;

    let result_2 = app_deployer.deploy(deployment_2).await;

    assert!(result_2.is_err());
    let error_message = result_2.unwrap_err().to_string();
    assert!(error_message.contains("logic eval error: assert failed"));

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_deploy_failure_for_updated_app_when_on_update_fail(
    #[future] fixture: FixtureResult,
) -> TestResult {
    let Fixture {
        test_account,
        mut app_deployer,
        algorand_fixture,
        ..
    } = fixture.await?;

    // Deploy initial app
    let metadata = get_metadata(AppDeployMetadataParams {
        ..Default::default()
    });
    let deployment_1 =
        get_testing_app_deploy_params(&test_account, &metadata, None, None, None, None).await?;

    let result_1 = app_deployer.deploy(deployment_1).await?;

    algorand_fixture
        .wait_for_indexer_transaction(&match result_1 {
            AppDeployResult::Create { result, .. } => result.transaction_ids[0].clone(),
            _ => return Err("Expected Create result".into()),
        })
        .await?;

    // Attempt to deploy with changes but OnUpdate::Fail
    let deployment_2 = get_testing_app_deploy_params(
        &test_account,
        &metadata,
        Some(2),
        None,
        Some(OnUpdate::Fail),
        None,
    )
    .await?;

    let result_2 = app_deployer.deploy(deployment_2).await;

    assert!(result_2.is_err());
    let error_message = result_2.unwrap_err().to_string();
    assert!(error_message.contains("Executing the fail on update strategy, stopping deployment"));

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_deploy_replacement_to_deletable_updated_app(
    #[future] fixture: FixtureResult,
) -> TestResult {
    let Fixture {
        test_account,
        mut app_deployer,
        algorand_fixture,
        ..
    } = fixture.await?;

    // Deploy initial app (deletable)
    let metadata = get_metadata(AppDeployMetadataParams {
        deletable: Some(true),
        ..Default::default()
    });
    let deployment_1 =
        get_testing_app_deploy_params(&test_account, &metadata, None, None, None, None).await?;

    let result_1 = app_deployer.deploy(deployment_1).await?;
    let app_1_id = match &result_1 {
        AppDeployResult::Create { app, .. } => app.app_id,
        _ => return Err("Expected Create result".into()),
    };

    algorand_fixture
        .wait_for_indexer_transaction(&match result_1 {
            AppDeployResult::Create { result, .. } => result.transaction_ids[0].clone(),
            _ => return Err("Expected Create result".into()),
        })
        .await?;

    // Deploy replacement with different code
    let metadata_2 = get_metadata(AppDeployMetadataParams {
        version: Some(String::from("2.0")),
        deletable: Some(true),
        ..Default::default()
    });
    let deployment_2 = get_testing_app_deploy_params(
        &test_account,
        &metadata_2,
        Some(2),
        None,
        Some(OnUpdate::Replace),
        None,
    )
    .await?;

    let result_2 = app_deployer.deploy(deployment_2).await?;
    let (app_2, create_result) = match result_2 {
        AppDeployResult::Replace { app, result, .. } => (app, result),
        _ => return Err("Expected Replace result".into()),
    };

    assert_ne!(app_2.app_id, app_1_id);
    assert_eq!(app_2.created_metadata, metadata_2);
    assert_eq!(app_2.created_round, app_2.updated_round);
    assert_eq!(
        app_2.created_round,
        create_result.confirmations[0].confirmed_round.unwrap()
    );
    assert_eq!(app_2.name, metadata_2.name);
    assert_eq!(app_2.version, metadata_2.version);
    assert_eq!(app_2.updatable, metadata_2.updatable);
    assert_eq!(app_2.deletable, metadata_2.deletable);
    assert!(!app_2.deleted);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_deploy_failure_for_replacement_of_permanent_updated_app(
    #[future] fixture: FixtureResult,
) -> TestResult {
    let Fixture {
        test_account,
        mut app_deployer,
        algorand_fixture,
        ..
    } = fixture.await?;

    // Deploy initial app (permanent)
    let metadata = get_metadata(AppDeployMetadataParams {
        deletable: Some(false),
        ..Default::default()
    });
    let deployment_1 =
        get_testing_app_deploy_params(&test_account, &metadata, None, None, None, None).await?;

    let result_1 = app_deployer.deploy(deployment_1).await?;

    algorand_fixture
        .wait_for_indexer_transaction(&match result_1 {
            AppDeployResult::Create { result, .. } => result.transaction_ids[0].clone(),
            _ => return Err("Expected Create result".into()),
        })
        .await?;

    // Attempt to replace permanent app (should fail)
    let metadata_2 = get_metadata(AppDeployMetadataParams {
        version: Some(String::from("2.0")),
        deletable: Some(false),
        ..Default::default()
    });
    let deployment_2 = get_testing_app_deploy_params(
        &test_account,
        &metadata_2,
        Some(2),
        None,
        Some(OnUpdate::Replace),
        None,
    )
    .await?;

    let result_2 = app_deployer.deploy(deployment_2).await;

    assert!(result_2.is_err());
    let error_message = result_2.unwrap_err().to_string();
    assert!(error_message.contains("logic eval error: assert failed"));

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_deploy_replacement_of_deletable_schema_broken_app(
    #[future] fixture: FixtureResult,
) -> TestResult {
    let Fixture {
        test_account,
        mut app_deployer,
        algorand_fixture,
        ..
    } = fixture.await?;

    // Deploy initial app (deletable)
    let metadata = get_metadata(AppDeployMetadataParams {
        deletable: Some(true),
        ..Default::default()
    });
    let deployment_1 =
        get_testing_app_deploy_params(&test_account, &metadata, None, None, None, None).await?;

    let result_1 = app_deployer.deploy(deployment_1).await?;
    let (app_1, tx_id) = match result_1 {
        AppDeployResult::Create { app, result } => (app, result.transaction_ids[0].clone()),
        _ => return Err("Expected Create result".into()),
    };

    algorand_fixture
        .wait_for_indexer_transaction(&tx_id)
        .await?;

    // Deploy replacement with schema break
    let metadata_2 = get_metadata(AppDeployMetadataParams {
        version: Some(String::from("2.0")),
        deletable: Some(true),
        ..Default::default()
    });
    let deployment_2 = get_testing_app_deploy_params(
        &test_account,
        &metadata_2,
        None,
        Some(OnSchemaBreak::Replace),
        None,
        Some(true), // break_schema
    )
    .await?;

    let result_2 = app_deployer.deploy(deployment_2).await?;
    let (app_2, create_result) = match result_2 {
        AppDeployResult::Replace { app, result, .. } => (app, result),
        _ => return Err("Expected Replace result".into()),
    };

    // Verify the app was replaced
    assert_ne!(app_2.app_id, app_1.app_id);
    assert_eq!(app_2.created_metadata, metadata_2);
    assert_eq!(app_2.created_round, app_2.updated_round);
    assert_eq!(
        app_2.created_round,
        create_result.confirmations[0].confirmed_round.unwrap()
    );
    assert_eq!(app_2.name, metadata_2.name);
    assert_eq!(app_2.version, metadata_2.version);
    assert!(!app_2.deleted);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_deploy_replacement_to_schema_broken_permanent_app_fails(
    #[future] fixture: FixtureResult,
) -> TestResult {
    let Fixture {
        test_account,
        mut app_deployer,
        algorand_fixture,
        ..
    } = fixture.await?;

    // Deploy initial app (permanent)
    let metadata = get_metadata(AppDeployMetadataParams {
        deletable: Some(false),
        ..Default::default()
    });
    let deployment_1 =
        get_testing_app_deploy_params(&test_account, &metadata, None, None, None, None).await?;

    let result_1 = app_deployer.deploy(deployment_1).await?;

    algorand_fixture
        .wait_for_indexer_transaction(&match result_1 {
            AppDeployResult::Create { result, .. } => result.transaction_ids[0].clone(),
            _ => return Err("Expected Create result".into()),
        })
        .await?;

    // Attempt to replace permanent app with schema break (should fail)
    let metadata_2 = get_metadata(AppDeployMetadataParams {
        version: Some(String::from("2.0")),
        deletable: Some(false),
        ..Default::default()
    });
    let deployment_2 = get_testing_app_deploy_params(
        &test_account,
        &metadata_2,
        None,
        Some(OnSchemaBreak::Replace),
        None,
        Some(true), // break_schema
    )
    .await?;

    let result_2 = app_deployer.deploy(deployment_2).await;

    assert!(result_2.is_err());
    let error_message = result_2.unwrap_err().to_string();
    assert!(error_message.contains("logic eval error: assert failed"));

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_deploy_failure_for_replacement_of_schema_broken_app_when_on_schema_break_fail(
    #[future] fixture: FixtureResult,
) -> TestResult {
    let Fixture {
        test_account,
        mut app_deployer,
        algorand_fixture,
        ..
    } = fixture.await?;

    // Deploy initial app
    let metadata = get_metadata(AppDeployMetadataParams {
        ..Default::default()
    });
    let deployment_1 =
        get_testing_app_deploy_params(&test_account, &metadata, None, None, None, None).await?;

    let result_1 = app_deployer.deploy(deployment_1).await?;

    algorand_fixture
        .wait_for_indexer_transaction(&match result_1 {
            AppDeployResult::Create { result, .. } => result.transaction_ids[0].clone(),
            _ => return Err("Expected Create result".into()),
        })
        .await?;

    // Attempt to deploy with schema break but OnSchemaBreak::Fail
    let deployment_2 = get_testing_app_deploy_params(
        &test_account,
        &metadata,
        None,
        Some(OnSchemaBreak::Fail),
        None,
        Some(true), // break_schema
    )
    .await?;

    let result_2 = app_deployer.deploy(deployment_2).await;

    assert!(result_2.is_err());
    let error_message = result_2.unwrap_err().to_string();
    assert!(
        error_message.contains("Executing the fail on schema break strategy, stopping deployment")
    );

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_do_nothing_if_deploying_app_with_no_changes(
    #[future] fixture: FixtureResult,
) -> TestResult {
    let Fixture {
        test_account,
        mut app_deployer,
        algorand_fixture,
        ..
    } = fixture.await?;

    // Deploy initial app
    let metadata = get_metadata(AppDeployMetadataParams {
        ..Default::default()
    });
    let deployment =
        get_testing_app_deploy_params(&test_account, &metadata, None, None, None, None).await?;

    let result_1 = app_deployer.deploy(deployment.clone()).await?;
    let (app_1, tx_id) = match result_1 {
        AppDeployResult::Create { app, result } => (app, result.transaction_ids[0].clone()),
        _ => return Err("Expected Create result".into()),
    };

    algorand_fixture
        .wait_for_indexer_transaction(&tx_id)
        .await?;

    // Deploy again with no changes
    let result_2 = app_deployer.deploy(deployment).await?;
    let app_2 = match result_2 {
        AppDeployResult::Nothing { app } => app,
        _ => return Err("Expected Nothing result".into()),
    };

    assert_eq!(app_2.app_id, app_1.app_id);
    assert_eq!(app_2.app_address, app_1.app_address);
    assert_eq!(app_2.created_metadata, metadata);
    assert_eq!(app_2.created_round, app_1.created_round);
    assert_eq!(app_2.updated_round, app_1.updated_round);
    assert_eq!(app_2.name, metadata.name);
    assert_eq!(app_2.version, metadata.version);
    assert_eq!(app_2.updatable, metadata.updatable);
    assert_eq!(app_2.deletable, metadata.deletable);
    assert!(!app_2.deleted);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_deploy_append_for_schema_broken_app_when_on_schema_break_append_app(
    #[future] fixture: FixtureResult,
) -> TestResult {
    let Fixture {
        test_account,
        mut app_deployer,
        algorand_fixture,
        ..
    } = fixture.await?;

    // Deploy initial app
    let metadata = get_metadata(AppDeployMetadataParams {
        ..Default::default()
    });
    let deployment_1 =
        get_testing_app_deploy_params(&test_account, &metadata, None, None, None, None).await?;

    let result_1 = app_deployer.deploy(deployment_1).await?;
    let (app_1, tx_id) = match result_1 {
        AppDeployResult::Create { app, result } => (app, result.transaction_ids[0].clone()),
        _ => return Err("Expected Create result".into()),
    };

    algorand_fixture
        .wait_for_indexer_transaction(&tx_id)
        .await?;

    // Deploy with schema break and OnSchemaBreak::Append
    let deployment_2 = get_testing_app_deploy_params(
        &test_account,
        &metadata,
        None,
        Some(OnSchemaBreak::Append),
        None,
        Some(true), // break_schema
    )
    .await?;

    let result_2 = app_deployer.deploy(deployment_2).await?;
    let (app_2, create_result) = match result_2 {
        AppDeployResult::Create { app, result } => (app, result),
        _ => return Err("Expected Create result".into()),
    };

    assert_ne!(app_2.app_id, app_1.app_id);
    assert_ne!(app_2.created_round, app_1.created_round);
    assert_eq!(
        app_2.created_round,
        create_result.confirmations[0].confirmed_round.unwrap()
    );
    assert_eq!(app_2.created_round, app_2.updated_round);
    assert_eq!(app_2.name, metadata.name);
    assert_eq!(app_2.version, metadata.version);
    assert!(!app_2.deleted);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_deploy_append_for_update_app_when_on_update_append_app(
    #[future] fixture: FixtureResult,
) -> TestResult {
    let Fixture {
        test_account,
        mut app_deployer,
        algorand_fixture,
        ..
    } = fixture.await?;

    // Deploy initial app
    let metadata = get_metadata(AppDeployMetadataParams {
        ..Default::default()
    });
    let deployment_1 =
        get_testing_app_deploy_params(&test_account, &metadata, None, None, None, None).await?;

    let result_1 = app_deployer.deploy(deployment_1).await?;
    let (app_1, tx_id) = match result_1 {
        AppDeployResult::Create { app, result } => (app, result.transaction_ids[0].clone()),
        _ => return Err("Expected Create result".into()),
    };

    algorand_fixture
        .wait_for_indexer_transaction(&tx_id)
        .await?;

    // Deploy with code changes and OnUpdate::Append
    let metadata_2 = get_metadata(AppDeployMetadataParams {
        version: Some(String::from("2.0")),
        ..Default::default()
    });
    let deployment_2 = get_testing_app_deploy_params(
        &test_account,
        &metadata_2,
        Some(3), // Different code injection value
        None,
        Some(OnUpdate::Append),
        None,
    )
    .await?;

    let result_2 = app_deployer.deploy(deployment_2).await?;
    let (app_2, create_result) = match result_2 {
        AppDeployResult::Create { app, result } => (app, result),
        _ => return Err("Expected Create result".into()),
    };

    assert_ne!(app_2.app_id, app_1.app_id);
    assert_ne!(app_2.created_round, app_1.created_round);
    assert_eq!(
        app_2.created_round,
        create_result.confirmations[0].confirmed_round.unwrap()
    );
    assert_eq!(app_2.created_metadata, metadata_2);
    assert_eq!(app_2.name, metadata_2.name);
    assert_eq!(app_2.version, metadata_2.version);
    assert!(!app_2.deleted);

    Ok(())
}

struct Fixture {
    test_account: Address,
    algorand_fixture: AlgorandFixture,
    app_manager: AppManager,
    transaction_sender: TransactionSender,
    app_deployer: AppDeployer,
}

type FixtureResult = Result<Fixture, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Default, Clone)]
struct AppDeployMetadataParams {
    name: Option<String>,
    version: Option<String>,
    updatable: Option<bool>,
    deletable: Option<bool>,
}

fn get_metadata(params: AppDeployMetadataParams) -> AppDeployMetadata {
    AppDeployMetadata {
        name: params.name.unwrap_or(String::from("MY_APP")),
        version: params.version.unwrap_or(String::from("1.0")),
        updatable: Some(params.updatable.unwrap_or(false)),
        deletable: Some(params.deletable.unwrap_or(false)),
    }
}

async fn get_testing_app_deploy_params(
    sender: &Address,
    metadata: &AppDeployMetadata,
    code_injection_value: Option<u64>,
    on_schema_break: Option<OnSchemaBreak>,
    on_update: Option<OnUpdate>,
    break_schema: Option<bool>,
) -> Result<AppDeployParams, Box<dyn std::error::Error + Send + Sync>> {
    let app_spec: serde_json::Value = serde_json::from_str(testing_app::APPLICATION)?;

    let approval_program_b64 = app_spec["source"]["approval"]
        .as_str()
        .ok_or("Missing approval program")?;
    let clear_program_b64 = app_spec["source"]["clear"]
        .as_str()
        .ok_or("Missing clear program")?;

    let approval_program = String::from_utf8(BASE64_STANDARD.decode(approval_program_b64)?)?;
    let clear_program = String::from_utf8(BASE64_STANDARD.decode(clear_program_b64)?)?;

    let mut template_params = HashMap::new();
    template_params.insert(
        "TMPL_VALUE".to_string(),
        TealTemplateValue::Int(code_injection_value.unwrap_or(1)),
    );

    let global_schema = if break_schema.unwrap_or(false) {
        algokit_transact::StateSchema {
            num_byte_slices: 3, // +1 from default 2
            num_uints: 3,
        }
    } else {
        algokit_transact::StateSchema {
            num_byte_slices: 2,
            num_uints: 3,
        }
    };

    Ok(AppDeployParams {
        metadata: metadata.clone(),
        deploy_time_params: Some(template_params),
        on_schema_break,
        on_update,
        create_params: CreateParams::AppCreateCall(DeployAppCreateParams {
            sender: sender.clone(),
            approval_program: AppProgram::Teal(approval_program),
            clear_state_program: AppProgram::Teal(clear_program),
            global_state_schema: Some(global_schema),
            local_state_schema: Some(algokit_transact::StateSchema {
                num_byte_slices: 2,
                num_uints: 1,
            }),
            ..Default::default()
        }),
        update_params: UpdateParams::AppUpdateCall(DeployAppUpdateParams {
            sender: sender.clone(),
            ..Default::default()
        }),
        delete_params: DeleteParams::AppDeleteCall(DeployAppDeleteParams {
            sender: sender.clone(),
            ..Default::default()
        }),
        existing_deployments: None,
        ignore_cache: None,
        send_params: SendParams {
            max_rounds_to_wait_for_confirmation: Some(100),
        },
    })
}

async fn get_testing_app_create_params(
    app_manager: &AppManager,
    sender: &Address,
    metadata: &AppDeployMetadata,
) -> Result<AppCreateParams, Box<dyn std::error::Error + Send + Sync>> {
    let app_spec: serde_json::Value = serde_json::from_str(testing_app::APPLICATION)?;

    // Decode base64 to get TEAL source
    let approval_program_b64 = app_spec["source"]["approval"]
        .as_str()
        .ok_or("Missing approval program")?;
    let clear_program_b64 = app_spec["source"]["clear"]
        .as_str()
        .ok_or("Missing clear program")?;

    let approval_program = String::from_utf8(BASE64_STANDARD.decode(approval_program_b64)?)?;
    let clear_program = String::from_utf8(BASE64_STANDARD.decode(clear_program_b64)?)?;

    // Apply template parameters
    let template_params = HashMap::from([(
        "TMPL_VALUE".to_string(),
        TealTemplateValue::String("1".to_string()),
    )]);

    // Compile TEAL with template substitution
    let approval_compiled = app_manager
        .compile_teal_template(
            &approval_program,
            Some(&template_params),
            Some(&DeploymentMetadata {
                updatable: metadata.updatable,
                deletable: metadata.deletable,
            }),
        )
        .await?;
    let clear_compiled = app_manager
        .compile_teal_template(&clear_program, Some(&template_params), None)
        .await?;

    // Create note with metadata
    let note_data = serde_json::json!({
        "name": metadata.name,
        "version": metadata.version,
        "updatable": metadata.updatable,
        "deletable": metadata.deletable
    });
    let note = format!("ALGOKIT_DEPLOYER:j{}", note_data);

    Ok(AppCreateParams {
        sender: sender.clone(),
        note: Some(note.into_bytes()),
        approval_program: approval_compiled.compiled_base64_to_bytes,
        clear_state_program: clear_compiled.compiled_base64_to_bytes,
        on_complete: OnApplicationComplete::NoOp,
        args: None,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        global_state_schema: Some(algokit_transact::StateSchema {
            num_byte_slices: 2,
            num_uints: 3,
        }),
        local_state_schema: Some(algokit_transact::StateSchema {
            num_byte_slices: 2,
            num_uints: 1,
        }),
        extra_program_pages: None,
        ..Default::default()
    })
}

async fn get_deploy_params_for_replacing_app_using_abi_methods(
    sender: &Address,
    metadata: &AppDeployMetadata,
    break_schema: bool,
) -> Result<AppDeployParams, Box<dyn std::error::Error + Send + Sync>> {
    let app_spec: serde_json::Value =
        serde_json::from_str(abi_create_and_delete::APPLICATION_ARC56)?;

    let approval_program_b64 = app_spec["source"]["approval"]
        .as_str()
        .ok_or("Missing approval program")?;
    let clear_program_b64 = app_spec["source"]["clear"]
        .as_str()
        .ok_or("Missing clear program")?;

    let approval_program = String::from_utf8(BASE64_STANDARD.decode(approval_program_b64)?)?;
    let clear_program = String::from_utf8(BASE64_STANDARD.decode(clear_program_b64)?)?;

    let global_schema = if break_schema {
        algokit_transact::StateSchema {
            num_byte_slices: 3, // +1 from default 2
            num_uints: 3,
        }
    } else {
        algokit_transact::StateSchema {
            num_byte_slices: 2,
            num_uints: 3,
        }
    };

    Ok(AppDeployParams {
        metadata: metadata.clone(),
        deploy_time_params: None,
        on_schema_break: Some(OnSchemaBreak::Replace),
        on_update: Some(OnUpdate::Replace),
        create_params: CreateParams::AppCreateMethodCall(DeployAppCreateMethodCallParams {
            sender: sender.clone(),
            method: ABIMethod::from_str("create(pay)string")?,
            approval_program: AppProgram::Teal(approval_program),
            clear_state_program: AppProgram::Teal(clear_program),
            global_state_schema: Some(global_schema),
            local_state_schema: Some(algokit_transact::StateSchema {
                num_byte_slices: 2,
                num_uints: 1,
            }),
            args: vec![AppMethodCallArg::Payment(PaymentParams {
                sender: sender.clone(),
                receiver: sender.clone(),
                amount: 1000,
                ..Default::default()
            })],
            ..Default::default()
        }),
        update_params: UpdateParams::AppUpdateCall(DeployAppUpdateParams {
            sender: sender.clone(),
            ..Default::default()
        }),
        delete_params: DeleteParams::AppDeleteMethodCall(DeployAppDeleteMethodCallParams {
            sender: sender.clone(),
            method: ABIMethod::from_str("delete(pay)string")?,
            args: vec![AppMethodCallArg::Payment(PaymentParams {
                sender: sender.clone(),
                receiver: sender.clone(),
                amount: 2000,
                ..Default::default()
            })],
            ..Default::default()
        }),
        existing_deployments: None,
        ignore_cache: None,
        send_params: SendParams {
            max_rounds_to_wait_for_confirmation: Some(100),
        },
    })
}

#[rstest]
#[tokio::test]
async fn test_replacing_app_using_abi_methods(#[future] fixture: FixtureResult) -> TestResult {
    let Fixture {
        test_account,
        mut app_deployer,
        algorand_fixture,
        ..
    } = fixture.await?;

    // Deploy initial app (deletable)
    let metadata = get_metadata(AppDeployMetadataParams {
        deletable: Some(true),
        ..Default::default()
    });
    let deployment_1 =
        get_deploy_params_for_replacing_app_using_abi_methods(&test_account, &metadata, false)
            .await?;

    let result_1 = app_deployer.deploy(deployment_1).await?;
    let app_1_id = match &result_1 {
        AppDeployResult::Create { app, .. } => app.app_id,
        _ => return Err("Expected Create result".into()),
    };

    algorand_fixture
        .wait_for_indexer_transaction(&match result_1 {
            AppDeployResult::Create { result, .. } => result.transaction_ids[0].clone(),
            _ => return Err("Expected Create result".into()),
        })
        .await?;

    // Deploy replacement with different code
    let metadata_2 = get_metadata(AppDeployMetadataParams {
        version: Some(String::from("2.0")),
        deletable: Some(true),
        ..Default::default()
    });
    let deployment_2 =
        get_deploy_params_for_replacing_app_using_abi_methods(&test_account, &metadata_2, true)
            .await?;

    let result_2 = app_deployer.deploy(deployment_2).await?;
    let (app_2, create_result) = match result_2 {
        AppDeployResult::Replace { app, result, .. } => (app, result),
        _ => return Err("Expected Replace result".into()),
    };

    assert_ne!(app_2.app_id, app_1_id);
    assert_eq!(app_2.created_metadata, metadata_2);
    assert_eq!(app_2.created_round, app_2.updated_round);
    assert_eq!(
        app_2.created_round,
        create_result.confirmations[0].confirmed_round.unwrap()
    );
    assert_eq!(app_2.name, metadata_2.name);
    assert_eq!(app_2.version, metadata_2.version);
    assert_eq!(app_2.updatable, metadata_2.updatable);
    assert_eq!(app_2.deletable, metadata_2.deletable);
    assert!(!app_2.deleted);

    // Check ABI return values
    assert!(create_result.create_abi_return.is_some());
    let create_abi_return = create_result.create_abi_return.unwrap();
    assert!(create_abi_return.return_value.is_some());
    if let Some(algokit_abi::ABIValue::String(s)) = create_abi_return.return_value {
        assert_eq!(s, "created");
    } else {
        panic!("Expected string return value from create method");
    }

    assert!(create_result.delete_abi_return.is_some());
    let delete_abi_return = create_result.delete_abi_return.unwrap();
    assert!(delete_abi_return.return_value.is_some());
    if let Some(algokit_abi::ABIValue::String(s)) = delete_abi_return.return_value {
        assert_eq!(s, "deleted");
    } else {
        panic!("Expected string return value from delete method");
    }

    Ok(())
}
