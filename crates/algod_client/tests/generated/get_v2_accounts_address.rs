// Polytest Suite: GET v2_accounts_ADDRESS
// Polytest Group: Common Tests
#[tokio::test]
#[ignore = "requires localnet"]
async fn basic_request_and_response_validation() {
    let manifest = algokit_localnet_testing::load_manifest();
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    client
        .account_information(&manifest.address, None)
        .await
        .expect("account_information request failed");

    algokit_localnet_testing::validate_response("Account", &capture.last_body())
        .expect("account_information response does not match schema");
}
