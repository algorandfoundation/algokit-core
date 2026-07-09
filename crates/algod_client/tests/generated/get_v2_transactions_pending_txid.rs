// Polytest Suite: GET v2_transactions_pending_TXID
// Polytest Group: Common Tests
#[tokio::test]
async fn basic_request_and_response_validation() {
    let manifest = algokit_localnet_testing::load_manifest();
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    // Decode-only: the generated client requests msgpack for this endpoint, so the raw body
    // cannot be validated against the JSON schema.
    client
        .pending_transaction_information(&manifest.txid)
        .await
        .expect("pending_transaction_information request failed");
}
