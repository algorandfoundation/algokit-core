use crate::*;

#[ffi_record]
pub struct AssetTransferTransactionFields {
    asset_id: u64,

    amount: u64,

    receiver: String,

    asset_sender: Option<String>,

    close_remainder_to: Option<String>,
}

impl From<algokit_transact::AssetTransferTransactionFields> for AssetTransferTransactionFields {
    fn from(tx: algokit_transact::AssetTransferTransactionFields) -> Self {
        Self {
            asset_id: tx.asset_id,
            amount: tx.amount,
            receiver: tx.receiver.as_str(),
            asset_sender: tx.asset_sender.map(|addr| addr.as_str()),
            close_remainder_to: tx.close_remainder_to.map(|addr| addr.as_str()),
        }
    }
}

impl TryFrom<Transaction> for algokit_transact::AssetTransferTransactionFields {
    type Error = AlgoKitTransactError;

    fn try_from(tx: Transaction) -> Result<Self, Self::Error> {
        if tx.transaction_type != TransactionType::AssetTransfer || tx.asset_transfer.is_none() {
            return Err(Self::Error::DecodingError {
                message: "Asset Transfer data missing".to_string(),
            });
        }

        let data = tx.clone().asset_transfer.unwrap();
        let header: algokit_transact::TransactionHeader = tx.try_into()?;

        let transaction_fields = Self {
            header,
            asset_id: data.asset_id,
            amount: data.amount,
            receiver: data.receiver.parse()?,
            asset_sender: data.asset_sender.map(|addr| addr.parse()).transpose()?,
            close_remainder_to: data
                .close_remainder_to
                .map(|addr| addr.parse())
                .transpose()?,
        };

        transaction_fields
            .validate()
            .map_err(|errors| AlgoKitTransactError::DecodingError {
                message: format!("Asset transfer validation failed: {}", errors.join(", ")),
            })?;

        Ok(transaction_fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::{TestDataMother, TransactionMother};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_encoded_asset_transfer_transaction_type() {
        let txn: Transaction = TransactionMother::simple_asset_transfer()
            .build()
            .unwrap()
            .try_into()
            .unwrap();

        // Encode the transaction
        let encoded = encode_transaction(txn).unwrap();

        // Test the get_encoded_transaction_type function
        let tx_type = get_encoded_transaction_type(&encoded).unwrap();
        assert_eq!(tx_type, TransactionType::AssetTransfer);
    }

    #[test]
    fn test_asset_transfer_transaction_id_ffi() {
        let data = TestDataMother::simple_asset_transfer();
        let tx_ffi: Transaction = data.transaction.try_into().unwrap();

        let actual_id = get_transaction_id(tx_ffi.clone()).unwrap();
        let actual_id_raw = get_transaction_id_raw(tx_ffi.clone()).unwrap();

        assert_eq!(actual_id, data.id);
        assert_eq!(actual_id_raw, data.id_raw);
    }

    #[test]
    fn test_encode_transaction_validation_integration() {
        // invalid
        let mut tx: Transaction = TestDataMother::simple_asset_transfer()
            .transaction
            .try_into()
            .unwrap();
        tx.asset_transfer.as_mut().unwrap().asset_id = 0;
        let result = encode_transaction(tx);
        assert!(result.is_err());

        // valid
        let result = encode_transaction(
            TestDataMother::simple_asset_transfer()
                .transaction
                .try_into()
                .unwrap(),
        );
        assert!(result.is_ok());
    }
}
