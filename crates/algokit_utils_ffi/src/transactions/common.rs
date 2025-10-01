use std::sync::Arc;

use algokit_transact::Address;
use algokit_transact::SignedTransaction as RustSignedTransaction;
use algokit_transact::Transaction as RustTransaction;
use algokit_transact_ffi::{SignedTransaction, Transaction};
use algokit_utils::transactions::common::TransactionSigner as RustTransactionSigner;
use algokit_utils::transactions::common::TransactionSignerGetter as RustTransactionSignerGetter;
use algokit_utils::transactions::common::TransactionWithSigner as RustTransactionWithSigner;

use async_trait::async_trait;
use derive_more::Debug;
use snafu::Snafu;

// TODO: implement proper errors
#[derive(Debug, Snafu, uniffi::Error)]
pub enum UtilsError {
    #[snafu(display("UtilsError: {message}"))]
    UtilsError { message: String },
}

#[uniffi::export(with_foreign)]
#[async_trait]
pub trait TransactionSigner: Send + Sync {
    async fn sign_transactions(
        &self,
        transactions: Vec<Transaction>,
        indices: Vec<u32>,
    ) -> Result<Vec<SignedTransaction>, UtilsError>;

    async fn sign_transaction(
        &self,
        transaction: Transaction,
    ) -> Result<SignedTransaction, UtilsError>;
}

pub struct RustTransactionSignerFromFfi {
    pub ffi_signer: Arc<dyn TransactionSigner>,
}

#[async_trait]
impl RustTransactionSigner for RustTransactionSignerFromFfi {
    async fn sign_transactions(
        &self,
        transactions: &[RustTransaction],
        indices: &[usize],
    ) -> Result<Vec<RustSignedTransaction>, String> {
        let ffi_txns: Vec<Transaction> = transactions.iter().map(|t| t.to_owned().into()).collect();

        let ffi_signed_txns = self
            .ffi_signer
            .sign_transactions(ffi_txns, indices.iter().map(|&i| i as u32).collect())
            .await
            .map_err(|e| e.to_string())?;

        let signed_txns: Result<Vec<RustSignedTransaction>, _> = ffi_signed_txns
            .into_iter()
            .map(|st| st.try_into())
            .collect();
        signed_txns.map_err(|e| format!("Failed to convert signed transactions: {}", e))
    }
}

pub struct FfiTransactionSignerFromRust {
    pub rust_signer: Arc<dyn RustTransactionSigner>,
}

#[async_trait]
impl TransactionSigner for FfiTransactionSignerFromRust {
    async fn sign_transactions(
        &self,
        transactions: Vec<Transaction>,
        indices: Vec<u32>,
    ) -> Result<Vec<SignedTransaction>, UtilsError> {
        let rust_txns: Result<Vec<RustTransaction>, _> =
            transactions.into_iter().map(|t| t.try_into()).collect();
        let rust_txns = rust_txns.map_err(|e| UtilsError::UtilsError {
            message: format!("Failed to convert transactions: {}", e),
        })?;

        let signed_txns = self
            .rust_signer
            .sign_transactions(
                &rust_txns,
                &indices.iter().map(|&i| i as usize).collect::<Vec<_>>(),
            )
            .await
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })?;

        Ok(signed_txns.into_iter().map(|st| st.into()).collect())
    }

    async fn sign_transaction(
        &self,
        transaction: Transaction,
    ) -> Result<SignedTransaction, UtilsError> {
        let txns = vec![transaction];
        let indices = vec![0u32];
        let mut signed_txns = self.sign_transactions(txns, indices).await?;
        signed_txns.pop().ok_or(UtilsError::UtilsError {
            message: "No signed transaction returned".to_string(),
        })
    }
}

#[uniffi::export(with_foreign)]
pub trait TransactionSignerGetter: Send + Sync {
    fn get_signer(&self, address: String) -> Result<Arc<dyn TransactionSigner>, UtilsError>;
}

pub struct RustTransactionSignerGetterFromFfi {
    pub ffi_signer_getter: Arc<dyn TransactionSignerGetter>,
}

impl RustTransactionSignerGetter for RustTransactionSignerGetterFromFfi {
    fn get_signer(&self, address: Address) -> Result<Arc<dyn RustTransactionSigner>, String> {
        self.ffi_signer_getter
            .get_signer(address.to_string())
            .map(|ffi_signer| {
                Arc::new(RustTransactionSignerFromFfi { ffi_signer })
                    as Arc<dyn RustTransactionSigner>
            })
            .map_err(|e| e.to_string())
    }
}

pub struct FfiTransactionSignerGetterFromRust {
    pub rust_signer_getter: Arc<dyn RustTransactionSignerGetter>,
}

impl TransactionSignerGetter for FfiTransactionSignerGetterFromRust {
    fn get_signer(&self, address: String) -> Result<Arc<dyn TransactionSigner>, UtilsError> {
        self.rust_signer_getter
            .get_signer(address.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid address {address}: {e}"),
            })?)
            .map(|rust_signer| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer }) as Arc<dyn TransactionSigner>
            })
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }
}

