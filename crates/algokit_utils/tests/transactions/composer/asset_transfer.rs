use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture};
use algokit_utils::AssetCreateParams;
use algokit_utils::{AssetOptInParams, AssetTransferParams};
use rstest::*;

#[rstest]
#[tokio::test]
async fn test_asset_transfer_transaction(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;
    let asset_creator_address = algorand_fixture.test_account.account().address();

    let mut composer = algorand_fixture.algorand_client.new_composer(None);

    composer.add_asset_create(AssetCreateParams {
        sender: asset_creator_address.clone(),
        total: 10,
        decimals: Some(0),
        default_frozen: Some(false),
        ..Default::default()
    })?;

    let asset_create_result = composer.send(None).await?;
    let asset_id = asset_create_result.results[0]
        .confirmation
        .asset_id
        .ok_or("Failed to get asset ID")?;

    let mut composer = algorand_fixture.algorand_client.new_composer(None);

    let asset_receiver = algorand_fixture.generate_account(None).await?;
    let asset_receive_address = asset_receiver.account().address();

    composer.add_asset_opt_in(AssetOptInParams {
        sender: asset_receive_address.clone(),
        asset_id,
        ..Default::default()
    })?;
    composer.add_asset_transfer(AssetTransferParams {
        sender: asset_creator_address.clone(),
        asset_id,
        receiver: asset_receive_address.clone(),
        amount: 1,
        ..Default::default()
    })?;

    let send_result = composer.send(None).await?;

    let asset_opt_in_transaction = &send_result.results[0].confirmation.txn.transaction;
    let asset_transfer_transaction = &send_result.results[1].confirmation.txn.transaction;

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
        _ => return Err("Transaction should be an asset transfer transaction".into()),
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
        _ => return Err("Transaction should be an asset transfer transaction".into()),
    }

    Ok(())
}
