use algod_client::models::PendingTransactionResponse;
use algokit_abi::ABIReturn;
use algokit_transact::{
    Address, AppCallTransactionFields, AssetConfigTransactionFields, Transaction,
};
use snafu::Snafu;

/// The unified, comprehensive result of sending a single transaction or transaction group.
///
/// This struct provides complete information about the transaction(s) that were sent,
/// including parsed transaction data, confirmations, and group context. It replaces
/// all the individual result structs with a single, powerful interface.
#[derive(Debug, Clone)]
pub struct SendTransactionResult {
    // Convenience accessors for the primary transaction (last in group)
    /// The transaction data for the primary transaction
    pub transaction: Transaction,
    /// The confirmation for the primary transaction
    pub confirmation: PendingTransactionResponse,
    /// The transaction ID of the primary transaction
    pub tx_id: String,

    // Full context for the entire group
    /// The group ID for the transaction group
    pub group_id: String,
    /// All transaction IDs in the group
    pub tx_ids: Vec<String>,
    /// All transactions in the group
    pub transactions: Vec<Transaction>,
    /// All confirmations in the group
    pub confirmations: Vec<PendingTransactionResponse>,

    // ABI support
    /// ABI return values for app calls (if any)
    pub abi_returns: Option<Vec<ABIReturn>>,
}

/// Result of sending an asset creation transaction.
///
/// This is a specialized result that includes the asset ID extracted from the confirmation.
#[derive(Debug, Clone)]
pub struct SendAssetCreateResult {
    /// The common transaction result containing all standard information
    pub common_params: SendTransactionResult,
    /// The ID of the newly created asset (extracted from confirmation)
    pub asset_id: u64,
}

/// Result of sending an app creation transaction.
///
/// This is a specialized result that includes the app ID and address extracted from the confirmation.
#[derive(Debug, Clone)]
pub struct SendAppCreateResult {
    /// The common transaction result containing all standard information
    pub common_params: SendTransactionResult,
    /// The ID of the newly created app (extracted from confirmation)
    pub app_id: u64,
    /// The address of the newly created app
    pub app_address: Address,
    /// The ABI return value if this was an ABI method call
    pub abi_return: Option<ABIReturn>,
    /// The compiled approval program (if provided)
    pub compiled_approval: Option<Vec<u8>>,
    /// The compiled clear state program (if provided)
    pub compiled_clear: Option<Vec<u8>>,
}

/// Result of sending an app update transaction.
///
/// This is a specialized result that includes the ABI return and compilation results.
#[derive(Debug, Clone)]
pub struct SendAppUpdateResult {
    /// The common transaction result containing all standard information
    pub common_params: SendTransactionResult,
    /// The ABI return value if this was an ABI method call
    pub abi_return: Option<ABIReturn>,
    /// The compiled approval program (if provided)
    pub compiled_approval: Option<Vec<u8>>,
    /// The compiled clear state program (if provided)
    pub compiled_clear: Option<Vec<u8>>,
}

/// Result of sending an app call transaction.
///
/// This is a specialized result that includes the ABI return value.
#[derive(Debug, Clone)]
pub struct SendAppCallResult {
    /// The common transaction result containing all standard information
    pub common_params: SendTransactionResult,
    /// The ABI return value if this was an ABI method call
    pub abi_return: Option<ABIReturn>,
}

/// Errors that can occur when constructing transaction results
#[derive(Debug, Snafu)]
pub enum TransactionResultError {
    #[snafu(display("Missing confirmation data: {message}"))]
    MissingConfirmation { message: String },
    #[snafu(display("Invalid confirmation data: {message}"))]
    InvalidConfirmation { message: String },
    #[snafu(display("Transaction parsing error: {message}"))]
    ParsingError { message: String },
}

impl SendTransactionResult {
    /// Create a new unified transaction result from composer output.
    ///
    /// This function takes the raw results from the transaction composer and
    /// processes them into the rich, parsed format that developers expect.
    pub fn new(
        group_id: String,
        tx_ids: Vec<String>,
        transactions: Vec<Transaction>,
        confirmations: Vec<PendingTransactionResponse>,
        abi_returns: Option<Vec<ABIReturn>>,
    ) -> Result<Self, TransactionResultError> {
        if transactions.is_empty() {
            return Err(TransactionResultError::MissingConfirmation {
                message: "No transactions provided".to_string(),
            });
        }

        if confirmations.is_empty() {
            return Err(TransactionResultError::MissingConfirmation {
                message: "No confirmations provided".to_string(),
            });
        }

        if tx_ids.len() != transactions.len() || tx_ids.len() != confirmations.len() {
            return Err(TransactionResultError::InvalidConfirmation {
                message: "Mismatched transaction, confirmation, and ID counts".to_string(),
            });
        }

        // The primary transaction is the last one in the group
        let transaction = transactions.last().unwrap().clone();
        let confirmation = confirmations.last().unwrap().clone();
        let tx_id = tx_ids.last().unwrap().clone();

        Ok(SendTransactionResult {
            transaction,
            confirmation,
            tx_id,
            group_id,
            tx_ids,
            transactions,
            confirmations,
            abi_returns,
        })
    }

