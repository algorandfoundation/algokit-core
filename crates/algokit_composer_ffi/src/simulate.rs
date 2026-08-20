//! FFI surface for composer simulate.
//!
//! Every export is sync and does no I/O. The calling language builds a request, sends it
//! with its own HTTP stack and async context, then maps the reply — matching the sync-only
//! convention the other `_ffi` crates follow for Swift 6.
//!
//! Transactions and the raw algod response cross as msgpack bytes. `SimulateResponse`
//! pulls in roughly forty generated structs that churn on every client regeneration;
//! mirroring them here would guarantee drift, and the curated projection below plus the
//! `simulate_response` blob is lossless.

use algokit_composer::{
    SimulateOptions as RustSimulateOptions, SimulateResult as RustSimulateResult,
    SimulateTraceConfig as RustSimulateTraceConfig, SimulateTxnResult as RustSimulateTxnResult,
    build_simulate_request, decode_simulate_response, empty_signature_envelopes,
    encode_simulate_request, map_simulate_response as rust_map_simulate_response,
};
use algokit_transact::{
    AlgorandMsgpack, SignedTransaction as RustSignedTransaction, Transaction as RustTransaction,
};
use ffi_macros::{ffi_func, ffi_record};
use serde::{Deserialize, Serialize};

use crate::AlgoKitComposerError;

// ------------------------------------------------------------------ records

#[ffi_record]
pub struct SimulateTraceConfig {
    pub enable: Option<bool>,
    pub stack_change: Option<bool>,
    pub scratch_change: Option<bool>,
    pub state_change: Option<bool>,
}

#[ffi_record]
pub struct SimulateOptions {
    /// `None` is treated as `false`. Optional only so the generated bindings get a
    /// default-constructible record.
    pub skip_signatures: Option<bool>,
    pub round: Option<u64>,
    pub allow_empty_signatures: Option<bool>,
    pub allow_more_logging: Option<bool>,
    pub allow_unnamed_resources: Option<bool>,
    pub extra_opcode_budget: Option<u64>,
    pub exec_trace_config: Option<SimulateTraceConfig>,
    pub fix_signers: Option<bool>,
}

#[ffi_record]
pub struct SimulateTxnResult {
    pub tx_id: String,
    /// MsgPack-encoded `PendingTransactionResponse`.
    pub confirmation: Vec<u8>,
    pub app_budget_consumed: Option<u32>,
    pub logic_sig_budget_consumed: Option<u32>,
    /// The address algod says should have signed, when `fix_signers` was set.
    pub fixed_signer: Option<String>,
    pub logs: Vec<Vec<u8>>,
}

#[ffi_record]
pub struct SimulateResult {
    /// MsgPack-encoded transactions, index-aligned with `txn_results`.
    pub transactions: Vec<Vec<u8>>,
    pub tx_ids: Vec<String>,
    pub group_id: Option<Vec<u8>>,
    pub txn_results: Vec<SimulateTxnResult>,
    pub failure_message: Option<String>,
    pub failed_at: Option<Vec<u64>>,
    pub app_budget_added: Option<u64>,
    pub app_budget_consumed: Option<u64>,
    pub last_round: u64,
    pub version: u64,
    /// The complete algod response as msgpack: execution traces, initial states, eval
    /// overrides, and anything this record does not flatten.
    pub simulate_response: Vec<u8>,
}

// ------------------------------------------------------------------ conversions

impl From<SimulateTraceConfig> for RustSimulateTraceConfig {
    fn from(config: SimulateTraceConfig) -> Self {
        Self {
            enable: config.enable,
            stack_change: config.stack_change,
            scratch_change: config.scratch_change,
            state_change: config.state_change,
        }
    }
}

impl From<SimulateOptions> for RustSimulateOptions {
    fn from(options: SimulateOptions) -> Self {
        Self {
            skip_signatures: options.skip_signatures.unwrap_or(false),
            round: options.round,
            allow_empty_signatures: options.allow_empty_signatures,
            allow_more_logging: options.allow_more_logging,
            allow_unnamed_resources: options.allow_unnamed_resources,
            extra_opcode_budget: options.extra_opcode_budget,
            exec_trace_config: options.exec_trace_config.map(Into::into),
            fix_signers: options.fix_signers,
        }
    }
}

