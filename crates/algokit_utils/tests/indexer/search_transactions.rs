use algokit_http_client::DefaultHttpClient;
use algokit_transact::TransactionId;
use algokit_utils::{ClientManager, PaymentParams};
use indexer_client::IndexerClient;
use rstest::rstest;
use std::sync::Arc;

use crate::common::{
    TestResult,
    fixture::{AlgorandFixtureResult, algorand_fixture},
    indexer_helpers::wait_for_indexer_transaction,
    logging::init_test_logging,
};

#[rstest]
#[tokio::test]
async fn finds_sent_transaction(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;

    let sender = algorand_fixture.test_account.account().address();
    let receiver = algorand_fixture.generate_account(None).await?;

    let payment_params = PaymentParams {
        sender: sender.clone(),
        receiver: receiver.account().address(),
        amount: 500_000,
        ..Default::default()
    };

    let mut composer = algorand_fixture.algorand_client.new_group(None);
    composer.add_payment(payment_params).unwrap();
    let result = composer.send(None).await.unwrap();
    let txid = result.confirmations[0].txn.id().unwrap();

    let config = ClientManager::get_config_from_environment_or_localnet();
    let base_url = if let Some(port) = config.indexer_config.port {
        format!("{}:{}", config.indexer_config.server, port)
    } else {
        config.indexer_config.server.clone()
    };
    let indexer_client = IndexerClient::new(Arc::new(DefaultHttpClient::new(&base_url)));

    wait_for_indexer_transaction(&indexer_client, &txid, None)
        .await
        .unwrap();

    let response = indexer_client
        .search_for_transactions(
            None,
            None,
            None,
            None,
            None,
            None,
            Some(&txid),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

    assert!(!response.transactions.is_empty());
    let found_tx = &response.transactions[0];
    assert_eq!(found_tx.id, Some(txid));
    assert_eq!(found_tx.sender, sender.to_string());
    assert_eq!(found_tx.tx_type, "pay");

    if let Some(payment_tx) = &found_tx.payment_transaction {
        assert_eq!(payment_tx.amount, 500_000);
        assert_eq!(
            payment_tx.receiver,
            receiver.account().address().to_string()
        );
    }

    Ok(())
}

#[tokio::test]
async fn handles_invalid_indexer() {
    init_test_logging();

    let indexer_client =
        IndexerClient::new(Arc::new(DefaultHttpClient::new("http://invalid-host:8980")));

    let result = indexer_client
        .search_for_transactions(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None,
        )
        .await;

    assert!(result.is_err());
}
