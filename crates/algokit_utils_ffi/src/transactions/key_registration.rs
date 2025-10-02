use crate::create_transaction_params;
use crate::transactions::common::UtilsError;

use algokit_utils::transactions::{
    NonParticipationKeyRegistrationParams as RustNonParticipationKeyRegistrationParams,
    OfflineKeyRegistrationParams as RustOfflineKeyRegistrationParams,
    OnlineKeyRegistrationParams as RustOnlineKeyRegistrationParams,
};

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct OnlineKeyRegistrationParams {
        /// The root participation public key.
        pub vote_key: Vec<u8>,

        /// The VRF public key.
        pub selection_key: Vec<u8>,

        /// The first round that the participation key is valid.
        pub vote_first: u64,

        /// The last round that the participation key is valid.
        pub vote_last: u64,

        /// This is the dilution for the 2-level participation key.
        pub vote_key_dilution: u64,

        /// The 64 byte state proof public key commitment.
        #[uniffi(default = None)]
        pub state_proof_key: Option<Vec<u8>>,
    }
}

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct OfflineKeyRegistrationParams {
        // No additional fields beyond common params
    }
}

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct NonParticipationKeyRegistrationParams {
        // No additional fields beyond common params
    }
}

impl TryFrom<OnlineKeyRegistrationParams> for RustOnlineKeyRegistrationParams {
    type Error = UtilsError;

    fn try_from(params: OnlineKeyRegistrationParams) -> Result<Self, Self::Error> {
        let vote_key: [u8; 32] =
            params
                .vote_key
                .try_into()
                .map_err(|_| UtilsError::UtilsError {
                    message: "Vote key must be exactly 32 bytes".to_string(),
                })?;

        let selection_key: [u8; 32] =
            params
                .selection_key
                .try_into()
                .map_err(|_| UtilsError::UtilsError {
                    message: "Selection key must be exactly 32 bytes".to_string(),
                })?;

        let state_proof_key = match params.state_proof_key {
            Some(key) => {
                let key_array: [u8; 64] = key.try_into().map_err(|_| UtilsError::UtilsError {
                    message: "State proof key must be exactly 64 bytes".to_string(),
                })?;
                Some(key_array)
            }
            None => None,
        };

        Ok(RustOnlineKeyRegistrationParams {
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
            vote_key,
            selection_key,
            vote_first: params.vote_first,
            vote_last: params.vote_last,
            vote_key_dilution: params.vote_key_dilution,
            state_proof_key,
        })
    }
}

impl TryFrom<OfflineKeyRegistrationParams> for RustOfflineKeyRegistrationParams {
    type Error = UtilsError;

    fn try_from(params: OfflineKeyRegistrationParams) -> Result<Self, Self::Error> {
        Ok(RustOfflineKeyRegistrationParams {
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
        })
    }
}

impl TryFrom<NonParticipationKeyRegistrationParams> for RustNonParticipationKeyRegistrationParams {
    type Error = UtilsError;

    fn try_from(params: NonParticipationKeyRegistrationParams) -> Result<Self, Self::Error> {
        Ok(RustNonParticipationKeyRegistrationParams {
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
        })
    }
}

impl From<RustOnlineKeyRegistrationParams> for OnlineKeyRegistrationParams {
    fn from(params: RustOnlineKeyRegistrationParams) -> Self {
        OnlineKeyRegistrationParams {
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
            vote_key: params.vote_key.to_vec(),
            selection_key: params.selection_key.to_vec(),
            vote_first: params.vote_first,
            vote_last: params.vote_last,
            vote_key_dilution: params.vote_key_dilution,
            state_proof_key: params.state_proof_key.map(|k| k.to_vec()),
        }
    }
}

impl From<RustOfflineKeyRegistrationParams> for OfflineKeyRegistrationParams {
    fn from(params: RustOfflineKeyRegistrationParams) -> Self {
        OfflineKeyRegistrationParams {
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
        }
    }
}

impl From<RustNonParticipationKeyRegistrationParams> for NonParticipationKeyRegistrationParams {
    fn from(params: RustNonParticipationKeyRegistrationParams) -> Self {
        NonParticipationKeyRegistrationParams {
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
        }
    }
}
