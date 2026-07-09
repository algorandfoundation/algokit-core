// Polytest Suite: POST v2_transactions_simulate
// Polytest Group: Common Tests
#[tokio::test]
async fn basic_request_and_response_validation() {
    use algod_client::models::{SimulateRequest, SimulateRequestTransactionGroup};
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

    let request = SimulateRequest {
        txn_groups: vec![SimulateRequestTransactionGroup { txns: vec![signed] }],
        ..Default::default()
    };

    // Decode-only: the generated client requests msgpack for this endpoint, so the raw body
    // cannot be validated against the JSON schema.
    fixture
        .algod
        .simulate_transactions(request)
        .await
        .expect("simulate_transactions request failed");
}
