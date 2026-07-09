// Polytest Suite: GET v2_deltas_ROUND_txn_group
// Polytest Group: Common Tests
//
// Deferred: localnet returns 501 Not Implemented ("failed retrieving the expected tracer from
// ledger") for this endpoint, which needs the transaction tracer that localnet does not run.
#[tokio::test]
#[ignore = "deferred: localnet does not run the transaction tracer (501)"]
async fn basic_request_and_response_validation() {}
