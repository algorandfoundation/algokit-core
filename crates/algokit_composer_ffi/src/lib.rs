use algokit_composer::{
    AssetOptInParams as RustAssetOptInParams, AssetTransferParams as RustAssetTransferParams,
    CommonTxnParams as RustCommonTxnParams, ComposerError as RustComposerError,
    ComposerParams as RustComposerParams, OfflineKeyRegParams as RustOfflineKeyRegParams,
    OnlineKeyRegParams as RustOnlineKeyRegParams, PaymentParams as RustPaymentParams,
    SuggestedParams as RustSuggestedParams, TxnParams as RustTxnParams,
};
use algokit_transact::{
    AlgorandMsgpack, SignedTransaction as RustSignedTransaction, Transaction as RustTransaction,
};
use ffi_macros::{ffi_enum, ffi_func, ffi_record};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

#[cfg(feature = "ffi_uniffi")]
uniffi::setup_scaffolding!();

/// FFI-compatible error type for composer operations.
#[derive(Debug, Clone, Snafu)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
pub enum AlgoKitComposerError {
    #[snafu(display("{error_msg}"))]
    InvalidAddress { error_msg: String },

    #[snafu(display("{error_msg}"))]
    InvalidByteLength { error_msg: String },

    #[snafu(display("{error_msg}"))]
    InvalidTxnParams { error_msg: String },

    #[snafu(display("{error_msg}"))]
    SignerCountMismatch { error_msg: String },

    #[snafu(display("{error_msg}"))]
    Signing { error_msg: String },

    #[snafu(display("{error_msg}"))]
    Transact { error_msg: String },

    #[snafu(display("{error_msg}"))]
    Algod { error_msg: String },

    #[snafu(display("{error_msg}"))]
    Msgpack { error_msg: String },

    #[snafu(display("{error_msg}"))]
    SimulateResponseShape { error_msg: String },

    #[snafu(display("{error_msg}"))]
    InvalidSimulateOptions { error_msg: String },

    #[snafu(display("{error_msg}"))]
    SimulationFailed { error_msg: String },
}

impl From<RustComposerError> for AlgoKitComposerError {
    fn from(e: RustComposerError) -> Self {
        let error_msg = e.to_string();
        match e {
            RustComposerError::InvalidAddress { .. } => Self::InvalidAddress { error_msg },
            RustComposerError::InvalidByteLength { .. } => Self::InvalidByteLength { error_msg },
            RustComposerError::SignerCountMismatch { .. } => {
                Self::SignerCountMismatch { error_msg }
            }
            RustComposerError::Signing { .. } => Self::Signing { error_msg },
            RustComposerError::Transact { .. } => Self::Transact { error_msg },
            RustComposerError::Algod { .. } => Self::Algod { error_msg },
            RustComposerError::Msgpack { .. } => Self::Msgpack { error_msg },
            RustComposerError::SimulateResponseShape { .. } => {
                Self::SimulateResponseShape { error_msg }
            }
            RustComposerError::InvalidSimulateOptions { .. } => {
                Self::InvalidSimulateOptions { error_msg }
            }
            RustComposerError::SimulationFailed { .. } => Self::SimulationFailed { error_msg },
        }
    }
}

impl From<algokit_transact::AlgoKitTransactError> for AlgoKitComposerError {
    fn from(e: algokit_transact::AlgoKitTransactError) -> Self {
        Self::Transact {
            error_msg: e.to_string(),
        }
    }
}

#[ffi_record]
pub struct CommonTxnParams {
    pub sender: String,
    pub note: Option<Vec<u8>>,
    pub lease: Option<Vec<u8>>,
    pub rekey_to: Option<String>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u64>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
}

#[ffi_record]
pub struct PaymentParams {
    pub common: CommonTxnParams,
    pub receiver: String,
    pub amount: u64,
    pub close_remainder_to: Option<String>,
}

#[ffi_record]
pub struct AssetTransferParams {
    pub common: CommonTxnParams,
    pub asset_id: u64,
    pub receiver: String,
    pub amount: u64,
    pub clawback_target: Option<String>,
    pub close_asset_to: Option<String>,
}

#[ffi_record]
pub struct AssetOptInParams {
    pub common: CommonTxnParams,
    pub asset_id: u64,
}

#[ffi_record]
pub struct OnlineKeyRegParams {
    pub common: CommonTxnParams,
    pub vote_key: Vec<u8>,
    pub selection_key: Vec<u8>,
    pub state_proof_key: Vec<u8>,
    pub vote_first: u64,
    pub vote_last: u64,
    pub vote_key_dilution: u64,
}

#[ffi_record]
pub struct OfflineKeyRegParams {
    pub common: CommonTxnParams,
}

#[ffi_enum]
pub enum TxnParamsKind {
    Payment,
    AssetTransfer,
    AssetOptIn,
    OnlineKeyReg,
    OfflineKeyReg,
}

