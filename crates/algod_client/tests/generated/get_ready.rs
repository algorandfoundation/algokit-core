// Polytest Suite: GET ready
// Polytest Group: Common Tests
#[tokio::test]
async fn basic_request_and_response_validation() {
    use algokit_localnet_testing::fixtures::seeding;

    let fixture = algokit_localnet_testing::LocalnetFixture::new().await;

    // Produce a block: in devmode /ready returns 503 until one is committed.
    seeding::fund_account(
        &fixture.algod,
        &fixture.dispenser,
        &fixture.dispenser.address,
        0,
    )
    .await;

    fixture.algod.ready().await.expect("ready request failed");
}
