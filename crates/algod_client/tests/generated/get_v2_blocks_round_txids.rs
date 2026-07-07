// Polytest Suite: GET v2_blocks_ROUND_txids
// Polytest Group: Common Tests
#[tokio::test]
#[ignore = "requires localnet"]
async fn basic_request_and_response_validation() {
    let manifest = algokit_localnet_testing::load_manifest();
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    client
        .block_tx_ids(manifest.confirmed_round)
        .await
        .expect("block_tx_ids request failed");

    algokit_localnet_testing::validate_response("BlockTxidsResponse", &capture.last_body())
        .expect("block_tx_ids response does not match schema");
}
