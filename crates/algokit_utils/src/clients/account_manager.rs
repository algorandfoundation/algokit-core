use std::{collections::HashMap, sync::Arc};

use algokit_transact::Address;
use snafu::Snafu;

use crate::{TransactionSigner, transactions::common::TransactionSignerGetter};

pub struct AccountManager {
    default_signer: Option<Arc<dyn TransactionSigner>>,
    accounts: HashMap<Address, Arc<dyn TransactionSigner>>,
}

impl Default for AccountManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AccountManager {
    pub fn new() -> Self {
        Self {
            default_signer: None,
            accounts: HashMap::new(),
        }
    }

    pub fn set_default_signer(&mut self, default_signer: Arc<dyn TransactionSigner>) {
        self.default_signer = Some(default_signer);
    }

    pub fn set_signer(&mut self, sender: Address, signer: Arc<dyn TransactionSigner>) {
        self.accounts.insert(sender, signer);
    }

    pub fn get_signer(
        &self,
        sender: Address,
    ) -> Result<Arc<dyn TransactionSigner>, AccountManagerError> {
        self.accounts
            .get(&sender)
            .cloned()
            .or(self.default_signer.clone())
            .ok_or_else(|| AccountManagerError::SignerNotFound {
                address: sender.to_string(),
            })
    }
}

#[derive(Debug, Snafu)]
pub enum AccountManagerError {
    #[snafu(display("No signer found for address: {address}"))]
    SignerNotFound { address: String },
}

impl TransactionSignerGetter for AccountManager {
    fn get_signer(&self, address: Address) -> Result<Arc<dyn TransactionSigner>, String> {
        self.get_signer(address).map_err(|e| e.to_string())
    }
}
