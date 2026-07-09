// Polytest Suite: GET v2_applications_APPLICATION-ID
// Polytest Group: Common Tests
#[tokio::test]
async fn basic_request_and_response_validation() {
    let manifest = algokit_localnet_testing::load_manifest();
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    client
        .application_by_id(manifest.app_id)
        .await
        .expect("application_by_id request failed");

    algokit_localnet_testing::validate_response("Application", &capture.last_body())
        .expect("application_by_id response does not match schema");
}
