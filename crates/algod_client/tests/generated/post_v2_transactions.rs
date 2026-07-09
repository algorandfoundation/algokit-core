// Polytest Suite: POST v2_transactions
// Polytest Group: Common Tests
#[tokio::test]
async fn basic_request_and_response_validation() {
    use algokit_transact::AlgorandMsgpack;

    use algokit_localnet_testing::fixtures::seeding;

    let fixture = algokit_localnet_testing::LocalnetFixture::new().await;

    let payment = seeding::payment(
        &fixture.algod,
        &fixture.dispenser.address,
        &fixture.dispenser.address,
        0,
    )
    .await;
    let signed = seeding::sign(&fixture.dispenser, payment).await;
    let bytes = signed
        .encode()
        .expect("failed to encode signed transaction");

    fixture
        .algod
        .raw_transaction(bytes)
        .await
        .expect("raw_transaction request failed");

    algokit_localnet_testing::validate_response(
        "PostTransactionsResponse",
        &fixture.capture.last_body(),
    )
    .expect("raw_transaction response does not match schema");
}