    /// Get the sender address of the primary transaction
    pub fn sender(&self) -> &Address {
        self.transaction.sender()
    }

    /// Get the fee of the primary transaction
    pub fn fee(&self) -> Option<u64> {
        self.transaction.fee()
    }

    /// Get the first valid round of the primary transaction
    pub fn first_valid_round(&self) -> u64 {
        self.transaction.first_valid_round()
    }

    /// Get the last valid round of the primary transaction
    pub fn last_valid_round(&self) -> u64 {
        self.transaction.last_valid_round()
    }

    /// Get the note of the primary transaction
    pub fn note(&self) -> Option<&Vec<u8>> {
        self.transaction.note()
    }

    /// Check if this is a single transaction (not part of a group)
    pub fn is_single_transaction(&self) -> bool {
        self.transactions.len() == 1
    }

    /// Check if this is part of a transaction group
    pub fn is_group_transaction(&self) -> bool {
        self.transactions.len() > 1
    }

    /// Get the number of transactions in the group
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// Get the total fees paid by all transactions in the group
    pub fn total_fees(&self) -> u64 {
        self.transactions
            .iter()
            .map(|tx| tx.fee().unwrap_or(0))
            .sum()
    }

    /// Get all ABI returns from the group (convenience method)
    pub fn all_abi_returns(&self) -> Vec<&ABIReturn> {
        self.abi_returns
            .as_ref()
            .map(|returns| returns.iter().collect())
            .unwrap_or_default()
    }

    /// Find a transaction by its ID within the group
    pub fn find_transaction_by_id(
        &self,
        tx_id: &str,
    ) -> Option<(&Transaction, &PendingTransactionResponse)> {
        self.tx_ids
            .iter()
            .position(|id| id == tx_id)
            .map(|index| (&self.transactions[index], &self.confirmations[index]))
    }

    /// Get all transactions of a specific type from the group
    pub fn filter_transactions<F>(
        &self,
        predicate: F,
    ) -> Vec<(&Transaction, &PendingTransactionResponse)>
    where
        F: Fn(&Transaction) -> bool,
    {
        self.transactions
            .iter()
            .zip(self.confirmations.iter())
            .filter(|(tx, _)| predicate(tx))
            .collect()
    }

    /// Get all payment transactions from the group
    pub fn payment_transactions(&self) -> Vec<(&Transaction, &PendingTransactionResponse)> {
        self.filter_transactions(|tx| matches!(tx, Transaction::Payment(_)))
    }

    /// Get all asset transfer transactions from the group
    pub fn asset_transfer_transactions(&self) -> Vec<(&Transaction, &PendingTransactionResponse)> {
        self.filter_transactions(|tx| matches!(tx, Transaction::AssetTransfer(_)))
    }

    /// Get all app call transactions from the group
    pub fn app_call_transactions(&self) -> Vec<(&Transaction, &PendingTransactionResponse)> {
        self.filter_transactions(|tx| matches!(tx, Transaction::AppCall(_)))
    }
}

impl SendAssetCreateResult {
    /// Create a new asset creation result by extracting the asset ID from the confirmation
    pub fn new(common_params: SendTransactionResult) -> Result<Self, TransactionResultError> {
        // Extract asset ID from the confirmation
        let asset_id = common_params.confirmation.asset_id.ok_or_else(|| {
            TransactionResultError::InvalidConfirmation {
                message: "Asset creation confirmation missing asset-index".to_string(),
            }
        })?;

        Ok(SendAssetCreateResult {
            common_params,
            asset_id,
        })
    }

    /// Get the asset configuration transaction from the common transaction
    pub fn asset_config_transaction(&self) -> Option<&AssetConfigTransactionFields> {
        if let Transaction::AssetConfig(asset_config) = &self.common_params.transaction {
            Some(asset_config)
        } else {
            None
        }
    }
}

