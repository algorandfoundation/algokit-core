// Polytest Suite: GET v2_accounts_ADDRESS_assets_ASSET-ID
// Polytest Group: Common Tests
#[tokio::test]
#[ignore = "requires localnet"]
async fn basic_request_and_response_validation() {
    let manifest = algokit_localnet_testing::load_manifest();
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    client
        .account_asset_information(&manifest.address, manifest.asset_id)
        .await
        .expect("account_asset_information request failed");

    algokit_localnet_testing::validate_response("AccountAssetResponse", &capture.last_body())
        .expect("account_asset_information response does not match schema");
}
