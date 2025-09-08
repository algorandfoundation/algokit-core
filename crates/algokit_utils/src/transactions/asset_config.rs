use algokit_transact::{
    Address, AssetConfigTransactionBuilder, AssetConfigTransactionFields, Transaction,
    TransactionHeader,
};

use crate::create_transaction_params;

create_transaction_params! {
    /// Parameters for creating an asset create transaction.
    #[derive(Clone, Default)]
    pub struct AssetCreateParams {
        /// The total amount of the smallest divisible (decimal) unit to create.
        ///
        /// For example, if creating a asset with 2 decimals and wanting a total supply of 100 units, this value should be 10000.
        ///
        /// This field can only be specified upon asset creation.
        pub total: u64,

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
        pub decimals: Option<u32>,

        /// Whether the asset is frozen by default for all accounts.
        /// Defaults to `false`.
        ///
        /// If `true` then for anyone apart from the creator to hold the
        /// asset it needs to be unfrozen per account using an asset freeze
        /// transaction from the `freeze` account, which must be set on creation.
        ///
        /// This field can only be specified upon asset creation.
        pub default_frozen: Option<bool>,

        /// The optional name of the asset.
        ///
        /// Max size is 32 bytes.
        ///
        /// This field can only be specified upon asset creation.
        pub asset_name: Option<String>,

        /// The optional name of the unit of this asset (e.g. ticker name).
        ///
        /// Max size is 8 bytes.
        ///
        /// This field can only be specified upon asset creation.
        pub unit_name: Option<String>,

        /// Specifies an optional URL where more information about the asset can be retrieved (e.g. metadata).
        ///
        /// Max size is 96 bytes.
        ///
        /// This field can only be specified upon asset creation.
        pub url: Option<String>,

        /// 32-byte hash of some metadata that is relevant to your asset and/or asset holders.
        ///
        /// The format of this metadata is up to the application.
        ///
        /// This field can only be specified upon asset creation.
        pub metadata_hash: Option<[u8; 32]>,

        /// The address of the optional account that can manage the configuration of the asset and destroy it.
        ///
        /// The fields it can change are `manager`, `reserve`, `clawback`, and `freeze`.
        ///
        /// If not set or set to the Zero address, the asset becomes permanently immutable.
        pub manager: Option<Address>,

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
        /// If not set or set to the Zero address, this field is permanently empty.
        pub reserve: Option<Address>,

        /// The address of the optional account that can be used to freeze or unfreeze holdings of this asset for any account.
        ///
        /// If empty, freezing is not permitted.
        ///
        /// If not set or set to the Zero address, this field is permanently empty.
        pub freeze: Option<Address>,

        /// The address of the optional account that can clawback holdings of this asset from any account.
        ///
        /// **This field should be used with caution** as the clawback account has the ability to **unconditionally take assets from any account**.
        ///
        /// If empty, clawback is not permitted.
        ///
        /// If not set or set to the Zero address is permanently empty.
        pub clawback: Option<Address>,
    }
}

create_transaction_params! {
    /// Parameters for creating an asset reconfiguration transaction.
    ///
    /// Only fields manager, reserve, freeze, and clawback can be set.
    ///
    /// **Note:** The manager, reserve, freeze, and clawback addresses
    /// are immutably empty if they are not set. If manager is not set then
    /// all fields are immutable from that point forward.
    #[derive(Clone, Default)]
    pub struct AssetConfigParams {
    /// ID of the asset to reconfigure
    pub asset_id: u64,

    /// The address of the optional account that can manage the configuration of the asset and destroy it.
    ///
    /// The configuration fields it can change are `manager`, `reserve`, `clawback`, and `freeze`.
    ///
    /// If not set or set to the Zero address the asset becomes permanently immutable.
    pub manager: Option<Address>,

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
    pub reserve: Option<Address>,

    /// The address of the optional account that can be used to freeze or unfreeze holdings of this asset for any account.
    ///
    /// If empty, freezing is not permitted.
    ///
    /// If not set or set to the Zero address is permanently empty.
    pub freeze: Option<Address>,

    /// The address of the optional account that can clawback holdings of this asset from any account.
    ///
    /// **This field should be used with caution** as the clawback account has the ability to **unconditionally take assets from any account**.
    ///
    /// If empty, clawback is not permitted.
    ///
    /// If not set or set to the Zero address is permanently empty.
    pub clawback: Option<Address>,
    }
}

create_transaction_params! {
    /// Parameters for creating an asset destroy transaction.
    ///
    /// Created assets can be destroyed only by the asset manager account. All of the assets must be owned by the creator of the asset before the asset can be deleted.
    #[derive(Clone, Default)]
    pub struct AssetDestroyParams {
        /// ID of the asset to destroy
        pub asset_id: u64,
    }
}

pub fn build_asset_create(
    params: &AssetCreateParams,
    header: TransactionHeader,
) -> Result<Transaction, String> {
    let mut builder = AssetConfigTransactionBuilder::default();
    builder.header(header).asset_id(0).total(params.total);

    if let Some(decimals) = params.decimals {
        builder.decimals(decimals);
    }

    if let Some(default_frozen) = params.default_frozen {
        builder.default_frozen(default_frozen);
    }

    if let Some(ref asset_name) = params.asset_name {
        builder.asset_name(asset_name.clone());
    }

    if let Some(ref unit_name) = params.unit_name {
        builder.unit_name(unit_name.clone());
    }

    if let Some(ref url) = params.url {
        builder.url(url.clone());
    }

    if let Some(metadata_hash) = params.metadata_hash {
        builder.metadata_hash(metadata_hash);
    }

    if let Some(ref manager) = params.manager {
        builder.manager(manager.clone());
    }

    if let Some(ref reserve) = params.reserve {
        builder.reserve(reserve.clone());
    }

    if let Some(ref freeze) = params.freeze {
        builder.freeze(freeze.clone());
    }

    if let Some(ref clawback) = params.clawback {
        builder.clawback(clawback.clone());
    }

    builder.build().map_err(|e| e.to_string())
}

pub fn build_asset_config(params: &AssetConfigParams, header: TransactionHeader) -> Transaction {
    Transaction::AssetConfig(AssetConfigTransactionFields {
        header,
        asset_id: params.asset_id,
        total: None,
        decimals: None,
        default_frozen: None,
        asset_name: None,
        unit_name: None,
        url: None,
        metadata_hash: None,
        manager: params.manager.clone(),
        reserve: params.reserve.clone(),
        freeze: params.freeze.clone(),
        clawback: params.clawback.clone(),
    })
}

pub fn build_asset_destroy(params: &AssetDestroyParams, header: TransactionHeader) -> Transaction {
    Transaction::AssetConfig(AssetConfigTransactionFields {
        header,
        asset_id: params.asset_id,
        total: None,
        decimals: None,
        default_frozen: None,
        asset_name: None,
        unit_name: None,
        url: None,
        metadata_hash: None,
        manager: None,
        reserve: None,
        freeze: None,
        clawback: None,
    })
}
