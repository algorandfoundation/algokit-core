use crate::*;

/// Parameters to define an asset config transaction.
///
/// For asset creation, the asset ID field must be 0.
/// For asset reconfiguration, the asset ID field must be set. Only fields manager, reserve, freeze, and clawback can be set.
/// For asset destroy, the asset ID field must be set, all other fields must not be set.
///
/// **Note:** The manager, reserve, freeze, and clawback addresses
/// are immutably empty if they are not set. If manager is not set then
/// all fields are immutable from that point forward.
#[ffi_record]
pub struct AssetConfigTransactionFields {
    /// ID of the asset to operate on.
    ///
    /// For asset creation, this must be 0.
    /// For asset reconfiguration this is the ID of the existing asset to be reconfigured,
    /// For asset destroy this is the ID of the existing asset to be destroyed.
    asset_id: u64,

    /// The total amount of the smallest divisible (decimal) unit to create.
    ///
    /// Required when creating a new asset.
    /// For example, if creating a asset with 2 decimals and wanting a total supply of 100 units, this value should be 10000.
    ///
    /// This field can only be specified upon asset creation.
    total: Option<u64>,

    /// The amount of decimal places the asset should have.
    ///
    /// If unspecified then the asset will be in whole units (i.e. `0`).
    /// * If 0, the asset is not divisible;
    /// * If 1, the base unit of the asset is in tenths;
    /// * If 2, the base unit of the asset is in hundredths;
    /// * If 3, the base unit of the asset is in thousandths;
    ///
    /// and so on up to 19 decimal places.
    ///
    /// This field can only be specified upon asset creation.
    decimals: Option<u32>,

    /// Whether the asset is frozen by default for all accounts.
    /// Defaults to `false`.
    ///
    /// If `true` then for anyone apart from the creator to hold the
    /// asset it needs to be unfrozen per account using an asset freeze
    /// transaction from the `freeze` account, which must be set on creation.
    ///
    /// This field can only be specified upon asset creation.
    default_frozen: Option<bool>,

    /// The optional name of the asset.
    ///
    /// Max size is 32 bytes.
    ///
    /// This field can only be specified upon asset creation.
    asset_name: Option<String>,

    /// The optional name of the unit of this asset (e.g. ticker name).
    ///
    /// Max size is 8 bytes.
    ///
    /// This field can only be specified upon asset creation.
    unit_name: Option<String>,

    /// Specifies an optional URL where more information about the asset can be retrieved (e.g. metadata).
    ///
    /// Max size is 96 bytes.
    ///
    /// This field can only be specified upon asset creation.
    url: Option<String>,

    /// 32-byte hash of some metadata that is relevant to your asset and/or asset holders.
    ///
    /// The format of this metadata is up to the application.
    ///
    /// This field can only be specified upon asset creation.
    metadata_hash: Option<ByteBuf>,

    /// The address of the optional account that can manage the configuration of the asset and destroy it.
    ///
    /// The fields it can change are `manager`, `reserve`, `clawback`, and `freeze`.
    ///
    /// If not set or set to the Zero address the asset becomes permanently immutable.
    manager: Option<String>,

    /// The address of the optional account that holds the reserve (uncirculated supply) units of the asset.
    ///
    /// This address has no specific authority in the protocol itself and is informational only.
    ///
    /// Some standards like [ARC-19](https://github.com/algorandfoundation/ARCs/blob/main/ARCs/arc-0019.md)
    /// rely on this field to hold meaningful data.
    ///
    /// It can be used in the case where you want to signal to holders of your asset that the uncirculated units
    /// of the asset reside in an account that is different from the default creator account.
    ///
    /// If not set or set to the Zero address is permanently empty.
    reserve: Option<String>,

    /// The address of the optional account that can be used to freeze or unfreeze holdings of this asset for any account.
    ///
    /// If empty, freezing is not permitted.
    ///
    /// If not set or set to the Zero address is permanently empty.
    freeze: Option<String>,

    /// The address of the optional account that can clawback holdings of this asset from any account.
    ///
    /// **This field should be used with caution** as the clawback account has the ability to **unconditionally take assets from any account**.
    ///
    /// If empty, clawback is not permitted.
    ///
    /// If not set or set to the Zero address is permanently empty.
    clawback: Option<String>,
}

impl From<algokit_transact::AssetConfigTransactionFields> for AssetConfigTransactionFields {
    fn from(tx: algokit_transact::AssetConfigTransactionFields) -> Self {
        Self {
            asset_id: tx.asset_id,
            total: tx.total,
            decimals: tx.decimals,
            default_frozen: tx.default_frozen,
            asset_name: tx.asset_name,
            unit_name: tx.unit_name,
            url: tx.url,
            metadata_hash: tx.metadata_hash.map(|h| h.to_vec().into()),
            manager: tx.manager.map(|addr| addr.as_str()),
            reserve: tx.reserve.map(|addr| addr.as_str()),
            freeze: tx.freeze.map(|addr| addr.as_str()),
            clawback: tx.clawback.map(|addr| addr.as_str()),
        }
    }
}

impl TryFrom<Transaction> for algokit_transact::AssetConfigTransactionFields {
    type Error = AlgoKitTransactError;

    fn try_from(tx: Transaction) -> Result<Self, Self::Error> {
        if tx.transaction_type != TransactionType::AssetConfig || tx.asset_config.is_none() {
            return Err(Self::Error::DecodingError(
                "Asset configuration data missing".to_string(),
            ));
        }

        let data = tx.clone().asset_config.unwrap();
        let header: algokit_transact::TransactionHeader = tx.try_into()?;

        let metadata_hash = data
            .metadata_hash
            .map(|buf| bytebuf_to_bytes::<32>(&buf))
            .transpose()?;

        let transaction_fields = algokit_transact::AssetConfigTransactionFields {
            header,
            asset_id: data.asset_id,
            total: data.total,
            decimals: data.decimals,
            default_frozen: data.default_frozen,
            asset_name: data.asset_name,
            unit_name: data.unit_name,
            url: data.url,
            metadata_hash,
            manager: data.manager.map(|addr| addr.parse()).transpose()?,
            reserve: data.reserve.map(|addr| addr.parse()).transpose()?,
            freeze: data.freeze.map(|addr| addr.parse()).transpose()?,
            clawback: data.clawback.map(|addr| addr.parse()).transpose()?,
        };

        transaction_fields.validate().map_err(|errors| {
            AlgoKitTransactError::DecodingError(format!(
                "Asset config validation failed: {}",
                errors.join("\n")
            ))
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
        let mut tx: Transaction = TestDataMother::asset_create()
            .transaction
            .try_into()
            .unwrap();
        tx.asset_config.as_mut().unwrap().asset_id = 123;
        let result = encode_transaction(tx);
        assert!(result.is_err());

        // valid
        let result = encode_transaction(
            TestDataMother::asset_create()
                .transaction
                .try_into()
                .unwrap(),
        );
        assert!(result.is_ok());
    }
}
