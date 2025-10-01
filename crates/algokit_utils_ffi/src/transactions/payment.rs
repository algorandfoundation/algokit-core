use crate::create_transaction_params;
use crate::transactions::common::UtilsError;

use algokit_utils::transactions::PaymentParams as RustPaymentParams;

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
