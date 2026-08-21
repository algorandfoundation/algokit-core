//! Simulate a transaction group against algod without submitting it.
//!
//! Simulate is a dry run: algod evaluates the group exactly as it would on-chain and
//! reports what *would* happen — success or failure, budgets consumed, and optionally a
//! full execution trace — without committing anything. It is the supported replacement
//! for the `dryrun` endpoint, which go-algorand removed in v5.0.0.
//!
//! The module is split so that only one step needs a network:
//!
//! - [`build_simulate_request`] / [`encode_simulate_request`] turn composed transactions
//!   into a request, offline.
//! - [`decode_simulate_response`] / [`map_simulate_response`] turn algod's reply into a
//!   usable result, offline.
//! - [`simulate`], [`simulate_unsigned`] and [`simulate_signed`] do the round trip, over
//!   a transport the caller supplies.
//!
//! Callers that already have their own HTTP stack can use the offline halves alone and
//! never touch this crate's networking.

use algod_client::models::{
    PendingTransactionResponse, SimulateRequest, SimulateRequestTransactionGroup, SimulateResponse,
    SimulateTraceConfig as AlgodSimulateTraceConfig, SimulateTransactionResult,
};
use algokit_http_client::HttpClient;
use algokit_transact::{Address, AlgorandMsgpack, SignedTransaction, Transaction, TransactionId};

use crate::composer::{ComposerParams, compose};
use crate::error::ComposerError;
use crate::params::TxnParams;

// ------------------------------------------------------------------ options

/// Execution-trace switches sent to algod as `exec-trace-config`.
///
/// The composer owns this type rather than re-exporting algod's so the *input* surface
/// is insulated from generated-client churn. Output types are algod's, verbatim.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SimulateTraceConfig {
    pub enable: Option<bool>,
    pub stack_change: Option<bool>,
    pub scratch_change: Option<bool>,
    pub state_change: Option<bool>,
}

impl SimulateTraceConfig {
    /// All four switches on — a full execution trace, for AVM debugging.
    pub fn full() -> Self {
        Self {
            enable: Some(true),
            stack_change: Some(true),
            scratch_change: Some(true),
            state_change: Some(true),
        }
    }
}

impl From<SimulateTraceConfig> for AlgodSimulateTraceConfig {
    fn from(config: SimulateTraceConfig) -> Self {
        Self {
            enable: config.enable,
            stack_change: config.stack_change,
            scratch_change: config.scratch_change,
            state_change: config.state_change,
        }
    }
}

/// Knobs for one simulate call.
///
/// Every field merges independently: setting one leaves the rest untouched.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SimulateOptions {
    /// Simulate without real signatures. Rebuilds every entry as a signature-less
    /// envelope and forces `allow_empty_signatures` and `fix_signers` on, overriding
    /// whatever those fields hold.
    pub skip_signatures: bool,
    /// Round to simulate against. `None` uses the latest committed round.
    pub round: Option<u64>,
    pub allow_empty_signatures: Option<bool>,
    pub allow_more_logging: Option<bool>,
    pub allow_unnamed_resources: Option<bool>,
    /// Extra opcode budget for the group. Not validated locally — algod is the authority.
    pub extra_opcode_budget: Option<u64>,
    pub exec_trace_config: Option<SimulateTraceConfig>,
    /// Ask algod to infer the authorizing signer for unsigned transactions and report it
    /// as [`SimulateTxnResult::fixed_signer`]. Only meaningful alongside
    /// `allow_empty_signatures`.
    pub fix_signers: Option<bool>,
}

impl SimulateOptions {
    /// `skip_signatures` on, nothing else set.
    pub fn skip_signatures() -> Self {
        Self {
            skip_signatures: true,
            ..Default::default()
        }
    }

    /// Skip signatures, verbose logging, and a full execution trace.
    pub fn debug_trace() -> Self {
        Self::skip_signatures().with_full_trace()
    }

    /// Turn on verbose logging and a full trace config, leaving every other field as-is.
    pub fn with_full_trace(mut self) -> Self {
        self.allow_more_logging = Some(true);
        self.exec_trace_config = Some(SimulateTraceConfig::full());
        self
    }
}

// ------------------------------------------------------------------ pure builders

