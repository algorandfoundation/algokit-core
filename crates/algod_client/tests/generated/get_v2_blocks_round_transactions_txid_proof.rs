// Polytest Suite: GET v2_blocks_ROUND_transactions_TXID_proof
// Polytest Group: Common Tests
#[tokio::test]
async fn basic_request_and_response_validation() {
    let manifest = algokit_localnet_testing::load_manifest();
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    client
        .transaction_proof(manifest.confirmed_round, &manifest.txid, None, None)
        .await
        .expect("transaction_proof request failed");

    algokit_localnet_testing::validate_response("TransactionProof", &capture.last_body())
        .expect("transaction_proof response does not match schema");
}
