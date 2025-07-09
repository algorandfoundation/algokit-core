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
    freeze_target: Account,

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
            freeze_target: tx.freeze_target.into(),
            frozen: tx.frozen.unwrap_or(false),
        }
    }
}

impl TryFrom<Transaction> for algokit_transact::AssetFreezeTransactionFields {
    type Error = AlgoKitTransactError;

    fn try_from(tx: Transaction) -> Result<Self, Self::Error> {
        if tx.transaction_type != TransactionType::AssetFreeze || tx.asset_freeze.is_none() {
            return Err(Self::Error::DecodingError(
                "Asset Freeze data missing".to_string(),
            ));
        }

        let data = tx.clone().asset_freeze.unwrap();
        let header: algokit_transact::TransactionHeader = tx.try_into()?;

        Ok(Self {
            header,
            asset_id: data.asset_id,
            freeze_target: data.freeze_target.try_into()?,
            frozen: Some(data.frozen),
        })
    }
}
