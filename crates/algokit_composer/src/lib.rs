pub mod composer;
pub mod error;
pub mod params;
pub mod signer;
pub mod simulate;

pub use composer::{ComposerParams, SuggestedParams, compose};
pub use error::ComposerError;
pub use params::{
    AssetOptInParams, AssetTransferParams, CommonTxnParams, OfflineKeyRegParams,
    OnlineKeyRegParams, PaymentParams, TxnParams,
};
pub use signer::sign_transactions;
pub use simulate::{
    SimulateOptions, SimulateResult, SimulateTraceConfig, SimulateTxnResult,
    build_simulate_request, decode_simulate_response, empty_signature_envelope,
    empty_signature_envelopes, encode_simulate_request, map_simulate_response, simulate,
    simulate_signed, simulate_unsigned,
};

// The transport and the algod wire types are part of the public simulate surface, so
// callers never need to depend on algod_client or algokit_http_client directly.
pub use algod_client::models::{
    PendingTransactionResponse, SimulateRequest, SimulateRequestTransactionGroup, SimulateResponse,
    SimulateTransactionGroupResult, SimulateTransactionResult,
};
pub use algokit_http_client::{HttpClient, HttpError, HttpMethod, HttpResponse};