impl TryFrom<RustSimulateTxnResult> for SimulateTxnResult {
    type Error = AlgoKitComposerError;

    fn try_from(result: RustSimulateTxnResult) -> Result<Self, Self::Error> {
        Ok(Self {
            confirmation: result.confirmation_msgpack()?,
            tx_id: result.tx_id,
            app_budget_consumed: result.app_budget_consumed,
            logic_sig_budget_consumed: result.logic_sig_budget_consumed,
            fixed_signer: result.fixed_signer.map(|address| address.to_string()),
            logs: result.logs,
        })
    }
}

impl TryFrom<RustSimulateResult> for SimulateResult {
    type Error = AlgoKitComposerError;

    fn try_from(result: RustSimulateResult) -> Result<Self, Self::Error> {
        let transactions = result
            .transactions
            .iter()
            .map(|txn| txn.encode())
            .collect::<Result<Vec<_>, _>>()?;

        let txn_results = result
            .txn_results
            .into_iter()
            .map(SimulateTxnResult::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            transactions,
            tx_ids: result.tx_ids,
            group_id: result.group_id.map(|id| id.to_vec()),
            txn_results,
            failure_message: result.failure_message,
            failed_at: result.failed_at,
            app_budget_added: result.app_budget_added,
            app_budget_consumed: result.app_budget_consumed,
            last_round: result.last_round,
            version: result.version,
            simulate_response: result.simulate_response.encode().map_err(|e| {
                AlgoKitComposerError::Msgpack {
                    error_msg: e.to_string(),
                }
            })?,
        })
    }
}

