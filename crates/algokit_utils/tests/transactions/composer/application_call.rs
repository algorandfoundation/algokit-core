use crate::common::init_test_logging;
use algokit_transact::{Address, OnApplicationComplete, StateSchema};
use algokit_utils::CommonParams;
use algokit_utils::{
    ApplicationCallParams, ApplicationCreateParams, ApplicationDeleteParams,
    ApplicationUpdateParams, testing::*,
};

#[tokio::test]
async fn test_application_call_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let app_id = create_test_app(context, sender_addr.clone())
        .await
        .expect("Failed to create test app");

    println!("Created test app with ID: {}", app_id);

    let app_call_params = ApplicationCallParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        on_complete: OnApplicationComplete::NoOp,
        args: Some(vec![b"Call".to_vec()]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_call(app_call_params)
        .expect("Failed to add application call");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send application call");
    let confirmation = &result.confirmations[0];

    assert!(
        confirmation.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        confirmation.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );

    let transaction = &confirmation.txn.transaction;

    match transaction {
        algokit_transact::Transaction::ApplicationCall(application_call_fields) => {
            assert_eq!(
                application_call_fields.app_id, app_id,
                "Application ID should match"
            );
            assert_eq!(
                application_call_fields.on_complete,
                OnApplicationComplete::NoOp,
                "On Complete should match"
            );
        }
        _ => panic!("Transaction should be an application call transaction"),
    }
}

#[tokio::test]
async fn test_application_create_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let app_create_params = ApplicationCreateParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        on_complete: OnApplicationComplete::NoOp,
        approval_program: HELLO_WORLD_APPROVAL_PROGRAM.to_vec(),
        clear_state_program: HELLO_WORLD_CLEAR_STATE_PROGRAM.to_vec(),
        global_state_schema: None,
        local_state_schema: None,
        extra_program_pages: None,
        args: Some(vec![b"Create".to_vec()]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_create(app_create_params)
        .expect("Failed to add application create");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send application create");
    let confirmation = &result.confirmations[0];

    assert!(
        confirmation.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        confirmation.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );

    let transaction = &confirmation.txn.transaction;

    match transaction {
        algokit_transact::Transaction::ApplicationCall(application_call_fields) => {
            assert_eq!(
                application_call_fields.app_id, 0,
                "Application ID should be 0 for create"
            );
            assert_eq!(
                application_call_fields.on_complete,
                OnApplicationComplete::NoOp,
                "Clear state program should match"
            );
            assert_eq!(
                application_call_fields.approval_program,
                Some(HELLO_WORLD_APPROVAL_PROGRAM.to_vec()),
                "Approval program should match"
            );
            assert_eq!(
                application_call_fields.clear_state_program,
                Some(HELLO_WORLD_CLEAR_STATE_PROGRAM.to_vec()),
                "Clear state program should match"
            );
        }
        _ => panic!("Transaction should be an application call transaction"),
    }
}

#[tokio::test]
async fn test_application_delete_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let app_id = create_test_app(context, sender_addr.clone())
        .await
        .expect("Failed to create test app");

    let app_delete_params = ApplicationDeleteParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        args: Some(vec![b"Delete".to_vec()]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_delete(app_delete_params)
        .expect("Failed to add application delete");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send application delete");
    let confirmation = &result.confirmations[0];

    assert!(
        confirmation.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        confirmation.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );

    let transaction = &confirmation.txn.transaction;

    match transaction {
        algokit_transact::Transaction::ApplicationCall(application_call_fields) => {
            assert_eq!(
                application_call_fields.app_id, app_id,
                "Application ID should match"
            );
            assert_eq!(
                application_call_fields.on_complete,
                OnApplicationComplete::DeleteApplication,
                "On Complete should be DeleteApplication"
            );
        }
        _ => panic!("Transaction should be an application delete transaction"),
    }
}

#[tokio::test]
async fn test_application_update_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let app_id = create_test_app(context, sender_addr.clone())
        .await
        .expect("Failed to create test app");

    let app_update_params = ApplicationUpdateParams {
        common_params: CommonParams {
            sender: sender_addr.clone(),
            ..Default::default()
        },
        app_id,
        approval_program: HELLO_WORLD_CLEAR_STATE_PROGRAM.to_vec(), // Update the approval program
        clear_state_program: HELLO_WORLD_CLEAR_STATE_PROGRAM.to_vec(),
        args: Some(vec![b"Update".to_vec()]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_update(app_update_params)
        .expect("Failed to add application update");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send application update");

    let confirmation = &result.confirmations[0];

    assert!(
        confirmation.confirmed_round.is_some(),
        "Transaction should be confirmed"
    );
    assert!(
        confirmation.confirmed_round.unwrap() > 0,
        "Confirmed round should be greater than 0"
    );
    let transaction = &confirmation.txn.transaction;

    match transaction {
        algokit_transact::Transaction::ApplicationCall(application_call_fields) => {
            assert_eq!(
                application_call_fields.app_id, app_id,
                "Application ID should match"
            );
            assert_eq!(
                application_call_fields.on_complete,
                OnApplicationComplete::UpdateApplication,
                "On Complete should be UpdateApplication"
            );
            assert_eq!(
                application_call_fields.approval_program,
                Some(HELLO_WORLD_CLEAR_STATE_PROGRAM.to_vec()),
                "Updated approval program should match"
            );
            assert_eq!(
                application_call_fields.clear_state_program,
                Some(HELLO_WORLD_CLEAR_STATE_PROGRAM.to_vec()),
                "Clear state program should match"
            );
        }
        _ => panic!("Transaction should be an application update transaction"),
    }
}

// Raw (non ABI) hello world approval program
const HELLO_WORLD_APPROVAL_PROGRAM: [u8; 18] = [
    10, 128, 7, 72, 101, 108, 108, 111, 44, 32, 54, 26, 0, 80, 176, 129, 1, 67,
];
// Raw (non ABI) hello world clear state program
const HELLO_WORLD_CLEAR_STATE_PROGRAM: [u8; 4] = [10, 129, 1, 67];

async fn create_test_app(context: &AlgorandTestContext, sender: Address) -> Option<u64> {
    let app_create_params = ApplicationCreateParams {
        common_params: CommonParams {
            sender: sender.clone(),
            ..Default::default()
        },
        on_complete: OnApplicationComplete::NoOp,
        approval_program: HELLO_WORLD_APPROVAL_PROGRAM.to_vec(),
        clear_state_program: HELLO_WORLD_CLEAR_STATE_PROGRAM.to_vec(),
        global_state_schema: Some(StateSchema {
            num_uints: 1,
            num_byte_slices: 1,
        }),
        local_state_schema: Some(StateSchema {
            num_uints: 1,
            num_byte_slices: 1,
        }),
        extra_program_pages: None,
        args: Some(vec![b"Create".to_vec()]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();

    composer
        .add_application_create(app_create_params)
        .expect("Failed to add application create");

    let result = composer
        .send(None)
        .await
        .expect("Failed to send application create");

    result.confirmations[0].application_index
}
