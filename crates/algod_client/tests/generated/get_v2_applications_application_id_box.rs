// Polytest Suite: GET v2_applications_APPLICATION-ID_box
// Polytest Group: Common Tests
#[tokio::test]
#[ignore = "requires localnet"]
async fn basic_request_and_response_validation() {
    let manifest = algokit_localnet_testing::load_manifest();
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    client
        .application_box_by_name(manifest.box_app_id, &format!("str:{}", manifest.box_name))
        .await
        .expect("application_box_by_name request failed");

    algokit_localnet_testing::validate_response("Box", &capture.last_body())
        .expect("application_box_by_name response does not match schema");
}
