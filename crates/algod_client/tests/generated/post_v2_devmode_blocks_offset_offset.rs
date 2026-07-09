// Polytest Suite: POST v2_devmode_blocks_offset_OFFSET
// Polytest Group: Common Tests
#[tokio::test]
async fn basic_request_and_response_validation() {
    let _guard = algokit_localnet_testing::state_lock().await;
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    client
        .set_block_time_stamp_offset(1)
        .await
        .expect("set_block_time_stamp_offset request failed");

    // Reset to the real clock so later tests see a clean node.
    client
        .set_block_time_stamp_offset(0)
        .await
        .expect("failed to reset block timestamp offset");
}
