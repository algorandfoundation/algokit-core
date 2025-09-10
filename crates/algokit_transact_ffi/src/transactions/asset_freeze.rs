use crate::*;

/// Represents an asset freeze transaction that freezes or unfreezes asset holdings.
///
/// Asset freeze transactions are used by the asset freeze account to control
/// whether a specific account can transfer a particular asset.
#[ffi_record]
pub struct AssetFreezeTransactionFields {
    /// The ID of the asset being frozen/unfrozen.
    asset_id: u64,

    /// The target account whose asset holdings will be affected.
    freeze_target: String,

    /// The new freeze status.
    ///
    /// `true` to freeze the asset holdings (prevent transfers),
    /// `false` to unfreeze the asset holdings (allow transfers).
    frozen: bool,
}

impl From<algokit_transact::AssetFreezeTransactionFields> for AssetFreezeTransactionFields {
    fn from(tx: algokit_transact::AssetFreezeTransactionFields) -> Self {
        Self {
            asset_id: tx.asset_id,
            freeze_target: tx.freeze_target.to_string(),
            frozen: tx.frozen,
        }
    }
}

impl TryFrom<Transaction> for algokit_transact::AssetFreezeTransactionFields {
    type Error = AlgoKitTransactError;

    fn try_from(tx: Transaction) -> Result<Self, Self::Error> {
        if tx.transaction_type != TransactionType::AssetFreeze || tx.asset_freeze.is_none() {
            return Err(Self::Error::DecodingError {
                message: "Asset Freeze data missing".to_string(),
            });
        }

        let data = tx.clone().asset_freeze.unwrap();
        let header: algokit_transact::TransactionHeader = tx.try_into()?;

        let transaction_fields = Self {
            header,
            asset_id: data.asset_id,
            freeze_target: data.freeze_target.parse()?,
            frozen: data.frozen,
        };

        transaction_fields
            .validate()
            .map_err(|errors| AlgoKitTransactError::DecodingError {
                message: format!("Asset freeze validation failed: {}", errors.join(", ")),
            })?;

        Ok(transaction_fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::TestDataMother;

    #[test]
    fn test_encode_transaction_validation_integration() {
        // invalid
        let mut tx: Transaction = TestDataMother::asset_freeze()
            .transaction
            .try_into()
            .unwrap();
        tx.asset_freeze.as_mut().unwrap().asset_id = 0;
        let result = encode_transaction(tx);
        assert!(result.is_err());

        // valid
        let result = encode_transaction(
            TestDataMother::asset_freeze()
                .transaction
                .try_into()
                .unwrap(),
        );
        assert!(result.is_ok());
    }
}