/// Flattened tagged-union for transaction inputs. Exactly one of the variant
/// payload fields must be Some, matching `kind`.
#[ffi_record]
pub struct TxnParams {
    pub kind: TxnParamsKind,
    pub payment: Option<PaymentParams>,
    pub asset_transfer: Option<AssetTransferParams>,
    pub asset_opt_in: Option<AssetOptInParams>,
    pub online_key_reg: Option<OnlineKeyRegParams>,
    pub offline_key_reg: Option<OfflineKeyRegParams>,
}

#[ffi_record]
pub struct SuggestedParams {
    pub fee: u64,
    pub flat_fee: bool,
    pub first_round_valid: u64,
    pub last_round_valid: u64,
    pub genesis_hash: Vec<u8>,
    pub genesis_id: String,
}

#[ffi_record]
pub struct ComposerParams {
    pub suggested_params: SuggestedParams,
    pub default_validity_window: Option<u64>,
}

impl From<CommonTxnParams> for RustCommonTxnParams {
    fn from(p: CommonTxnParams) -> Self {
        Self {
            sender: p.sender,
            note: p.note,
            lease: p.lease,
            rekey_to: p.rekey_to,
            static_fee: p.static_fee,
            extra_fee: p.extra_fee,
            max_fee: p.max_fee,
            validity_window: p.validity_window,
            first_valid_round: p.first_valid_round,
            last_valid_round: p.last_valid_round,
        }
    }
}

impl From<PaymentParams> for RustPaymentParams {
    fn from(p: PaymentParams) -> Self {
        Self {
            common: p.common.into(),
            receiver: p.receiver,
            amount: p.amount,
            close_remainder_to: p.close_remainder_to,
        }
    }
}

impl From<AssetTransferParams> for RustAssetTransferParams {
    fn from(p: AssetTransferParams) -> Self {
        Self {
            common: p.common.into(),
            asset_id: p.asset_id,
            receiver: p.receiver,
            amount: p.amount,
            clawback_target: p.clawback_target,
            close_asset_to: p.close_asset_to,
        }
    }
}

impl From<AssetOptInParams> for RustAssetOptInParams {
    fn from(p: AssetOptInParams) -> Self {
        Self {
            common: p.common.into(),
            asset_id: p.asset_id,
        }
    }
}

impl From<OnlineKeyRegParams> for RustOnlineKeyRegParams {
    fn from(p: OnlineKeyRegParams) -> Self {
        Self {
            common: p.common.into(),
            vote_key: p.vote_key,
            selection_key: p.selection_key,
            state_proof_key: p.state_proof_key,
            vote_first: p.vote_first,
            vote_last: p.vote_last,
            vote_key_dilution: p.vote_key_dilution,
        }
    }
}

impl From<OfflineKeyRegParams> for RustOfflineKeyRegParams {
    fn from(p: OfflineKeyRegParams) -> Self {
        Self {
            common: p.common.into(),
        }
    }
}

impl From<SuggestedParams> for RustSuggestedParams {
    fn from(p: SuggestedParams) -> Self {
        Self {
            fee: p.fee,
            flat_fee: p.flat_fee,
            first_round_valid: p.first_round_valid,
            last_round_valid: p.last_round_valid,
            genesis_hash: p.genesis_hash,
            genesis_id: p.genesis_id,
        }
    }
}

impl From<ComposerParams> for RustComposerParams {
    fn from(p: ComposerParams) -> Self {
        Self {
            suggested_params: p.suggested_params.into(),
            default_validity_window: p.default_validity_window,
        }
    }
}

impl TryFrom<TxnParams> for RustTxnParams {
    type Error = AlgoKitComposerError;

    fn try_from(p: TxnParams) -> Result<Self, Self::Error> {
        match p.kind {
            TxnParamsKind::Payment => p.payment.map(|x| RustTxnParams::Payment(x.into())).ok_or(
                AlgoKitComposerError::InvalidTxnParams {
                    error_msg: "TxnParams.kind=Payment but payment field is None".to_string(),
                },
            ),
            TxnParamsKind::AssetTransfer => p
                .asset_transfer
                .map(|x| RustTxnParams::AssetTransfer(x.into()))
                .ok_or(AlgoKitComposerError::InvalidTxnParams {
                    error_msg: "TxnParams.kind=AssetTransfer but asset_transfer field is None"
                        .to_string(),
                }),
            TxnParamsKind::AssetOptIn => p
                .asset_opt_in
                .map(|x| RustTxnParams::AssetOptIn(x.into()))
                .ok_or(AlgoKitComposerError::InvalidTxnParams {
                    error_msg: "TxnParams.kind=AssetOptIn but asset_opt_in field is None"
                        .to_string(),
                }),
            TxnParamsKind::OnlineKeyReg => p
                .online_key_reg
                .map(|x| RustTxnParams::OnlineKeyReg(x.into()))
                .ok_or(AlgoKitComposerError::InvalidTxnParams {
                    error_msg: "TxnParams.kind=OnlineKeyReg but online_key_reg field is None"
                        .to_string(),
                }),
            TxnParamsKind::OfflineKeyReg => p
                .offline_key_reg
                .map(|x| RustTxnParams::OfflineKeyReg(x.into()))
                .ok_or(AlgoKitComposerError::InvalidTxnParams {
                    error_msg: "TxnParams.kind=OfflineKeyReg but offline_key_reg field is None"
                        .to_string(),
                }),
        }
    }
}

