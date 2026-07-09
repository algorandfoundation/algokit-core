// Polytest Suite: GET v2_deltas_ROUND
// Polytest Group: Common Tests
#[tokio::test]
async fn basic_request_and_response_validation() {
    use algokit_localnet_testing::fixtures::seeding;

    let fixture = algokit_localnet_testing::LocalnetFixture::new().await;

    // Submit a transaction to produce a recent round; deltas for older rounds are pruned.
    let txid = seeding::fund_account(
        &fixture.algod,
        &fixture.dispenser,
        &fixture.dispenser.address,
        0,
    )
    .await;
    let round = fixture
        .algod
        .pending_transaction_information(&txid)
        .await
        .expect("failed to fetch confirmed round")
        .confirmed_round
        .expect("transaction not confirmed");

    // Decode-only: the generated client requests msgpack for this endpoint, so the raw body
    // cannot be validated against the JSON schema.
    fixture
        .algod
        .ledger_state_delta(round)
        .await
        .expect("ledger_state_delta request failed");
}
