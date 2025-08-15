use crate::common::init_test_logging;
use algokit_utils::{AssetCreateParams, CommonParams};
use algokit_utils::{AssetOptInParams, AssetTransferParams, testing::*};
use std::sync::Arc;

#[tokio::test]
async fn test_asset_transfer_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");

    let mut composer = context.composer.clone();

    let asset_creator_address = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    composer
        .add_asset_create(AssetCreateParams {
            common_params: CommonParams {
                sender: asset_creator_address.clone(),
                ..Default::default()
            },
            total: 10,
            decimals: Some(0),
            default_frozen: Some(false),
            asset_name: None,
            unit_name: None,
            url: None,
            metadata_hash: None,
            manager: None,
            reserve: None,
            freeze: None,
            clawback: None,
        })
        .expect("Failed to add asset create transaction");

    let asset_create_result = composer
        .send(None)
        .await
        .expect("Failed to send asset create");
    let asset_id = asset_create_result.confirmations[0]
        .asset_id
        .expect("Failed to get asset ID");

    let mut composer = context.composer.clone();

    let asset_receiver = fixture
        .generate_account(None)
        .await
        .expect("Failed to generate asset receiver");
    let asset_receive_address = asset_receiver
        .account()
        .expect("Failed to get receiver account")
        .address();

    composer
        .add_asset_opt_in(AssetOptInParams {
            common_params: CommonParams {
                sender: asset_receive_address.clone(),
                signer: Some(Arc::new(asset_receiver)),
                ..Default::default()
            },
            asset_id,
        })
        .expect("Failed to add asset opt in transaction");
    composer
        .add_asset_transfer(AssetTransferParams {
            common_params: CommonParams {
                sender: asset_creator_address.clone(),
                ..Default::default()
            },
            asset_id,
            receiver: asset_receive_address.clone(),
            amount: 1,
        })
        .expect("Failed to add asset transfer transaction");

    let send_result = composer
        .send(None)
        .await
        .expect("Failed to send transaction group");

    let asset_opt_in_transaction = &send_result.confirmations[0].txn.transaction;
    let asset_transfer_transaction = &send_result.confirmations[1].txn.transaction;

    match asset_opt_in_transaction {
        algokit_transact::Transaction::AssetTransfer(asset_opt_in_fields) => {
            assert_eq!(
                asset_opt_in_fields.header.sender,
                asset_receive_address.clone(),
                "Account opting in should be the asset user"
            );
            assert_eq!(
                asset_opt_in_fields.receiver,
                asset_receive_address.clone(),
                "Sender and receiver should be the same for opt-in"
            );
            assert_eq!(
                asset_opt_in_fields.asset_id,
                asset_id.clone(),
                "Asset ID should match the created asset"
            );
            assert_eq!(
                asset_opt_in_fields.amount, 0,
                "Amount should be 0 for opt-in"
            );
        }
        _ => panic!("Transaction should be an asset transfer transaction"),
    }
    match asset_transfer_transaction {
        algokit_transact::Transaction::AssetTransfer(asset_transfer_fields) => {
            assert_eq!(
                asset_transfer_fields.header.sender, asset_creator_address,
                "Sender should be the asset creator"
            );
            assert_eq!(
                asset_transfer_fields.receiver, asset_receive_address,
                "Receiver should be the asset user"
            );
            assert_eq!(
                asset_transfer_fields.asset_id, asset_id,
                "Asset ID should match the created asset"
            );
            assert_eq!(asset_transfer_fields.amount, 1, "Amount should be 1");
        }
        _ => panic!("Transaction should be an asset transfer transaction"),
    }
}
