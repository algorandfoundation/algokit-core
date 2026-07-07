// Polytest Suite: GET v2_accounts_ADDRESS_applications_APPLICATION-ID
// Polytest Group: Common Tests
#[tokio::test]
#[ignore = "requires localnet"]
async fn basic_request_and_response_validation() {
    let manifest = algokit_localnet_testing::load_manifest();
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    // Explicit json format: the generated client defaults to msgpack, which the JSON schema
    // validator cannot parse.
    client
        .account_application_information(
            &manifest.address,
            manifest.app_id,
            Some(algod_client::apis::Format::Json),
        )
        .await
        .expect("account_application_information request failed");

    algokit_localnet_testing::validate_response("AccountApplicationResponse", &capture.last_body())
        .expect("account_application_information response does not match schema");
}
