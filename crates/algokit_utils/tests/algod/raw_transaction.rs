use crate::common::{
    AlgorandFixtureResult, NetworkType, TestAccountConfig, TestResult, algorand_fixture,
};
use algokit_transact::{
    AlgorandMsgpack, PaymentTransactionBuilder, Transaction, TransactionHeaderBuilder,
};
use algokit_utils::TransactionSigner;
use rstest::rstest;
use std::convert::TryInto;

#[rstest]
#[tokio::test]
async fn test_raw_transaction_broadcast(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;

    // Create algod client using ClientManager
    let algod_client = algorand_fixture.algod.clone();

    // Create account manager and generate test accounts
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

    let sender = algorand_fixture
        .generate_account(Some(sender_config))
        .await?;

    let receiver = algorand_fixture
        .generate_account(Some(receiver_config))
        .await?;

    let sender_account = sender.account();
    let receiver_account = receiver.account();

    // Get transaction parameters
    let params = algod_client.transaction_params().await?;

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
        .build()?;

    // Build payment transaction
    let payment_fields = PaymentTransactionBuilder::default()
        .header(header)
        .receiver(receiver_account.address())
        .amount(500_000) // 0.5 ALGO
        .build_fields()?;

    let transaction = Transaction::Payment(payment_fields);
    let signed_transaction = sender.sign_transaction(&transaction).await?;

    let signed_bytes = signed_transaction.encode().unwrap();

    let response = algod_client.raw_transaction(signed_bytes).await?;

    assert!(
        !response.tx_id.is_empty(),
        "Response should contain a transaction ID"
    );

    Ok(())
}
