use algokit_transact::{
    AlgorandMsgpack, PaymentTransactionBuilder, Transaction, TransactionHeaderBuilder,
};
use algokit_utils::{
    ClientManager, TransactionSigner,
    testing::{NetworkType, TestAccountConfig, TestAccountManager},
};
use std::convert::TryInto;

use crate::common::init_test_logging;

#[tokio::test]
async fn test_raw_transaction_broadcast() {
    init_test_logging();

    // Create algod client using ClientManager
    let config = ClientManager::get_config_from_environment_or_localnet();
    let algod_client = std::sync::Arc::new(ClientManager::get_algod_client(&config.algod_config));

    // Create account manager and generate test accounts
    let mut account_manager = TestAccountManager::new(algod_client.clone());

    let sender_config = TestAccountConfig {
        initial_funds: 10_000_000, // 10 ALGO
        network_type: NetworkType::LocalNet,
        funding_note: Some("Test sender account".to_string()),
    };

    let receiver_config = TestAccountConfig {
        initial_funds: 1_000_000, // 1 ALGO
        network_type: NetworkType::LocalNet,
        funding_note: Some("Test receiver account".to_string()),
    };

    let sender = account_manager
        .get_test_account(Some(sender_config))
        .await
        .expect("Failed to create sender account");

    let receiver = account_manager
        .get_test_account(Some(receiver_config))
        .await
        .expect("Failed to create receiver account");

    let sender_account = sender.account().expect("Failed to get sender account");
    let receiver_account = receiver.account().expect("Failed to get receiver account");

    // Get transaction parameters
    let params = algod_client
        .transaction_params()
        .await
        .expect("Failed to get transaction params");

    // Convert genesis hash to 32-byte array
    let genesis_hash_bytes: [u8; 32] = params
        .genesis_hash
        .try_into()
        .expect("Genesis hash must be 32 bytes");

    // Build transaction header
    let header = TransactionHeaderBuilder::default()
        .sender(sender_account.address())
        .fee(params.min_fee)
        .first_valid(params.last_round)
        .last_valid(params.last_round + 1000)
        .genesis_id(params.genesis_id.clone())
        .genesis_hash(genesis_hash_bytes)
        .note(b"Test payment transaction".to_vec())
        .build()
        .expect("Failed to build transaction header");

    // Build payment transaction
    let payment_fields = PaymentTransactionBuilder::default()
        .header(header)
        .receiver(receiver_account.address())
        .amount(500_000) // 0.5 ALGO
        .build_fields()
        .expect("Failed to build payment fields");

    let transaction = Transaction::Payment(payment_fields);
    let signed_transaction = sender
        .sign_transaction(&transaction)
        .await
        .expect("Failed to sign transaction");

    let signed_bytes = signed_transaction.encode().unwrap();

    let response = algod_client
        .raw_transaction(signed_bytes)
        .await
        .expect("Failed to broadcast transaction");

    assert!(
        !response.tx_id.is_empty(),
        "Response should contain a transaction ID"
    );
}
