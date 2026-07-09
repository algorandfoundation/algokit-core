// Polytest Suite: GET genesis
// Polytest Group: Common Tests
#[tokio::test]
async fn basic_request_and_response_validation() {
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    client.genesis().await.expect("genesis request failed");

    algokit_localnet_testing::validate_response("Genesis", &capture.last_body())
        .expect("genesis response does not match schema");
}
