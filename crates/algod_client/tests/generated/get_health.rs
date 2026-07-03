// Polytest Suite: GET health
// Polytest Group: Common Tests
#[tokio::test]
#[ignore = "requires localnet"]
async fn basic_request_and_response_validation() {
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    // /health has no response body to validate; a successful call is the assertion.
    client.health_check().await.expect("health request failed");
}