impl SendAppCreateResult {
    /// Create a new app creation result by extracting the app ID from the confirmation
    pub fn new(
        common_params: SendTransactionResult,
        abi_return: Option<ABIReturn>,
        compiled_approval: Option<Vec<u8>>,
        compiled_clear: Option<Vec<u8>>,
    ) -> Result<Self, TransactionResultError> {
        // Extract app ID from the confirmation
        let app_id = common_params.confirmation.app_id.ok_or_else(|| {
            TransactionResultError::InvalidConfirmation {
                message: "App creation confirmation missing application-index".to_string(),
            }
        })?;

        // Calculate app address
        let app_address = Address::from_app_id(&app_id);

        Ok(SendAppCreateResult {
            common_params,
            app_id,
            app_address,
            abi_return,
            compiled_approval,
            compiled_clear,
        })
    }

    /// Get the app call transaction from the common transaction
    pub fn app_call_transaction(&self) -> Option<&AppCallTransactionFields> {
        match &self.common_params.transaction {
            Transaction::AppCall(app_call) => Some(app_call),
            _ => None,
        }
    }
}

impl SendAppUpdateResult {
    /// Create a new app update result with compilation results
    pub fn new(
        common_params: SendTransactionResult,
        abi_return: Option<ABIReturn>,
        compiled_approval: Option<Vec<u8>>,
        compiled_clear: Option<Vec<u8>>,
    ) -> Self {
        SendAppUpdateResult {
            common_params,
            abi_return,
            compiled_approval,
            compiled_clear,
        }
    }
}

impl SendAppCallResult {
    /// Create a new app call result with ABI return
    pub fn new(common_params: SendTransactionResult, abi_return: Option<ABIReturn>) -> Self {
        SendAppCallResult {
            common_params,
            abi_return,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::{PaymentTransactionBuilder, TransactionHeaderBuilder};

    #[test]
    fn test_send_transaction_result_creation() {
        let header = TransactionHeaderBuilder::default()
            .sender(Address([0u8; 32]))
            .fee(1000)
            .first_valid(100)
            .last_valid(200)
            .build()
            .unwrap();

        let payment_tx = PaymentTransactionBuilder::default()
            .header(header)
            .receiver(Address([1u8; 32]))
            .amount(1_000_000)
            .build()
            .unwrap();

        // Mock confirmation (in real usage this comes from algod)
        let confirmation = PendingTransactionResponse {
            app_id: None,
            asset_id: None,
            asset_closing_amount: None,
            close_rewards: None,
            closing_amount: None,
            confirmed_round: Some(150),
            global_state_delta: None,
            inner_txns: None,
            local_state_delta: None,
            logs: None,
            pool_error: String::new(),
            receiver_rewards: None,
            sender_rewards: None,
            // Create a minimal SignedTransaction for testing
            txn: algokit_transact::SignedTransaction {
                transaction: algokit_transact::Transaction::Payment(
                    algokit_transact::PaymentTransactionFields {
                        header: algokit_transact::TransactionHeader {
                            sender: Address([0u8; 32]),
                            fee: Some(1000),
                            first_valid: 100,
                            last_valid: 200,
                            genesis_hash: None,
                            genesis_id: None,
                            note: None,
                            rekey_to: None,
                            lease: None,
                            group: None,
                        },
                        receiver: Address([1u8; 32]),
                        amount: 1_000_000,
                        close_remainder_to: None,
                    },
                ),
                signature: Some([0u8; 64]),
                auth_address: None,
                multisignature: None,
            },
        };

        let result = SendTransactionResult::new(
            "test-group-id".to_string(),
            vec!["test-tx-id".to_string()],
            vec![payment_tx],
            vec![confirmation],
            None,
        )
        .unwrap();

        assert_eq!(result.tx_id, "test-tx-id");
        assert_eq!(result.group_id, "test-group-id");
        assert_eq!(result.transaction_count(), 1);
        assert!(result.is_single_transaction());
        assert!(!result.is_group_transaction());
        assert_eq!(result.fee(), Some(1000));
    }

    #[test]
    fn test_send_transaction_result_validation() {
        // Test mismatched counts
        let result = SendTransactionResult::new(
            "group-id".to_string(),
            vec!["tx1".to_string(), "tx2".to_string()],
            vec![], // Empty transactions
            vec![], // Empty confirmations
            None,
        );

        assert!(result.is_err());
        match result.unwrap_err() {
            TransactionResultError::MissingConfirmation { message } => {
                assert!(message.contains("No transactions provided"));
            }
            _ => panic!("Expected MissingConfirmation error"),
        }
    }
}
