//! Localnet integration scaffold for the kmd client.
//!
//! Hits a real kmd at `http://localhost:4002` (the algokit localnet default,
//! baked into `KmdClient::localnet()`). `#[ignore]`'d so the offline default test
//! run stays green; run against a live localnet with:
//!
//! ```sh
//! algokit localnet start
//! cargo test -p kmd_client --test integration_localnet -- --ignored
//! ```
//!
//! Minimal read-only smoke test; a fuller stateful suite (wallet/key lifecycle)
//! lands in a follow-up PR.

use kmd_client::KmdClient;

#[tokio::test]
#[ignore = "requires a running algokit localnet (kmd on :4002)"]
async fn version_lists_supported_versions() {
    let client = KmdClient::localnet();
    let versions = client.version().await.expect("kmd version request failed");
    assert!(
        !versions.versions.is_empty(),
        "expected at least one API version"
    );
}