pub struct TransactionWithSigner {
    pub transaction: Transaction,
    pub signer: Arc<dyn TransactionSigner>,
}

impl TryFrom<TransactionWithSigner> for RustTransactionWithSigner {
    type Error = UtilsError;

    fn try_from(value: TransactionWithSigner) -> Result<Self, Self::Error> {
        let rust_txn: RustTransaction =
            value
                .transaction
                .try_into()
                .map_err(|e| UtilsError::UtilsError {
                    message: format!("Failed to convert transaction: {}", e),
                })?;

        Ok(RustTransactionWithSigner {
            transaction: rust_txn,
            signer: Arc::new(RustTransactionSignerFromFfi {
                ffi_signer: value.signer,
            }),
        })
    }
}

impl TryFrom<RustTransactionWithSigner> for TransactionWithSigner {
    type Error = UtilsError;

    fn try_from(value: RustTransactionWithSigner) -> Result<Self, Self::Error> {
        let ffi_txn: Transaction = value.transaction.into();

        Ok(TransactionWithSigner {
            transaction: ffi_txn,
            signer: Arc::new(FfiTransactionSignerFromRust {
                rust_signer: value.signer,
            }),
        })
    }
}

#[derive(Debug, uniffi::Record)]
pub struct CommonParams {
    pub sender: String,
    #[debug(skip)]
    #[uniffi(default = None)]
    pub signer: Option<Arc<dyn TransactionSigner>>,
    #[uniffi(default = None)]
    pub rekey_to: Option<String>,
    #[uniffi(default = None)]
    pub note: Option<Vec<u8>>,
    #[uniffi(default = None)]
    pub lease: Option<Vec<u8>>,
    #[uniffi(default = None)]
    pub static_fee: Option<u64>,
    #[uniffi(default = None)]
    pub extra_fee: Option<u64>,
    #[uniffi(default = None)]
    pub max_fee: Option<u64>,
    #[uniffi(default = None)]
    pub validity_window: Option<u64>,
    #[uniffi(default = None)]
    pub first_valid_round: Option<u64>,
    #[uniffi(default = None)]
    pub last_valid_round: Option<u64>,
}

#[macro_export]
macro_rules! create_transaction_params {
    (
        $(#[$struct_attr:meta])*
        pub struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                pub $field:ident: $field_type:ty,
            )*
        }
    ) => {
        $(#[$struct_attr])*
        #[derive(derive_more::Debug)]
        pub struct $name {
            /// The address of the account sending the transaction.
            pub sender: String,
            #[debug(skip)]
            /// A signer used to sign transaction(s); if not specified then
            /// an attempt will be made to find a registered signer for the
            ///  given `sender` or use a default signer (if configured).
            #[uniffi(default = None)]
            pub signer: Option<std::sync::Arc<dyn $crate::transactions::common::TransactionSigner>>,
            /// Change the signing key of the sender to the given address.
            /// **Warning:** Please be careful with this parameter and be sure to read the [official rekey guidance](https://dev.algorand.co/concepts/accounts/rekeying).
            #[uniffi(default = None)]
            pub rekey_to: Option<String>,
            /// Note to attach to the transaction. Max of 1000 bytes.
            #[uniffi(default = None)]
            pub note: Option<Vec<u8>>,
            /// Prevent multiple transactions with the same lease being included within the validity window.
            ///
            /// A [lease](https://dev.algorand.co/concepts/transactions/leases)
            /// enforces a mutually exclusive transaction (useful to prevent double-posting and other scenarios).
            #[uniffi(default = None)]
            pub lease: Option<Vec<u8>>,
            /// The static transaction fee. In most cases you want to use extra fee unless setting the fee to 0 to be covered by another transaction.
            #[uniffi(default = None)]
            pub static_fee: Option<u64>,
            /// The fee to pay IN ADDITION to the suggested fee. Useful for manually covering inner transaction fees.
            #[uniffi(default = None)]
            pub extra_fee: Option<u64>,
            /// Throw an error if the fee for the transaction is more than this amount; prevents overspending on fees during high congestion periods.
            #[uniffi(default = None)]
            pub max_fee: Option<u64>,
            /// How many rounds the transaction should be valid for, if not specified then the registered default validity window will be used.
            #[uniffi(default = None)]
            pub validity_window: Option<u32>,
            /// Set the first round this transaction is valid.
            /// If left undefined, the value from algod will be used.
            ///
            /// We recommend you only set this when you intentionally want this to be some time in the future.
            #[uniffi(default = None)]
            pub first_valid_round: Option<u64>,
            /// The last round this transaction is valid. It is recommended to use validity window instead.
            #[uniffi(default = None)]
            pub last_valid_round: Option<u64>,
            // Specific fields
            $(
                $(#[$field_attr])*
                pub $field: $field_type,
            )*
        }
    };
}