/// Wrap a transaction in the envelope algod accepts under `allow-empty-signatures`.
///
/// The envelope carries no `sig` key at all: `signature` is `skip_serializing_if =
/// "Option::is_none"`, so `None` encodes to a one-key map `{"txn": ...}`.
///
/// Deliberately not `Some(EMPTY_SIGNATURE)` — despite the name, that constant is a test
/// fixture, and it would emit 64 zero bytes no other Algorand SDK sends.
pub fn empty_signature_envelope(transaction: Transaction) -> SignedTransaction {
    SignedTransaction {
        transaction,
        signature: None,
        auth_address: None,
        multisignature: None,
        logic_signature: None,
    }
}

/// [`empty_signature_envelope`] applied across a composed group.
pub fn empty_signature_envelopes(txns: Vec<Transaction>) -> Vec<SignedTransaction> {
    txns.into_iter().map(empty_signature_envelope).collect()
}

/// Build the simulate request body for one atomic group.
///
/// Always emits exactly one entry in `txn-groups`. algod accepts more, but
/// [`map_simulate_response`] relies on the one-to-one correspondence.
///
/// When `options.skip_signatures` is set every entry is rebuilt via
/// [`empty_signature_envelope`], discarding any signature the caller supplied, and
/// `allow_empty_signatures`/`fix_signers` are forced on.
///
/// # Errors
/// [`ComposerError::InvalidSimulateOptions`] when `txn_group` is empty.
pub fn build_simulate_request(
    txn_group: Vec<SignedTransaction>,
    options: &SimulateOptions,
) -> Result<SimulateRequest, ComposerError> {
    if txn_group.is_empty() {
        return Err(ComposerError::InvalidSimulateOptions {
            message: "a simulate request needs at least one transaction".to_string(),
        });
    }

    let (txns, allow_empty_signatures, fix_signers) = if options.skip_signatures {
        let stripped = txn_group
            .into_iter()
            .map(|signed| empty_signature_envelope(signed.transaction))
            .collect();
        (stripped, Some(true), Some(true))
    } else {
        (
            txn_group,
            options.allow_empty_signatures,
            options.fix_signers,
        )
    };

    Ok(SimulateRequest {
        txn_groups: vec![SimulateRequestTransactionGroup { txns }],
        round: options.round,
        allow_empty_signatures,
        allow_more_logging: options.allow_more_logging,
        allow_unnamed_resources: options.allow_unnamed_resources,
        extra_opcode_budget: options.extra_opcode_budget,
        exec_trace_config: options.exec_trace_config.clone().map(Into::into),
        fix_signers,
    })
}

/// Serialize a request exactly as `algod_client` puts it on the wire.
///
/// Uses `to_vec_named` rather than [`AlgorandMsgpack::encode`], which sorts map keys.
/// algod does not require sorted keys here, and matching the client keeps captured bytes
/// comparable.
pub fn encode_simulate_request(request: &SimulateRequest) -> Result<Vec<u8>, ComposerError> {
    rmp_serde::to_vec_named(request).map_err(|e| ComposerError::Msgpack {
        message: format!("failed to encode simulate request: {e}"),
    })
}

/// Decode a `?format=msgpack` simulate response body, matching the client's own decoder.
pub fn decode_simulate_response(bytes: &[u8]) -> Result<SimulateResponse, ComposerError> {
    rmp_serde::from_slice(bytes).map_err(|e| ComposerError::Msgpack {
        message: format!("failed to decode simulate response: {e}"),
    })
}

// ------------------------------------------------------------------ result

/// Per-transaction simulate outcome, projected from `txn-groups[0].txn-results[i]`.
#[derive(Debug, Clone, PartialEq)]
pub struct SimulateTxnResult {
    /// Base32 transaction id, computed locally — algod does not echo it back.
    pub tx_id: String,
    /// The `txn-result`, equivalent to a confirmation from the pending-transaction
    /// endpoint.
    pub confirmation: PendingTransactionResponse,
    pub app_budget_consumed: Option<u32>,
    pub logic_sig_budget_consumed: Option<u32>,
    /// The signer algod inferred when `fix_signers` was set and no signature was given.
    pub fixed_signer: Option<Address>,
    /// Application logs lifted out of the confirmation; empty when absent.
    pub logs: Vec<Vec<u8>>,
    /// The untouched per-transaction result: execution traces, unnamed resources, and
    /// anything this projection does not flatten.
    pub raw: SimulateTransactionResult,
}

