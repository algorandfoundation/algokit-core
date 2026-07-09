// Polytest Suite: GET v2_applications_APPLICATION-ID_boxes
// Polytest Group: Common Tests
#[tokio::test]
async fn basic_request_and_response_validation() {
    let manifest = algokit_localnet_testing::load_manifest();
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    let boxes = client
        .application_boxes(manifest.box_app_id, None)
        .await
        .expect("application_boxes request failed");
    assert!(!boxes.boxes.is_empty(), "seeded box app has no boxes");

    algokit_localnet_testing::validate_response("BoxesResponse", &capture.last_body())
        .expect("application_boxes response does not match schema");
}
