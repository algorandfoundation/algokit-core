// Polytest Suite: GET v2_status
// Polytest Group: Common Tests
#[tokio::test]
async fn basic_request_and_response_validation() {
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    client.status().await.expect("status request failed");

    algokit_localnet_testing::validate_response("NodeStatusResponse", &capture.last_body())
        .expect("status response does not match schema");
}
