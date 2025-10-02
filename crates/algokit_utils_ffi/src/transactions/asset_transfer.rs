use crate::create_transaction_params;
use crate::transactions::common::UtilsError;

use algokit_utils::transactions::{
    AssetClawbackParams as RustAssetClawbackParams, AssetOptInParams as RustAssetOptInParams,
    AssetOptOutParams as RustAssetOptOutParams, AssetTransferParams as RustAssetTransferParams,
};

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AssetTransferParams {
        /// The ID of the asset being transferred.
        pub asset_id: u64,

        /// The amount of the asset to transfer.
        pub amount: u64,

        /// The address that will receive the asset.
        pub receiver: String,
    }
}

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AssetOptInParams {
        /// The ID of the asset to opt into.
        pub asset_id: u64,
    }
}

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AssetOptOutParams {
        /// The ID of the asset to opt out of.
        pub asset_id: u64,

        /// The address to close the remainder to. If None, defaults to the asset creator.
        #[uniffi(default = None)]
        pub close_remainder_to: Option<String>,
    }
}

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AssetClawbackParams {
        /// The ID of the asset being clawed back.
        pub asset_id: u64,

        /// The amount of the asset to clawback.
        pub amount: u64,

        /// The address that will receive the clawed back asset.
        pub receiver: String,

        /// The address from which assets are taken.
        pub clawback_target: String,
    }
}

impl TryFrom<AssetTransferParams> for RustAssetTransferParams {
    type Error = UtilsError;

    fn try_from(params: AssetTransferParams) -> Result<Self, Self::Error> {
        Ok(RustAssetTransferParams {
            sender: params.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: params.signer.map(|s| {
                std::sync::Arc::new(super::common::RustTransactionSignerFromFfi { ffi_signer: s })
                    as std::sync::Arc<dyn algokit_utils::transactions::common::TransactionSigner>
            }),
            rekey_to: params
                .rekey_to
                .map(|r| r.parse())
                .transpose()
                .map_err(|e| UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                })?,
            note: params.note,
            lease: params.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            asset_id: params.asset_id,
            amount: params.amount,
            receiver: params
                .receiver
                .parse()
                .map_err(|e| UtilsError::UtilsError {
                    message: format!("Failed to parse receiver address: {}", e),
                })?,
        })
    }
}

impl TryFrom<AssetOptInParams> for RustAssetOptInParams {
    type Error = UtilsError;

    fn try_from(params: AssetOptInParams) -> Result<Self, Self::Error> {
        Ok(RustAssetOptInParams {
            sender: params.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: params.signer.map(|s| {
                std::sync::Arc::new(super::common::RustTransactionSignerFromFfi { ffi_signer: s })
                    as std::sync::Arc<dyn algokit_utils::transactions::common::TransactionSigner>
            }),
            rekey_to: params
                .rekey_to
                .map(|r| r.parse())
                .transpose()
                .map_err(|e| UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                })?,
            note: params.note,
            lease: params.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            asset_id: params.asset_id,
        })
    }
}

impl TryFrom<AssetOptOutParams> for RustAssetOptOutParams {
    type Error = UtilsError;

    fn try_from(params: AssetOptOutParams) -> Result<Self, Self::Error> {
        let close_remainder_to = match params.close_remainder_to {
            Some(addr) => Some(addr.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Failed to parse close_remainder_to address: {}", e),
            })?),
            None => None,
        };

        Ok(RustAssetOptOutParams {
            sender: params.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: params.signer.map(|s| {
                std::sync::Arc::new(super::common::RustTransactionSignerFromFfi { ffi_signer: s })
                    as std::sync::Arc<dyn algokit_utils::transactions::common::TransactionSigner>
            }),
            rekey_to: params
                .rekey_to
                .map(|r| r.parse())
                .transpose()
                .map_err(|e| UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                })?,
            note: params.note,
            lease: params.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            asset_id: params.asset_id,
            close_remainder_to,
        })
    }
}

impl TryFrom<AssetClawbackParams> for RustAssetClawbackParams {
    type Error = UtilsError;

