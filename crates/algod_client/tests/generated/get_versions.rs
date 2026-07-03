// Polytest Suite: GET versions
// Polytest Group: Common Tests
#[tokio::test]
#[ignore = "requires localnet"]
async fn basic_request_and_response_validation() {
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    client.version().await.expect("versions request failed");

    algokit_localnet_testing::validate_response("Version", &capture.last_body())
        .expect("versions response does not match schema");
}
