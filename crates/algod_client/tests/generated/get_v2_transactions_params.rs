// Polytest Suite: GET v2_transactions_params
// Polytest Group: Common Tests
#[tokio::test]
async fn basic_request_and_response_validation() {
    let capture = algokit_localnet_testing::CapturingHttpClient::localnet();
    let client = capture.client();

    client
        .transaction_params()
        .await
        .expect("transaction params request failed");

    algokit_localnet_testing::validate_response(
        "TransactionParametersResponse",
        &capture.last_body(),
    )
    .expect("transaction params response does not match schema");
}
