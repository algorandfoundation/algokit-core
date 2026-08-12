//! Localnet integration scaffold for the indexer client.
//!
//! Hits a real indexer at `http://localhost:8980` (the algokit localnet default,
//! baked into `IndexerClient::localnet()`). `#[ignore]`'d so the offline default
//! test run stays green; run against a live localnet with:
//!
//! ```sh
//! algokit localnet start
//! cargo test -p indexer_client --test integration_localnet -- --ignored
//! ```
//!
//! Minimal smoke test; fuller integration suite lands in a follow-up PR.

use indexer_client::IndexerClient;

#[tokio::test]
#[ignore = "requires a running algokit localnet (indexer on :8980)"]
async fn health_check_reports_a_version() {
    let client = IndexerClient::localnet();
    let health = client
        .health_check()
        .await
        .expect("indexer health request failed");
    assert!(!health.version.is_empty(), "expected a non-empty version");
}
