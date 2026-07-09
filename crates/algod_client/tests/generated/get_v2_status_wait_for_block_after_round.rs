// Polytest Suite: GET v2_status_wait-for-block-after_ROUND
// Polytest Group: Common Tests
#[tokio::test]
async fn basic_request_and_response_validation() {
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    // Wait on an already-reached round; waiting on the current round hangs in devmode.
    let last_round = client
        .status()
        .await
        .expect("status request failed")
        .last_round;
    client
        .status_after_block(last_round.saturating_sub(1))
        .await
        .expect("status_after_block request failed");

    algokit_localnet_testing::validate_response("NodeStatusResponse", &capture.last_body())
        .expect("status_after_block response does not match schema");
}
