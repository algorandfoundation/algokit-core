// Polytest Suite: POST v2_ledger_sync_ROUND
// Polytest Group: Common Tests
//
// Deferred: the sync-round endpoints are only served by a node in ledger follower mode, which
// localnet is not, so this returns 404.
#[tokio::test]
#[ignore = "deferred: sync-round endpoints require follower mode (404 on localnet)"]
async fn basic_request_and_response_validation() {}
