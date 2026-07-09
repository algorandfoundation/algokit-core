// Polytest Suite: GET v2_assets_ASSET-ID
// Polytest Group: Common Tests
#[tokio::test]
async fn basic_request_and_response_validation() {
    let manifest = algokit_localnet_testing::load_manifest();
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    client
        .asset_by_id(manifest.asset_id)
        .await
        .expect("asset_by_id request failed");

    algokit_localnet_testing::validate_response("Asset", &capture.last_body())
        .expect("asset_by_id response does not match schema");
}
