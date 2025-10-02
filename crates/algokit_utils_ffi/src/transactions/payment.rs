use std::sync::Arc;

use crate::create_transaction_params;
use crate::transactions::common::UtilsError;

use algokit_utils::transactions::{
    AccountCloseParams as RustAccountCloseParams, PaymentParams as RustPaymentParams,
};

use super::common::{FfiTransactionSignerFromRust, TransactionSigner};

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct PaymentParams {
        /// The address of the account receiving the ALGO payment.
        pub receiver: String,

        /// The amount of microALGO to send.
        ///
        /// Specified in microALGO (1 ALGO = 1,000,000 microALGO).
        pub amount: u64,
    }
}

impl TryFrom<PaymentParams> for RustPaymentParams {
    type Error = UtilsError;

    fn try_from(params: PaymentParams) -> Result<Self, Self::Error> {
        Ok(RustPaymentParams {
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
            receiver: params
                .receiver
                .parse()
                .map_err(|_| UtilsError::UtilsError {
                    message: "Invalid receiver address".to_string(),
                })?,
            amount: params.amount,
        })
    }
}

impl From<RustPaymentParams> for PaymentParams {
    fn from(params: RustPaymentParams) -> Self {
        PaymentParams {
            sender: params.sender.to_string(),
            signer: params.signer.map(|s| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
                    as Arc<dyn TransactionSigner>
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
            receiver: params.receiver.to_string(),
            amount: params.amount,
        }
    }
}

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AccountCloseParams {
        /// Close the sender account and send the remaining balance to this address
        ///
        /// *Warning:* Be careful this can lead to loss of funds if not used correctly.
        pub close_remainder_to: String,
    }
}

impl TryFrom<AccountCloseParams> for RustAccountCloseParams {
    type Error = UtilsError;

    fn try_from(params: AccountCloseParams) -> Result<Self, Self::Error> {
        Ok(RustAccountCloseParams {
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
            close_remainder_to: params.close_remainder_to.parse().map_err(|_| {
                UtilsError::UtilsError {
                    message: "Invalid close_remainder_to address".to_string(),
                }
            })?,
        })
    }
}

impl From<RustAccountCloseParams> for AccountCloseParams {
    fn from(params: RustAccountCloseParams) -> Self {
        AccountCloseParams {
            sender: params.sender.to_string(),
            signer: params.signer.map(|s| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
                    as Arc<dyn TransactionSigner>
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
            close_remainder_to: params.close_remainder_to.to_string(),
        }
    }
}