fn decode_transactions(bytes: Vec<Vec<u8>>) -> Result<Vec<RustTransaction>, AlgoKitComposerError> {
    bytes
        .iter()
        .map(|b| RustTransaction::decode(b))
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

fn decode_signed_transactions(
    bytes: Vec<Vec<u8>>,
) -> Result<Vec<RustSignedTransaction>, AlgoKitComposerError> {
    bytes
        .iter()
        .map(|b| RustSignedTransaction::decode(b))
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

// ------------------------------------------------------------------ sync exports

/// Wrap unsigned transactions in signature-less envelopes and encode the simulate request
/// body, ready to POST to `/v2/transactions/simulate?format=msgpack` with `Content-Type`
/// and `Accept` both `application/msgpack`.
#[ffi_func]
pub fn build_unsigned_simulate_request(
    transactions: Vec<Vec<u8>>,
    options: SimulateOptions,
) -> Result<Vec<u8>, AlgoKitComposerError> {
    let options = RustSimulateOptions {
        skip_signatures: true,
        ..options.into()
    };
    let envelopes = empty_signature_envelopes(decode_transactions(transactions)?);
    let request = build_simulate_request(envelopes, &options)?;
    Ok(encode_simulate_request(&request)?)
}

/// Encode a simulate request for an already-signed group. Signatures are preserved unless
/// `options.skip_signatures` is set.
#[ffi_func]
pub fn build_signed_simulate_request(
    signed_transactions: Vec<Vec<u8>>,
    options: SimulateOptions,
) -> Result<Vec<u8>, AlgoKitComposerError> {
    let request = build_simulate_request(
        decode_signed_transactions(signed_transactions)?,
        &options.into(),
    )?;
    Ok(encode_simulate_request(&request)?)
}

/// Map a raw msgpack simulate response onto a structured result.
///
/// `transactions` is the list the request was built from, passed explicitly rather than
/// recovered from the request body.
#[ffi_func]
pub fn map_simulate_response(
    transactions: Vec<Vec<u8>>,
    response: Vec<u8>,
) -> Result<SimulateResult, AlgoKitComposerError> {
    let response = decode_simulate_response(&response)?;
    rust_map_simulate_response(decode_transactions(transactions)?, response)?.try_into()
}

// The round trip is deliberately not exported. Every function here is sync and performs
// no I/O: the calling language holds its own async context and HTTP stack, calls
// build_*_simulate_request, sends the bytes, and passes the reply to
// map_simulate_response. Exporting an async fn would be the first in this workspace and
// breaks Swift 6 packaging; blocking on a network future instead would deadlock against a
// foreign-supplied client and stall the event loop on wasm32. Rust callers that want the
// round trip use algokit_composer::simulate directly.

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::TransactionMother;
    use pretty_assertions::assert_eq;

    fn payment_bytes() -> Vec<u8> {
        TransactionMother::simple_payment()
            .build()
            .unwrap()
            .encode()
            .unwrap()
    }

    fn options() -> SimulateOptions {
        SimulateOptions {
            skip_signatures: None,
            round: None,
            allow_empty_signatures: None,
            allow_more_logging: None,
            allow_unnamed_resources: None,
            extra_opcode_budget: None,
            exec_trace_config: None,
            fix_signers: None,
        }
    }

    #[test]
    fn absent_skip_signatures_is_false() {
        let rust: RustSimulateOptions = options().into();
        assert!(!rust.skip_signatures);
    }

    #[test]
    fn options_convert_field_for_field() {
        let ffi = SimulateOptions {
            skip_signatures: Some(true),
            round: Some(42),
            allow_more_logging: Some(true),
            extra_opcode_budget: Some(1000),
            exec_trace_config: Some(SimulateTraceConfig {
                enable: Some(true),
                stack_change: Some(false),
                scratch_change: None,
                state_change: Some(true),
            }),
            ..options()
        };

        let rust: RustSimulateOptions = ffi.into();

        assert!(rust.skip_signatures);
        assert_eq!(rust.round, Some(42));
        assert_eq!(rust.allow_more_logging, Some(true));
        assert_eq!(rust.extra_opcode_budget, Some(1000));

        let trace = rust.exec_trace_config.unwrap();
        assert_eq!(trace.enable, Some(true));
        assert_eq!(trace.stack_change, Some(false));
        assert_eq!(trace.scratch_change, None);
        assert_eq!(trace.state_change, Some(true));
    }

    #[test]
    fn build_unsigned_request_round_trips_through_bytes() {
        let encoded = build_unsigned_simulate_request(vec![payment_bytes()], options()).unwrap();

        let request: algokit_composer::SimulateRequest = rmp_serde::from_slice(&encoded).unwrap();
        assert_eq!(request.txn_groups.len(), 1);
        assert_eq!(request.allow_empty_signatures, Some(true));
        assert_eq!(request.fix_signers, Some(true));
        assert!(
            request.txn_groups[0]
                .txns
                .iter()
                .all(|t| t.signature.is_none())
        );
    }

    #[test]
    fn build_unsigned_request_rejects_undecodable_transactions() {
        let err = build_unsigned_simulate_request(vec![vec![0xff, 0xff]], options()).unwrap_err();
        assert!(matches!(err, AlgoKitComposerError::Transact { .. }));
    }

    #[test]
    fn map_response_projects_and_encodes_nested_types() {
        let txn = TransactionMother::simple_payment().build().unwrap();
        let confirmation = algokit_composer::PendingTransactionResponse::new(
            String::new(),
            algokit_composer::empty_signature_envelope(txn.clone()),
        );
        let response = algokit_composer::SimulateResponse::new(
            2,
            0,
            vec![algokit_composer::SimulateTransactionGroupResult::new(vec![
                algokit_composer::SimulateTransactionResult::new(confirmation),
            ])],
        );

        let result = map_simulate_response(
            vec![txn.encode().unwrap()],
            rmp_serde::to_vec_named(&response).unwrap(),
        )
        .unwrap();

        assert_eq!(result.txn_results.len(), 1);
        assert_eq!(result.tx_ids.len(), 1);
        assert!(
            !result.txn_results[0].confirmation.is_empty(),
            "the confirmation crosses as msgpack bytes"
        );
        assert!(
            !result.simulate_response.is_empty(),
            "the full response is carried as an escape hatch"
        );
        assert_eq!(result.failure_message, None);
    }
}
