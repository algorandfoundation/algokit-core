//! Localnet integration tests for composer simulate.
//!
//! These hit a real algod node at `http://localhost:4001` (the algokit localnet
//! default). They are `#[ignore]`'d so the default `cargo t` / `sanity.sh` stay offline
//! and green; run them explicitly against a running localnet with:
//!
//! ```sh
//! algokit localnet start
//! cargo test -p algokit_composer --test integration_localnet -- --ignored
//! ```
//!
//! `algokit localnet` tracks the floating `algorand/algod:latest` tag, so run
//! `algokit localnet reset --update` to move a stale localnet onto the current image.

use algokit_composer::{
    CommonTxnParams, ComposerParams, PaymentParams, SimulateOptions, SuggestedParams, TxnParams,
    simulate,
};
use algokit_http_client::DefaultHttpClient;

/// Major version of algod these tests target (go-algorand 5.x, consensus v42).
const EXPECTED_ALGOD_MAJOR: u64 = 5;

/// The well-known localnet dev account, funded in genesis.
const LOCALNET_TOKEN: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn client() -> DefaultHttpClient {
    DefaultHttpClient::with_header("http://localhost:4001", "X-Algo-API-Token", LOCALNET_TOKEN)
        .expect("failed to build localnet http client")
}

fn algod() -> algod_client::AlgodClient {
    algod_client::AlgodClient::localnet()
}

/// Fails with an actionable message when localnet is not on the expected major version,
/// rather than letting the mismatch surface as a confusing failure downstream.
async fn assert_supported_node_version() {
    let version = algod()
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

/// Suggested params pulled from the live node, plus a funded sender to simulate from.
///
/// The genesis wallets are regenerated on every `algokit localnet reset`, so the funded
/// account is discovered rather than hardcoded.
async fn live_params() -> (SuggestedParams, String) {
    let params = algod()
        .transaction_params()
        .await
        .expect("failed to fetch transaction params");

    let genesis = algod().genesis().await.expect("failed to fetch genesis");
    let sender = genesis
        .alloc
        .iter()
        .find(|a| {
            // `state` is untyped JSON in the generated model; the balance lives at
            // `state.algo` and the fee sink / rewards pool hold only dust.
            serde_json::to_value(&a.state)
                .ok()
                .and_then(|v| v.get("algo").and_then(serde_json::Value::as_u64))
                .is_some_and(|algo| algo > 1_000_000)
        })
        .map(|a| a.addr.clone())
        .expect("no funded account in localnet genesis");

    (
        SuggestedParams {
            fee: params.fee,
            flat_fee: false,
            first_round_valid: params.last_round,
            last_round_valid: params.last_round + 1000,
            genesis_hash: params.genesis_hash.to_vec(),
            genesis_id: params.genesis_id,
        },
        sender,
    )
}

fn payment(sender: &str, receiver: &str, amount: u64) -> TxnParams {
    TxnParams::Payment(PaymentParams {
        common: CommonTxnParams {
            sender: sender.to_string(),
            note: None,
            lease: None,
            rekey_to: None,
            // An explicit fee is required here because the composer derives its fee from
            // `SuggestedParams::fee`, which an uncongested node reports as 0, and it does
            // not yet consult `min_fee`. Under consensus v42 a zero-fee group is rejected
            // outright: "txgroup with 0.0A fees is less than 1mA (usage=1.000000 *
            // base=1mA)". Wiring the composer to the v42 usage-based fee model is its own
            // piece of work; this keeps the simulate tests honest until then.
            static_fee: Some(1_000),
            extra_fee: None,
            max_fee: None,
            validity_window: None,
            first_valid_round: None,
            last_valid_round: None,
        },
        receiver: receiver.to_string(),
        amount,
        close_remainder_to: None,
    })
}

#[tokio::test]
#[ignore = "requires a running algokit localnet (algod on :4001)"]
async fn unsigned_payment_group_simulates() {
    assert_supported_node_version().await;
    let (suggested_params, sender) = live_params().await;

    let result = simulate(
        &client(),
        vec![payment(&sender, &sender, 1000)],
        ComposerParams {
            suggested_params,
            default_validity_window: None,
        },
        SimulateOptions::skip_signatures(),
    )
    .await
    .expect("simulate request failed");

    assert!(
        result.is_success(),
        "expected a successful simulation, got {:?}",
        result.failure_message
    );
    assert_eq!(result.tx_ids.len(), 1);
    assert_eq!(result.txn_results.len(), 1);
}

#[tokio::test]
#[ignore = "requires a running algokit localnet (algod on :4001)"]
async fn failing_group_returns_failure_as_data() {
    assert_supported_node_version().await;
    let (suggested_params, sender) = live_params().await;

    // Overspend: more microAlgo than any localnet account holds.
    let result = simulate(
        &client(),
        vec![payment(&sender, &sender, u64::MAX / 2)],
        ComposerParams {
            suggested_params,
            default_validity_window: None,
        },
        SimulateOptions::skip_signatures(),
    )
    .await
    .expect("simulate request itself should succeed even when the group fails");

    assert!(!result.is_success());
    assert!(result.failure_message.is_some());
    assert_eq!(result.failed_at, Some(vec![0]));
    assert_eq!(
        result.txn_results.len(),
        1,
        "per-transaction results survive a failing group"
    );

    // Opting in turns the same data into an error.
    assert!(result.into_result().is_err());
}

#[tokio::test]
#[ignore = "requires a running algokit localnet (algod on :4001)"]
async fn debug_trace_preset_returns_exec_trace_config() {
    assert_supported_node_version().await;
    let (suggested_params, sender) = live_params().await;

    let result = simulate(
        &client(),
        vec![payment(&sender, &sender, 1000)],
        ComposerParams {
            suggested_params,
            default_validity_window: None,
        },
        SimulateOptions::debug_trace(),
    )
    .await
    .expect("simulate request failed");

    assert!(
        result.simulate_response.exec_trace_config.is_some(),
        "the node should echo back the trace configuration it honoured"
    );
}

#[tokio::test]
#[ignore = "requires a running algokit localnet (algod on :4001)"]
async fn nested_confirmation_decodes_correct_transaction_variant() {
    assert_supported_node_version().await;
    let (suggested_params, sender) = live_params().await;

    let result = simulate(
        &client(),
        vec![payment(&sender, &sender, 1000)],
        ComposerParams {
            suggested_params,
            default_validity_window: None,
        },
        SimulateOptions::skip_signatures(),
    )
    .await
    .expect("simulate request failed");

    // Pins that a SignedTransaction nested inside a response still deserializes to the
    // right variant, so a future generated-client change cannot silently regress it.
    assert!(matches!(
        result.txn_results[0].confirmation.txn.transaction,
        algokit_transact::Transaction::Payment(_)
    ));
}
