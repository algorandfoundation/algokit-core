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
//! `algokit localnet` tracks the floating `algorand/algod:latest` tag, so the node
//! version is whatever was last pulled. Run `algokit localnet reset --update` to
//! move a stale localnet onto the current image.
//!
//! This is intentionally a minimal smoke test; a fuller integration suite lands
//! in a follow-up PR.

use algod_client::AlgodClient;

/// Major version of algod these tests are written against (go-algorand 5.x,
/// consensus v42). A localnet on an older image silently changes wire formats,
/// fee rules and the available endpoint set, so tests assert the version up
/// front rather than failing further downstream in ways that look like bugs.
const EXPECTED_ALGOD_MAJOR: u64 = 5;

/// Fails with an actionable message when localnet is not on the expected major
/// version, instead of letting the mismatch surface as a confusing assertion
/// failure in an unrelated test.
async fn assert_supported_node_version(client: &AlgodClient) {
    let version = client
        .version()
        .await
        .expect("algod versions request failed");
    let build = &version.build;

    assert_eq!(
        build.major, EXPECTED_ALGOD_MAJOR,
        "localnet is running algod {}.{}.{} but these tests target algod {}.x; \
         run `algokit localnet reset --update` to pull the current image",
        build.major, build.minor, build.build_number, EXPECTED_ALGOD_MAJOR
    );
}

#[tokio::test]
#[ignore = "requires a running algokit localnet (algod on :4001)"]
async fn node_reports_expected_version() {
    let client = AlgodClient::localnet();
    assert_supported_node_version(&client).await;
}

#[tokio::test]
#[ignore = "requires a running algokit localnet (algod on :4001)"]
async fn status_reports_a_consensus_version() {
    let client = AlgodClient::localnet();
    assert_supported_node_version(&client).await;

    let status = client.status().await.expect("algod status request failed");

    // A freshly reset localnet sits at round 0 until a transaction is submitted,
    // so the round number carries no signal here. The consensus version does: a
    // live node always reports the protocol it is running.
    assert!(
        !status.last_version.is_empty(),
        "expected a consensus version in the node status"
    );
}
