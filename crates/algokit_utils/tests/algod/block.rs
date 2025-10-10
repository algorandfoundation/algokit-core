// Block tests
// These tests demonstrate the integration test structure and API communication

use algokit_utils::ClientManager;

use crate::common::logging::init_test_logging;

#[tokio::test]
async fn test_block_endpoint() {
    init_test_logging();

    let config =
        ClientManager::get_algonode_config("testnet", algokit_utils::AlgorandService::Algod);
    let algod_client = ClientManager::get_algod_client(&config).unwrap();
    let large_block_with_state_proof_txns = 24098947;
    let block_response = algod_client
        .get_block(large_block_with_state_proof_txns, Some(false))
        .await
        .unwrap();

    assert!(block_response.cert.is_some());
    assert!(block_response.block.state_proof_tracking.is_some());
    assert!(block_response.block.transactions.is_some());

    // Validate deeply nested signed transaction fields are present and
    // leverage transact crate model
    let transactions = block_response
        .block
        .transactions
        .as_ref()
        .expect("expected transactions");
    assert!(!transactions.is_empty());
    assert_eq!(
        transactions[0]
            .signed_transaction
            .transaction
            .sender()
            .as_str(),
        "XM6FEYVJ2XDU2IBH4OT6VZGW75YM63CM4TC6AV6BD3JZXFJUIICYTVB5EU"
    );
}
