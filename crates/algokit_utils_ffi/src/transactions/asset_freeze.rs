use crate::create_transaction_params;
use crate::transactions::common::UtilsError;

use algokit_utils::transactions::{
    AssetFreezeParams as RustAssetFreezeParams, AssetUnfreezeParams as RustAssetUnfreezeParams,
};

use super::common::TransactionSigner;

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AssetFreezeParams {
        /// The ID of the asset being frozen.
        pub asset_id: u64,

        /// The target account whose asset holdings will be frozen.
        pub target_address: String,
    }
}

impl TryFrom<AssetFreezeParams> for RustAssetFreezeParams {
    type Error = UtilsError;

    fn try_from(params: AssetFreezeParams) -> Result<Self, Self::Error> {
        Ok(RustAssetFreezeParams {
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
            target_address: params
                .target_address
                .parse()
                .map_err(|_| UtilsError::UtilsError {
                    message: "Invalid target address".to_string(),
                })?,
        })
    }
}

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AssetUnfreezeParams {
        /// The ID of the asset being unfrozen.
        pub asset_id: u64,

        /// The target account whose asset holdings will be unfrozen.
        pub target_address: String,
    }
}

impl TryFrom<AssetUnfreezeParams> for RustAssetUnfreezeParams {
    type Error = UtilsError;

    fn try_from(params: AssetUnfreezeParams) -> Result<Self, Self::Error> {
        Ok(RustAssetUnfreezeParams {
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
            target_address: params
                .target_address
                .parse()
                .map_err(|_| UtilsError::UtilsError {
                    message: "Invalid target address".to_string(),
                })?,
        })
    }
}

impl From<RustAssetFreezeParams> for AssetFreezeParams {
    fn from(params: RustAssetFreezeParams) -> Self {
        AssetFreezeParams {
            sender: params.sender.to_string(),
            signer: params.signer.map(|s| {
                std::sync::Arc::new(super::common::FfiTransactionSignerFromRust { rust_signer: s })
                    as std::sync::Arc<dyn TransactionSigner>
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
            target_address: params.target_address.to_string(),
        }
    }
}

impl From<RustAssetUnfreezeParams> for AssetUnfreezeParams {
    fn from(params: RustAssetUnfreezeParams) -> Self {
        AssetUnfreezeParams {
            sender: params.sender.to_string(),
            signer: params.signer.map(|s| {
                std::sync::Arc::new(super::common::FfiTransactionSignerFromRust { rust_signer: s })
                    as std::sync::Arc<dyn TransactionSigner>
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
            target_address: params.target_address.to_string(),
        }
    }
}
