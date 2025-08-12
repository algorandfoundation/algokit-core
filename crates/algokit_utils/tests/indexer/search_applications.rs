use algokit_http_client::DefaultHttpClient;
use algokit_transact::{OnApplicationComplete, StateSchema};
use algokit_utils::{AppCreateParams, ClientManager, CommonParams, testing::*};
use indexer_client::{IndexerClient, apis::Error as IndexerError};
use std::sync::Arc;

use crate::common::init_test_logging;

const HELLO_WORLD_APPROVAL_PROGRAM: [u8; 18] = [
    10, 128, 7, 72, 101, 108, 108, 111, 44, 32, 54, 26, 0, 80, 176, 129, 1, 67,
];

const HELLO_WORLD_CLEAR_STATE_PROGRAM: [u8; 4] = [10, 129, 1, 67];

async fn create_app(
    context: &AlgorandTestContext,
    sender: algokit_transact::Address,
) -> Option<u64> {
    let params = AppCreateParams {
        common_params: CommonParams {
            sender,
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
    composer.add_app_create(params).unwrap();
    let result = composer.send(None).await.unwrap();
    result.confirmations[0].application_index
}

#[tokio::test]
async fn finds_created_application() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.unwrap();
    fixture.new_scope().await.unwrap();
    let context = fixture.context().unwrap();
    let sender = context.test_account.account().unwrap().address();

    let app_id = create_app(context, sender).await.unwrap();

    let config = ClientManager::get_config_from_environment_or_localnet();
    let base_url = if let Some(port) = config.indexer_config.port {
        format!("{}:{}", config.indexer_config.server, port)
    } else {
        config.indexer_config.server.clone()
    };
    let indexer_client = IndexerClient::new(Arc::new(DefaultHttpClient::new(&base_url)));

    wait_for_indexer(
        || {
            let client = indexer_client.clone();
            Box::pin(async move {
                client
                    .search_for_applications(Some(app_id), None, None, None, None)
                    .await
                    .and_then(|response| {
                        if response.applications.is_empty() {
                            Err(IndexerError::Serde("Application not found".to_string()))
                        } else {
                            Ok(())
                        }
                    })
            })
        },
        None,
    )
    .await
    .unwrap();

    let response = indexer_client
        .search_for_applications(Some(app_id), None, None, None, None)
        .await
        .unwrap();

    assert!(!response.applications.is_empty());
    assert_eq!(response.applications[0].id, app_id);
}

#[tokio::test]
async fn handles_invalid_indexer() {
    init_test_logging();

    let indexer_client =
        IndexerClient::new(Arc::new(DefaultHttpClient::new("http://invalid-host:8980")));

    let result = indexer_client
        .search_for_applications(None, None, None, None, None)
        .await;

    assert!(result.is_err());
}