impl SimulateTxnResult {
    /// MsgPack bytes of `confirmation`. Lives here so the FFI crate needs no
    /// `algod_client` dependency of its own.
    pub fn confirmation_msgpack(&self) -> Result<Vec<u8>, ComposerError> {
        self.confirmation
            .encode()
            .map_err(|e| ComposerError::Msgpack {
                message: format!("failed to encode confirmation: {e}"),
            })
    }
}

/// The composer's projection of a single-group simulate response.
///
/// A simulation that reports a failing group is still a *successful* simulate: the
/// failure lands in `failure_message`/`failed_at` alongside the budgets and traces that
/// explain it, rather than in an `Err`. Use [`SimulateResult::into_result`] to turn a
/// failing group into an `Err` instead.
#[derive(Debug, Clone, PartialEq)]
pub struct SimulateResult {
    /// The transactions exactly as simulated, in submission order.
    pub transactions: Vec<Transaction>,
    /// Base32 transaction ids, index-aligned with `transactions`.
    pub tx_ids: Vec<String>,
    /// Group id shared by `transactions`; `None` for a single ungrouped transaction.
    pub group_id: Option<[u8; 32]>,
    pub txn_results: Vec<SimulateTxnResult>,
    /// Set when group 0 failed. An empty string from algod is normalised to `None`.
    pub failure_message: Option<String>,
    /// Path to the failing transaction, zero-based, outer to inner.
    pub failed_at: Option<Vec<u64>>,
    pub app_budget_added: Option<u64>,
    pub app_budget_consumed: Option<u64>,
    pub last_round: u64,
    pub version: u64,
    /// The raw algod response, unmodified.
    pub simulate_response: SimulateResponse,
}

impl SimulateResult {
    /// Whether the group simulated without reporting a failure.
    pub fn is_success(&self) -> bool {
        self.failure_message.is_none()
    }

