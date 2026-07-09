// Polytest Suite: GET v2_devmode_blocks_offset
// Polytest Group: Common Tests
#[tokio::test]
#[ignore = "requires localnet"]
async fn basic_request_and_response_validation() {
    let _guard = algokit_localnet_testing::state_lock();
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    // Set an offset first: the endpoint 404s until one is configured.
    client
        .set_block_time_stamp_offset(1)
        .await
        .expect("set_block_time_stamp_offset request failed");

    client
        .block_time_stamp_offset()
        .await
        .expect("block_time_stamp_offset request failed");

    algokit_localnet_testing::validate_response(
        "GetBlockTimeStampOffsetResponse",
        &capture.last_body(),
    )
    .expect("block_time_stamp_offset response does not match schema");

    // Reset to the real clock so later tests see a clean node.
    client
        .set_block_time_stamp_offset(0)
        .await
        .expect("failed to reset block timestamp offset");
}
