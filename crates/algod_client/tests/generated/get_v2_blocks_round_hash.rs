// Polytest Suite: GET v2_blocks_ROUND_hash
// Polytest Group: Common Tests
#[tokio::test]
#[ignore = "requires localnet"]
async fn basic_request_and_response_validation() {
    let manifest = algokit_localnet_testing::load_manifest();
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    client
        .block_hash(manifest.confirmed_round)
        .await
        .expect("block_hash request failed");

    algokit_localnet_testing::validate_response("BlockHashResponse", &capture.last_body())
        .expect("block_hash response does not match schema");
}
