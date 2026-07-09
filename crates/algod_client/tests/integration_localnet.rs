//! Localnet integration scaffold for the algod client.
//!
//! These tests hit a real algod node at `http://localhost:4001` (the algokit
//! localnet default, baked into `AlgodClient::localnet()`). They are `#[ignore]`'d
//! so the default `cargo t` / `sanity.sh` (which run offline) stay green; run them
//! explicitly against a running localnet with:
//!
//! ```sh
//! algokit localnet start
//! cargo test -p algod_client --test integration_localnet -- --ignored
//! ```
//!
//! This is intentionally a minimal smoke test; a fuller integration suite lands
//! in a follow-up PR.

use algod_client::AlgodClient;

#[tokio::test]
#[ignore = "requires a running algokit localnet (algod on :4001)"]
async fn status_returns_a_round() {
    let client = AlgodClient::localnet();
    let status = client.status().await.expect("algod status request failed");
    // A live node always reports a non-decreasing round; just assert we got one.
    assert!(status.last_round >= 1, "expected a positive last_round");
}
