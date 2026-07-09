// Polytest Suite: GET v2_transactions_pending
// Polytest Group: Common Tests
#[tokio::test]
#[ignore = "requires localnet"]
async fn basic_request_and_response_validation() {
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    // Decode-only: the generated client requests msgpack for this endpoint, so the raw body
    // cannot be validated against the JSON schema.
    client
        .pending_transactions(None)
        .await
        .expect("pending_transactions request failed");
}
