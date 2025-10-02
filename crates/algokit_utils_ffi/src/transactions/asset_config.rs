use crate::create_transaction_params;
use crate::transactions::common::UtilsError;

use algokit_utils::transactions::{
    AssetConfigParams as RustAssetConfigParams, AssetCreateParams as RustAssetCreateParams,
    AssetDestroyParams as RustAssetDestroyParams,
};

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AssetCreateParams {
        /// The total amount of the smallest divisible (decimal) unit to create.
        pub total: u64,

        /// The amount of decimal places the asset should have.
        #[uniffi(default = None)]
        pub decimals: Option<u32>,

        /// Whether the asset is frozen by default for all accounts.
        #[uniffi(default = None)]
        pub default_frozen: Option<bool>,

        /// The optional name of the asset.
        #[uniffi(default = None)]
        pub asset_name: Option<String>,

        /// The optional name of the unit of this asset.
        #[uniffi(default = None)]
        pub unit_name: Option<String>,

        /// Specifies an optional URL where more information about the asset can be retrieved.
        #[uniffi(default = None)]
        pub url: Option<String>,

        /// 32-byte hash of some metadata that is relevant to your asset.
        #[uniffi(default = None)]
        pub metadata_hash: Option<Vec<u8>>,

        /// The address of the optional account that can manage the configuration.
        #[uniffi(default = None)]
        pub manager: Option<String>,

        /// The address of the optional account that holds the reserve units.
        #[uniffi(default = None)]
        pub reserve: Option<String>,

        /// The address of the optional account that can freeze/unfreeze holdings.
        #[uniffi(default = None)]
        pub freeze: Option<String>,

        /// The address of the optional account that can clawback holdings.
        #[uniffi(default = None)]
        pub clawback: Option<String>,
    }
}

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AssetConfigParams {
        /// ID of the existing asset to be reconfigured.
        pub asset_id: u64,

        /// The address of the optional account that can manage the configuration.
        #[uniffi(default = None)]
        pub manager: Option<String>,

        /// The address of the optional account that holds the reserve units.
        #[uniffi(default = None)]
        pub reserve: Option<String>,

        /// The address of the optional account that can freeze/unfreeze holdings.
        #[uniffi(default = None)]
        pub freeze: Option<String>,

        /// The address of the optional account that can clawback holdings.
        #[uniffi(default = None)]
        pub clawback: Option<String>,
    }
}

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AssetDestroyParams {
        /// ID of the existing asset to be destroyed.
        pub asset_id: u64,
    }
}

fn parse_optional_address(
    addr_opt: Option<String>,
    field_name: &str,
) -> Result<Option<algokit_transact::Address>, UtilsError> {
    match addr_opt {
        Some(addr_str) => {
            let addr = addr_str.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Failed to parse {} address: {}", field_name, e),
            })?;
            Ok(Some(addr))
        }
        None => Ok(None),
    }
}

impl TryFrom<AssetCreateParams> for RustAssetCreateParams {
    type Error = UtilsError;

    fn try_from(params: AssetCreateParams) -> Result<Self, Self::Error> {
        let metadata_hash = match params.metadata_hash {
            Some(hash_vec) => {
                if hash_vec.len() != 32 {
                    return Err(UtilsError::UtilsError {
                        message: format!(
                            "metadata_hash must be exactly 32 bytes, got {}",
                            hash_vec.len()
                        ),
                    });
                }
                let mut hash_array = [0u8; 32];
                hash_array.copy_from_slice(&hash_vec);
                Some(hash_array)
            }
            None => None,
        };

        Ok(RustAssetCreateParams {
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
            total: params.total,
            decimals: params.decimals,
            default_frozen: params.default_frozen,
            asset_name: params.asset_name,
            unit_name: params.unit_name,
            url: params.url,
            metadata_hash,
            manager: parse_optional_address(params.manager, "manager")?,
            reserve: parse_optional_address(params.reserve, "reserve")?,
            freeze: parse_optional_address(params.freeze, "freeze")?,
            clawback: parse_optional_address(params.clawback, "clawback")?,
        })
    }
}

impl TryFrom<AssetConfigParams> for RustAssetConfigParams {
    type Error = UtilsError;

    fn try_from(params: AssetConfigParams) -> Result<Self, Self::Error> {
        Ok(RustAssetConfigParams {
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
            manager: parse_optional_address(params.manager, "manager")?,
            reserve: parse_optional_address(params.reserve, "reserve")?,
            freeze: parse_optional_address(params.freeze, "freeze")?,
            clawback: parse_optional_address(params.clawback, "clawback")?,
        })
    }
}

impl TryFrom<AssetDestroyParams> for RustAssetDestroyParams {
    type Error = UtilsError;

    fn try_from(params: AssetDestroyParams) -> Result<Self, Self::Error> {
        Ok(RustAssetDestroyParams {
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

impl From<RustAssetCreateParams> for AssetCreateParams {
    fn from(params: RustAssetCreateParams) -> Self {
        AssetCreateParams {
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
            total: params.total,
            decimals: params.decimals,
            default_frozen: params.default_frozen,
            asset_name: params.asset_name,
            unit_name: params.unit_name,
            url: params.url,
            metadata_hash: params.metadata_hash.map(|h| h.to_vec()),
            manager: params.manager.map(|m| m.to_string()),
            reserve: params.reserve.map(|r| r.to_string()),
            freeze: params.freeze.map(|f| f.to_string()),
            clawback: params.clawback.map(|c| c.to_string()),
        }
    }
}

impl From<RustAssetConfigParams> for AssetConfigParams {
    fn from(params: RustAssetConfigParams) -> Self {
        AssetConfigParams {
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
            manager: params.manager.map(|m| m.to_string()),
            reserve: params.reserve.map(|r| r.to_string()),
            freeze: params.freeze.map(|f| f.to_string()),
            clawback: params.clawback.map(|c| c.to_string()),
        }
    }
}

impl From<RustAssetDestroyParams> for AssetDestroyParams {
    fn from(params: RustAssetDestroyParams) -> Self {
        AssetDestroyParams {
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
