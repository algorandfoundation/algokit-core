// Polytest Suite: GET v2_ledger_supply
// Polytest Group: Common Tests
#[tokio::test]
async fn basic_request_and_response_validation() {
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    client.supply().await.expect("supply request failed");

    algokit_localnet_testing::validate_response("SupplyResponse", &capture.last_body())
        .expect("supply response does not match schema");
}