    fn try_from(params: AssetClawbackParams) -> Result<Self, Self::Error> {
        Ok(RustAssetClawbackParams {
            sender: params.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: params.signer.map(|s| {
                std::sync::Arc::new(super::common::RustTransactionSignerFromFfi { ffi_signer: s })
                    as std::sync::Arc<dyn algokit_utils::transactions::common::TransactionSigner>
            }),
            rekey_to: params
                .rekey_to
                .map(|r| r.parse())
                .transpose()
                .map_err(|e| UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                })?,
            note: params.note,
            lease: params.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            asset_id: params.asset_id,
            amount: params.amount,
            receiver: params
                .receiver
                .parse()
                .map_err(|e| UtilsError::UtilsError {
                    message: format!("Failed to parse receiver address: {}", e),
                })?,
            clawback_target: params.clawback_target.parse().map_err(|e| {
                UtilsError::UtilsError {
                    message: format!("Failed to parse clawback_target address: {}", e),
                }
            })?,
        })
    }
}

// impl From<RustPaymentParams> for PaymentParams {
//     fn from(params: RustPaymentParams) -> Self {
//         PaymentParams {
//             sender: params.sender.to_string(),
//             signer: params.signer.map(|s| {
//                 Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
//                     as Arc<dyn TransactionSigner>
//             }),
//
//             rekey_to: params.rekey_to.map(|r| r.to_string()),
//             note: params.note,
//             lease: params.lease.map(|l| l.to_vec()),
//             static_fee: params.static_fee,
//             extra_fee: params.extra_fee,
//             max_fee: params.max_fee,
//             validity_window: params.validity_window,
//             first_valid_round: params.first_valid_round,
//             last_valid_round: params.last_valid_round,
//             receiver: params.receiver.to_string(),
//             amount: params.amount,
//         }
//     }
// }

impl From<RustAssetTransferParams> for AssetTransferParams {
    fn from(params: RustAssetTransferParams) -> Self {
        AssetTransferParams {
            sender: params.sender.to_string(),
            signer: params.signer.map(|s| {
                std::sync::Arc::new(super::common::FfiTransactionSignerFromRust { rust_signer: s })
                    as std::sync::Arc<dyn super::common::TransactionSigner>
            }),
            rekey_to: params.rekey_to.map(|r| r.to_string()),
            note: params.note,
            lease: params.lease.map(|l| l.to_vec()),
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            asset_id: params.asset_id,
            amount: params.amount,
            receiver: params.receiver.to_string(),
        }
    }
}

impl From<RustAssetOptInParams> for AssetOptInParams {
    fn from(params: RustAssetOptInParams) -> Self {
        AssetOptInParams {
            sender: params.sender.to_string(),
            signer: params.signer.map(|s| {
                std::sync::Arc::new(super::common::FfiTransactionSignerFromRust { rust_signer: s })
                    as std::sync::Arc<dyn super::common::TransactionSigner>
            }),
            rekey_to: params.rekey_to.map(|r| r.to_string()),
            note: params.note,
            lease: params.lease.map(|l| l.to_vec()),
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            asset_id: params.asset_id,
        }
    }
}

impl From<RustAssetOptOutParams> for AssetOptOutParams {
    fn from(params: RustAssetOptOutParams) -> Self {
        AssetOptOutParams {
            sender: params.sender.to_string(),
            signer: params.signer.map(|s| {
                std::sync::Arc::new(super::common::FfiTransactionSignerFromRust { rust_signer: s })
                    as std::sync::Arc<dyn super::common::TransactionSigner>
            }),
            rekey_to: params.rekey_to.map(|r| r.to_string()),
            note: params.note,
            lease: params.lease.map(|l| l.to_vec()),
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            asset_id: params.asset_id,
            close_remainder_to: params.close_remainder_to.map(|addr| addr.to_string()),
        }
    }
}

impl From<RustAssetClawbackParams> for AssetClawbackParams {
    fn from(params: RustAssetClawbackParams) -> Self {
        AssetClawbackParams {
            sender: params.sender.to_string(),
            signer: params.signer.map(|s| {
                std::sync::Arc::new(super::common::FfiTransactionSignerFromRust { rust_signer: s })
                    as std::sync::Arc<dyn super::common::TransactionSigner>
            }),
            rekey_to: params.rekey_to.map(|r| r.to_string()),
            note: params.note,
            lease: params.lease.map(|l| l.to_vec()),
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            asset_id: params.asset_id,
            amount: params.amount,
            receiver: params.receiver.to_string(),
            clawback_target: params.clawback_target.to_string(),
        }
    }
}