    /// `Err(ComposerError::SimulationFailed)` when group 0 failed. Renders "unknown"
    /// when `failed_at` is absent or empty.
    pub fn into_result(self) -> Result<Self, ComposerError> {
        match &self.failure_message {
            Some(failure_message) => {
                let failed_at = self
                    .failed_at
                    .as_ref()
                    .filter(|indexes| !indexes.is_empty())
                    .map(|indexes| {
                        indexes
                            .iter()
                            .map(|i| i.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                    .unwrap_or_else(|| "unknown".to_string());

                Err(ComposerError::SimulationFailed {
                    failed_at,
                    failure_message: failure_message.clone(),
                    response: Box::new(self.simulate_response),
                })
            }
            None => Ok(self),
        }
    }

    /// MsgPack bytes of `simulate_response`. Lives here so the FFI crate needs no
    /// `algod_client` dependency of its own.
    pub fn response_msgpack(&self) -> Result<Vec<u8>, ComposerError> {
        self.simulate_response
            .encode()
            .map_err(|e| ComposerError::Msgpack {
                message: format!("failed to encode simulate response: {e}"),
            })
    }
}

/// Project a raw response onto [`SimulateResult`], paired with the transactions that
/// produced it.
///
/// Never reads the nested `txn_result.txn`, so the projection does not depend on how a
/// `SignedTransaction` inside a response deserializes.
///
/// # Errors
/// [`ComposerError::SimulateResponseShape`] when the response does not carry exactly one
/// group, or when its result count does not match `transactions`.
pub fn map_simulate_response(
    transactions: Vec<Transaction>,
    response: SimulateResponse,
) -> Result<SimulateResult, ComposerError> {
    if response.txn_groups.len() != 1 {
        return Err(ComposerError::SimulateResponseShape {
            message: format!(
                "expected exactly one transaction group, got {}",
                response.txn_groups.len()
            ),
        });
    }

    let group = &response.txn_groups[0];
    if group.txn_results.len() != transactions.len() {
        return Err(ComposerError::SimulateResponseShape {
            message: format!(
                "response has {} transaction results but {} transactions were simulated",
                group.txn_results.len(),
                transactions.len()
            ),
        });
    }

    let tx_ids = transactions
        .iter()
        .map(|txn| txn.id())
        .collect::<Result<Vec<_>, _>>()?;

    let txn_results = group
        .txn_results
        .iter()
        .zip(tx_ids.iter())
        .map(|(result, tx_id)| SimulateTxnResult {
            tx_id: tx_id.clone(),
            confirmation: result.txn_result.clone(),
            app_budget_consumed: result.app_budget_consumed,
            logic_sig_budget_consumed: result.logic_sig_budget_consumed,
            fixed_signer: result.fixed_signer.clone(),
            logs: result.txn_result.logs.clone().unwrap_or_default(),
            raw: result.clone(),
        })
        .collect();

    // algod sends an empty string for a group that did not fail; normalise it away
    // rather than reporting a failure with no message.
    let failure_message = group
        .failure_message
        .clone()
        .filter(|message| !message.is_empty());

    let group_id = transactions.first().and_then(|txn| txn.header().group);

    Ok(SimulateResult {
        transactions,
        tx_ids,
        group_id,
        txn_results,
        failure_message,
        failed_at: group.failed_at.clone(),
        app_budget_added: group.app_budget_added,
        app_budget_consumed: group.app_budget_consumed,
        last_round: response.last_round,
        version: response.version,
        simulate_response: response,
    })
}

// ------------------------------------------------------------------ async end-to-end

/// Compose a group and simulate it in one call.
///
/// The returned [`SimulateResult::transactions`] are the composed, grouped transactions,
/// so a caller can sign and submit them afterwards without recomposing.
pub async fn simulate(
    http_client: &dyn HttpClient,
    txn_params: Vec<TxnParams>,
    composer_params: ComposerParams,
    options: SimulateOptions,
) -> Result<SimulateResult, ComposerError> {
    let txns = compose(txn_params, composer_params)?;
    simulate_unsigned(http_client, txns, options).await
}

/// Simulate an already-composed but unsigned group.
///
/// Always wraps in signature-less envelopes and forces `allow_empty_signatures` and
/// `fix_signers`, regardless of `options.skip_signatures`.
pub async fn simulate_unsigned(
    http_client: &dyn HttpClient,
    txns: Vec<Transaction>,
    options: SimulateOptions,
) -> Result<SimulateResult, ComposerError> {
    let options = SimulateOptions {
        skip_signatures: true,
        ..options
    };
    simulate_signed(http_client, empty_signature_envelopes(txns), options).await
}

/// Simulate a signed group with its real signatures.
///
/// Signatures are preserved unless `options.skip_signatures` is set. For a partially
/// signed group, set `options.allow_empty_signatures` yourself.
pub async fn simulate_signed(
    http_client: &dyn HttpClient,
    signed_txns: Vec<SignedTransaction>,
    options: SimulateOptions,
) -> Result<SimulateResult, ComposerError> {
    let txns: Vec<Transaction> = signed_txns
        .iter()
        .map(|signed| signed.transaction.clone())
        .collect();
    let request = build_simulate_request(signed_txns, &options)?;
    let response =
        algod_client::apis::simulate_transactions::simulate_transactions(http_client, request)
            .await?;
    map_simulate_response(txns, response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use algod_client::models::{SimulateTransactionGroupResult, SimulateTransactionResult};
    use algokit_transact::test_utils::TransactionMother;
    use pretty_assertions::assert_eq;

    fn payment() -> Transaction {
        TransactionMother::simple_payment().build().unwrap()
    }

    /// Decode an encoded envelope to its msgpack map keys, so tests assert on the wire
    /// shape rather than on Rust-side field values.
    fn envelope_keys(signed: &SignedTransaction) -> Vec<String> {
        let bytes = rmp_serde::to_vec_named(signed).unwrap();
        let value: rmpv::Value = rmp_serde::from_slice(&bytes).unwrap();
        match value {
            rmpv::Value::Map(entries) => entries
                .iter()
                .map(|(k, _)| k.as_str().unwrap().to_string())
                .collect(),
            other => panic!("expected a msgpack map, got {other:?}"),
        }
    }

    fn group_result(txn_results: Vec<SimulateTransactionResult>) -> SimulateTransactionGroupResult {
        SimulateTransactionGroupResult::new(txn_results)
    }

    fn txn_result_for(txn: &Transaction) -> SimulateTransactionResult {
        SimulateTransactionResult::new(PendingTransactionResponse::new(
            String::new(),
            empty_signature_envelope(txn.clone()),
        ))
    }

    fn response_for(txns: &[Transaction]) -> SimulateResponse {
        let results = txns.iter().map(txn_result_for).collect();
        // new(version, last_round, txn_groups)
        SimulateResponse::new(2, 0, vec![group_result(results)])
    }

    #[test]
    fn empty_signature_envelope_encodes_txn_key_only() {
        let envelope = empty_signature_envelope(payment());
        assert_eq!(envelope_keys(&envelope), vec!["txn".to_string()]);
    }

    #[test]
    fn empty_signature_envelope_matches_a_zero_signature() {
        let txn = payment();
        let empty = empty_signature_envelope(txn.clone());
        let zeroed = SignedTransaction {
            transaction: txn,
            signature: Some([0u8; 64]),
            auth_address: None,
            multisignature: None,
            logic_signature: None,
        };

        // An all-zero signature is omitted, so both produce the same envelope.
        assert_eq!(envelope_keys(&zeroed), vec!["txn".to_string()]);
        assert_eq!(
            rmp_serde::to_vec_named(&empty).unwrap(),
            rmp_serde::to_vec_named(&zeroed).unwrap(),
        );
    }

    #[test]
    fn skip_signatures_forces_allow_empty_signatures_and_fix_signers() {
        let options = SimulateOptions {
            skip_signatures: true,
            allow_empty_signatures: Some(false),
            fix_signers: Some(false),
            ..Default::default()
        };

        let request =
            build_simulate_request(vec![empty_signature_envelope(payment())], &options).unwrap();

        assert_eq!(request.allow_empty_signatures, Some(true));
        assert_eq!(request.fix_signers, Some(true));
    }

    #[test]
    fn skip_signatures_rebuilds_envelopes_without_signatures() {
        let signed = SignedTransaction {
            transaction: payment(),
            signature: Some([7u8; 64]),
            auth_address: None,
            multisignature: None,
            logic_signature: None,
        };

        let request =
            build_simulate_request(vec![signed], &SimulateOptions::skip_signatures()).unwrap();

        assert!(
            request.txn_groups[0]
                .txns
                .iter()
                .all(|t| t.signature.is_none())
        );
    }

    #[test]
    fn signed_path_preserves_signatures() {
        let signed = SignedTransaction {
            transaction: payment(),
            signature: Some([7u8; 64]),
            auth_address: None,
            multisignature: None,
            logic_signature: None,
        };

        let request = build_simulate_request(vec![signed], &SimulateOptions::default()).unwrap();

        assert_eq!(request.txn_groups[0].txns[0].signature, Some([7u8; 64]));
        assert_eq!(request.allow_empty_signatures, None);
        assert_eq!(request.fix_signers, None);
    }

    #[test]
    fn options_merge_per_field() {
        let options = SimulateOptions {
            extra_opcode_budget: Some(1000),
            ..Default::default()
        };

        let request =
            build_simulate_request(vec![empty_signature_envelope(payment())], &options).unwrap();

        assert_eq!(request.extra_opcode_budget, Some(1000));
        assert_eq!(request.round, None);
        assert_eq!(request.allow_more_logging, None);
        assert_eq!(request.allow_unnamed_resources, None);
        assert_eq!(request.exec_trace_config, None);
    }

    #[test]
    fn debug_trace_preset_enables_logging_and_full_trace() {
        let options = SimulateOptions::debug_trace();
        assert!(options.skip_signatures);
        assert_eq!(options.allow_more_logging, Some(true));

        let trace = options.exec_trace_config.clone().unwrap();
        assert_eq!(trace, SimulateTraceConfig::full());
        assert_eq!(
            (
                trace.enable,
                trace.stack_change,
                trace.scratch_change,
                trace.state_change
            ),
            (Some(true), Some(true), Some(true), Some(true))
        );
    }

    #[test]
    fn build_simulate_request_rejects_empty_group() {
        let err = build_simulate_request(vec![], &SimulateOptions::default()).unwrap_err();
        assert!(matches!(err, ComposerError::InvalidSimulateOptions { .. }));
    }

    #[test]
    fn build_simulate_request_always_emits_exactly_one_group() {
        let txns = vec![
            empty_signature_envelope(payment()),
            empty_signature_envelope(payment()),
        ];
        let request = build_simulate_request(txns, &SimulateOptions::default()).unwrap();

        assert_eq!(request.txn_groups.len(), 1);
        assert_eq!(request.txn_groups[0].txns.len(), 2);
    }

    #[test]
    fn map_simulate_response_projects_ids_and_confirmations() {
        let txns = vec![payment()];
        let result = map_simulate_response(txns.clone(), response_for(&txns)).unwrap();

        assert_eq!(result.txn_results.len(), 1);
        assert_eq!(result.tx_ids, vec![txns[0].id().unwrap()]);
        assert_eq!(result.txn_results[0].tx_id, txns[0].id().unwrap());
        assert!(result.is_success());
    }

    #[test]
    fn map_simulate_response_reports_failure_as_data() {
        let txns = vec![payment()];
        let mut response = response_for(&txns);
        response.txn_groups[0].failure_message = Some("assert failed".to_string());
        response.txn_groups[0].failed_at = Some(vec![0]);

        let result = map_simulate_response(txns, response).unwrap();

        assert!(!result.is_success());
        assert_eq!(result.failure_message.as_deref(), Some("assert failed"));
        assert_eq!(result.failed_at, Some(vec![0]));
        assert_eq!(
            result.txn_results.len(),
            1,
            "per-transaction results survive a failing group"
        );
    }

    #[test]
    fn map_simulate_response_treats_empty_failure_message_as_success() {
        let txns = vec![payment()];
        let mut response = response_for(&txns);
        response.txn_groups[0].failure_message = Some(String::new());

        let result = map_simulate_response(txns, response).unwrap();

        assert_eq!(result.failure_message, None);
        assert!(result.is_success());
    }

    #[test]
    fn into_result_renders_failure_message() {
        let txns = vec![payment()];
        let mut response = response_for(&txns);
        response.txn_groups[0].failure_message = Some("assert failed".to_string());
        response.txn_groups[0].failed_at = Some(vec![0, 1]);

        let err = map_simulate_response(txns, response)
            .unwrap()
            .into_result()
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            "Transaction failed at transaction(s) 0, 1 in the group. assert failed"
        );
    }

    #[test]
    fn into_result_renders_unknown_for_absent_and_empty_failed_at() {
        for failed_at in [None, Some(vec![])] {
            let txns = vec![payment()];
            let mut response = response_for(&txns);
            response.txn_groups[0].failure_message = Some("boom".to_string());
            response.txn_groups[0].failed_at = failed_at.clone();

            let err = map_simulate_response(txns, response)
                .unwrap()
                .into_result()
                .unwrap_err();

            assert_eq!(
                err.to_string(),
                "Transaction failed at transaction(s) unknown in the group. boom",
                "failed_at {failed_at:?} should render as unknown"
            );
        }
    }

    #[test]
    fn into_result_returns_ok_for_a_successful_group() {
        let txns = vec![payment()];
        let result = map_simulate_response(txns.clone(), response_for(&txns)).unwrap();
        assert!(result.into_result().is_ok());
    }

    #[test]
    fn map_simulate_response_rejects_group_count_mismatch() {
        let txns = vec![payment()];
        let mut response = response_for(&txns);
        response.txn_groups.push(group_result(vec![]));

        let err = map_simulate_response(txns, response).unwrap_err();
        assert!(matches!(err, ComposerError::SimulateResponseShape { .. }));
    }

    #[test]
    fn map_simulate_response_rejects_txn_result_count_mismatch() {
        let txns = vec![payment(), payment()];
        let response = response_for(&txns[..1]);

        let err = map_simulate_response(txns, response).unwrap_err();
        assert!(matches!(err, ComposerError::SimulateResponseShape { .. }));
    }

    #[test]
    fn response_msgpack_round_trips() {
        let txns = vec![payment()];
        let result = map_simulate_response(txns.clone(), response_for(&txns)).unwrap();

        let bytes = result.response_msgpack().unwrap();
        let decoded = SimulateResponse::decode(&bytes).unwrap();

        assert_eq!(decoded, result.simulate_response);
    }

    /// Records the request it was handed and replays a canned response, so the async
    /// path is exercised end to end without a node.
    struct StubHttpClient {
        response: SimulateResponse,
        captured: std::sync::Mutex<Option<CapturedRequest>>,
    }

    #[derive(Debug, Clone)]
    struct CapturedRequest {
        method: algokit_http_client::HttpMethod,
        path: String,
        query: Option<std::collections::HashMap<String, String>>,
        headers: Option<std::collections::HashMap<String, String>>,
        body: Option<Vec<u8>>,
    }

    #[async_trait::async_trait]
    impl HttpClient for StubHttpClient {
        async fn request(
            &self,
            http_method: algokit_http_client::HttpMethod,
            path: String,
            query: Option<std::collections::HashMap<String, String>>,
            body: Option<Vec<u8>>,
            headers: Option<std::collections::HashMap<String, String>>,
        ) -> Result<algokit_http_client::HttpResponse, algokit_http_client::HttpError> {
            *self.captured.lock().unwrap() = Some(CapturedRequest {
                method: http_method,
                path,
                query,
                headers,
                body,
            });

            Ok(algokit_http_client::HttpResponse {
                body: rmp_serde::to_vec_named(&self.response).unwrap(),
                headers: std::collections::HashMap::from([(
                    "content-type".to_string(),
                    "application/msgpack".to_string(),
                )]),
            })
        }
    }

    #[tokio::test]
    async fn simulate_unsigned_round_trips_through_a_stub_transport() {
        let txns = vec![payment()];
        let client = StubHttpClient {
            response: response_for(&txns),
            captured: std::sync::Mutex::new(None),
        };

        let result = simulate_unsigned(&client, txns.clone(), SimulateOptions::default())
            .await
            .unwrap();

        // The result is projected from the canned response.
        assert!(result.is_success());
        assert_eq!(result.tx_ids, vec![txns[0].id().unwrap()]);

        // And the request went out on the contract algod expects.
        let captured = client.captured.lock().unwrap().clone().unwrap();
        assert!(matches!(
            captured.method,
            algokit_http_client::HttpMethod::Post
        ));
        assert_eq!(captured.path, "/v2/transactions/simulate");
        assert_eq!(
            captured.query.unwrap().get("format").map(String::as_str),
            Some("msgpack")
        );

        let headers = captured.headers.unwrap();
        assert_eq!(
            headers.get("Content-Type").map(String::as_str),
            Some("application/msgpack")
        );
        assert_eq!(
            headers.get("Accept").map(String::as_str),
            Some("application/msgpack")
        );

        // simulate_unsigned always sends signature-less envelopes with both flags on.
        let sent: SimulateRequest = rmp_serde::from_slice(&captured.body.unwrap()).unwrap();
        assert_eq!(sent.allow_empty_signatures, Some(true));
        assert_eq!(sent.fix_signers, Some(true));
        assert_eq!(sent.txn_groups.len(), 1);
        assert!(
            sent.txn_groups[0]
                .txns
                .iter()
                .all(|t| t.signature.is_none())
        );
    }

    #[tokio::test]
    async fn simulate_signed_keeps_signatures_on_the_wire() {
        let txn = payment();
        let signed = SignedTransaction {
            transaction: txn.clone(),
            signature: Some([9u8; 64]),
            auth_address: None,
            multisignature: None,
            logic_signature: None,
        };
        let client = StubHttpClient {
            response: response_for(&[txn]),
            captured: std::sync::Mutex::new(None),
        };

        simulate_signed(&client, vec![signed], SimulateOptions::default())
            .await
            .unwrap();

        let captured = client.captured.lock().unwrap().clone().unwrap();
        let sent: SimulateRequest = rmp_serde::from_slice(&captured.body.unwrap()).unwrap();

        assert_eq!(sent.txn_groups[0].txns[0].signature, Some([9u8; 64]));
        assert_eq!(sent.allow_empty_signatures, None);
    }
}
