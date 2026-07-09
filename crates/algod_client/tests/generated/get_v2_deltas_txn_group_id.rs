// Polytest Suite: GET v2_deltas_txn_group_ID
// Polytest Group: Common Tests
//
// Deferred: obtaining a group id requires the txn-group deltas endpoint, which localnet answers with
// 501 Not Implemented ("failed retrieving the expected tracer from ledger").
#[tokio::test]
#[ignore = "deferred: localnet does not run the transaction tracer (501)"]
async fn basic_request_and_response_validation() {}