/// Build transactions from the supplied TxnParams list using composer-level
/// defaults. Returns each transaction encoded to MsgPack bytes. When the input
/// has more than one entry the results are atomically grouped (shared group ID).
#[ffi_func]
pub fn compose(
    txn_params: Vec<TxnParams>,
    composer_params: ComposerParams,
) -> Result<Vec<Vec<u8>>, AlgoKitComposerError> {
    let rust_params: Vec<RustTxnParams> = txn_params
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<_, _>>()?;
    let rust_composer: RustComposerParams = composer_params.into();

    let txns = algokit_composer::compose(rust_params, rust_composer)?;

    txns.into_iter()
        .map(|t| t.encode().map_err(Into::into))
        .collect()
}

/// Sign each MsgPack-encoded transaction with the corresponding 32-byte Ed25519
/// secret key and return the encoded signed transactions ready for submission.
#[ffi_func]
pub fn sign_transactions(
    transaction_bytes: Vec<Vec<u8>>,
    secret_keys: Vec<Vec<u8>>,
) -> Result<Vec<Vec<u8>>, AlgoKitComposerError> {
    let txns: Vec<RustTransaction> = transaction_bytes
        .iter()
        .map(|b| RustTransaction::decode(b))
        .collect::<Result<_, _>>()?;

    let signed: Vec<RustSignedTransaction> =
        algokit_composer::sign_transactions(txns, secret_keys)?;

    signed
        .into_iter()
        .map(|s| s.encode().map_err(Into::into))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::{TestDataMother, TransactionMother};
    use pretty_assertions::assert_eq;

    fn common_from(header: &algokit_transact::TransactionHeader) -> CommonTxnParams {
        CommonTxnParams {
            sender: header.sender.to_string(),
            note: header.note.clone(),
            lease: None,
            rekey_to: None,
            static_fee: header.fee,
            extra_fee: None,
            max_fee: None,
            validity_window: None,
            first_valid_round: None,
            last_valid_round: None,
        }
    }

    fn composer_params_from(header: &algokit_transact::TransactionHeader) -> ComposerParams {
        ComposerParams {
            suggested_params: SuggestedParams {
                fee: header.fee.unwrap_or(0),
                flat_fee: true,
                first_round_valid: header.first_valid,
                last_round_valid: header.last_valid,
                genesis_hash: header.genesis_hash.unwrap().to_vec(),
                genesis_id: header.genesis_id.clone().unwrap(),
            },
            default_validity_window: None,
        }
    }

    #[test]
    fn compose_payment_round_trip_matches_fixture() {
        let expected = TransactionMother::simple_payment().build().unwrap();
        let payment = match &expected {
            RustTransaction::Payment(p) => p.clone(),
            _ => panic!("expected Payment fixture"),
        };

        let composer_params = composer_params_from(&payment.header);
        let txn_params = TxnParams {
            kind: TxnParamsKind::Payment,
            payment: Some(PaymentParams {
                common: common_from(&payment.header),
                receiver: payment.receiver.to_string(),
                amount: payment.amount,
                close_remainder_to: None,
            }),
            asset_transfer: None,
            asset_opt_in: None,
            online_key_reg: None,
            offline_key_reg: None,
        };

        let encoded = compose(vec![txn_params], composer_params).unwrap();
        assert_eq!(encoded.len(), 1);
        assert_eq!(encoded[0], expected.encode().unwrap());
    }

    #[test]
    fn sign_transactions_matches_fixture() {
        let fixture = TestDataMother::simple_payment();
        let signed = sign_transactions(
            vec![fixture.unsigned_bytes.clone()],
            vec![fixture.signing_private_key.to_vec()],
        )
        .unwrap();
        assert_eq!(signed.len(), 1);
        assert_eq!(signed[0], fixture.signed_bytes);
    }

    #[test]
    fn try_from_txn_params_rejects_kind_field_mismatch() {
        let txn_params = TxnParams {
            kind: TxnParamsKind::Payment,
            payment: None,
            asset_transfer: None,
            asset_opt_in: None,
            online_key_reg: None,
            offline_key_reg: None,
        };
        let err: AlgoKitComposerError = RustTxnParams::try_from(txn_params).unwrap_err();
        assert!(matches!(err, AlgoKitComposerError::InvalidTxnParams { .. }));
    }
}
