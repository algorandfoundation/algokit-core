// Polytest Suite: GET v2_stateproofs_ROUND
// Polytest Group: Common Tests
//
// Deferred: localnet does not run the state-proof protocol, so no state proof exists for any round
// and this returns 404 ("no state proof can be found for that round").
#[tokio::test]
#[ignore = "deferred: localnet produces no state proofs (404)"]
async fn basic_request_and_response_validation() {}
