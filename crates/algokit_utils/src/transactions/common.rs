use algokit_transact::{Address, EMPTY_SIGNATURE, SignedTransaction, Transaction};
use async_trait::async_trait;
use derive_more::Debug;
use std::sync::{Arc, Mutex};

#[async_trait]
pub trait TransactionSigner: Send + Sync {
    async fn sign_transactions(
        &self,
        transactions: &[Transaction],
        indices: &[usize],
    ) -> Result<Vec<SignedTransaction>, String>;

    async fn sign_transaction(
        &self,
        transaction: &Transaction,
    ) -> Result<SignedTransaction, String> {
        let result = self.sign_transactions(&[transaction.clone()], &[0]).await?;
        Ok(result[0].clone())
    }
}

pub trait TransactionSignerGetter {
    fn get_signer(&self, address: Address) -> Result<Arc<dyn TransactionSigner>, String>;
}

impl<T: TransactionSignerGetter> TransactionSignerGetter for Mutex<T> {
    fn get_signer(&self, address: Address) -> Result<Arc<dyn TransactionSigner>, String> {
        self.lock().map_err(|e| e.to_string())?.get_signer(address)
    }
}

#[derive(Clone)]
pub struct EmptySigner {}

#[async_trait]
impl TransactionSigner for EmptySigner {
    async fn sign_transactions(
        &self,
        txns: &[Transaction],
        indices: &[usize],
    ) -> Result<Vec<SignedTransaction>, String> {
        indices
            .iter()
            .map(|&idx| {
                if idx < txns.len() {
                    Ok(SignedTransaction {
                        transaction: txns[idx].clone(),
                        signature: Some(EMPTY_SIGNATURE),
                        auth_address: None,
                        multisignature: None,
                    })
                } else {
                    Err(format!("Index {} out of bounds for transactions", idx))
                }
            })
            .collect()
    }
}

impl TransactionSignerGetter for EmptySigner {
    fn get_signer(&self, _address: Address) -> Result<Arc<dyn TransactionSigner>, String> {
        Ok(Arc::new(self.clone()))
    }
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
            pub sender: algokit_transact::Address,
            #[debug(skip)]
            /// A signer used to sign transaction(s); if not specified then
            /// an attempt will be made to find a registered signer for the
            ///  given `sender` or use a default signer (if configured).
            pub signer: Option<std::sync::Arc<dyn $crate::TransactionSigner>>,
            /// Change the signing key of the sender to the given address.
            /// **Warning:** Please be careful with this parameter and be sure to read the [official rekey guidance](https://dev.algorand.co/concepts/accounts/rekeying).
            pub rekey_to: Option<algokit_transact::Address>,
            /// Note to attach to the transaction. Max of 1000 bytes.
            pub note: Option<Vec<u8>>,
            /// Prevent multiple transactions with the same lease being included within the validity window.
            ///
            /// A [lease](https://dev.algorand.co/concepts/transactions/leases)
            /// enforces a mutually exclusive transaction (useful to prevent double-posting and other scenarios).
            pub lease: Option<[u8; 32]>,
            /// The static transaction fee. In most cases you want to use extra fee unless setting the fee to 0 to be covered by another transaction.
            pub static_fee: Option<u64>,
            /// The fee to pay IN ADDITION to the suggested fee. Useful for manually covering inner transaction fees.
            pub extra_fee: Option<u64>,
            /// Throw an error if the fee for the transaction is more than this amount; prevents overspending on fees during high congestion periods.
            pub max_fee: Option<u64>,
            /// How many rounds the transaction should be valid for, if not specified then the registered default validity window will be used.
            pub validity_window: Option<u32>,
            /// Set the first round this transaction is valid.
            /// If left undefined, the value from algod will be used.
            ///
            /// We recommend you only set this when you intentionally want this to be some time in the future.
            pub first_valid_round: Option<u64>,
            /// The last round this transaction is valid. It is recommended to use validity window instead.
            pub last_valid_round: Option<u64>,
            // Specific fields
            $(
                $(#[$field_attr])*
                pub $field: $field_type,
            )*
        }
    };
}

#[derive(Debug, Clone)]
pub struct TransactionWithSigner {
    pub transaction: Transaction,
    #[debug(skip)]
    pub signer: Arc<dyn TransactionSigner>,
}
