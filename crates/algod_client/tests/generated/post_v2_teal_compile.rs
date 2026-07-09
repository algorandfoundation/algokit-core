// Polytest Suite: POST v2_teal_compile
// Polytest Group: Common Tests
#[tokio::test]
async fn basic_request_and_response_validation() {
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    let program = b"#pragma version 8\nint 1\nreturn\n".to_vec();
    client
        .teal_compile(program, None)
        .await
        .expect("teal_compile request failed");

    algokit_localnet_testing::validate_response("CompileResponse", &capture.last_body())
        .expect("teal_compile response does not match schema");
}
