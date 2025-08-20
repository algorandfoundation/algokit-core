use algod_client::apis::{AlgodClient, Error as AlgodError};
use algod_client::models::{AccountAssetInformation as AlgodAccountAssetInformation, Asset};
use algokit_transact::Address;
use snafu::Snafu;
use std::{str::FromStr, sync::Arc};

use crate::transactions::{
    AssetOptInParams, AssetOptOutParams, CommonParams, Composer, ComposerError,
    TransactionSignerGetter,
};

#[derive(Debug, Clone)]
pub struct BulkAssetOptInOutResult {
    pub asset_id: u64,
    pub transaction_id: String,
}

/// Information about an Algorand Standard Asset (ASA).
///
/// This type provides a flattened, developer-friendly interface to asset information
/// that aligns with TypeScript and Python implementations.
#[derive(Debug, Clone)]
pub struct AssetInformation {
    /// The ID of the asset.
    pub asset_id: u64,

    /// The address of the account that created the asset.
    ///
    /// This is the address where the parameters for this asset can be found,
    /// and also the address where unwanted asset units can be sent when
    /// closing out an asset position and opting-out of the asset.
    pub creator: String,

    /// The total amount of the smallest divisible (decimal) units that were created of the asset.
    ///
    /// For example, if `decimals` is, say, 2, then for every 100 `total` there is 1 whole unit.
    pub total: u64,

    /// The amount of decimal places the asset was created with.
    ///
    /// * If 0, the asset is not divisible;
    /// * If 1, the base unit of the asset is in tenths;
    /// * If 2, the base unit of the asset is in hundredths;
    /// * If 3, the base unit of the asset is in thousandths;
    /// * and so on up to 19 decimal places.
    pub decimals: u32,

    /// Whether the asset was frozen by default for all accounts.
    ///
    /// If `true` then for anyone apart from the creator to hold the
    /// asset it needs to be unfrozen per account using an asset freeze
    /// transaction from the `freeze` account.
    pub default_frozen: Option<bool>,

    /// The address of the optional account that can manage the configuration of the asset and destroy it.
    ///
    /// If not set the asset is permanently immutable.
    pub manager: Option<String>,

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
    /// If not set the field is permanently empty.
    pub reserve: Option<String>,

    /// The address of the optional account that can be used to freeze or unfreeze holdings of this asset for any account.
    ///
    /// If empty, freezing is not permitted.
    ///
    /// If not set the field is permanently empty.
    pub freeze: Option<String>,

    /// The address of the optional account that can clawback holdings of this asset from any account.
    ///
    /// The clawback account has the ability to **unconditionally take assets from any account**.
    ///
    /// If empty, clawback is not permitted.
    ///
    /// If not set the field is permanently empty.
    pub clawback: Option<String>,

    /// The optional name of the unit of this asset (e.g. ticker name).
    ///
    /// Max size is 8 bytes.
    pub unit_name: Option<String>,

    /// The optional name of the unit of this asset as bytes.
    ///
    /// Max size is 8 bytes.
    pub unit_name_b64: Option<Vec<u8>>,

    /// The optional name of the asset.
    ///
    /// Max size is 32 bytes.
    pub asset_name: Option<String>,

    /// The optional name of the asset as bytes.
    ///
    /// Max size is 32 bytes.
    pub asset_name_b64: Option<Vec<u8>>,

    /// Optional URL where more information about the asset can be retrieved (e.g. metadata).
    ///
    /// Max size is 96 bytes.
    pub url: Option<String>,

    /// Optional URL where more information about the asset can be retrieved as bytes.
    ///
    /// Max size is 96 bytes.
    pub url_b64: Option<Vec<u8>>,

    /// 32-byte hash of some metadata that is relevant to the asset and/or asset holders.
    ///
    /// The format of this metadata is up to the application.
    pub metadata_hash: Option<Vec<u8>>,
}

impl From<Asset> for AssetInformation {
    fn from(asset: Asset) -> Self {
        Self {
            asset_id: asset.index,
            creator: asset.params.creator,
            total: asset.params.total,
            decimals: asset.params.decimals as u32,
            default_frozen: asset.params.default_frozen,
            manager: asset.params.manager,
            reserve: asset.params.reserve,
            freeze: asset.params.freeze,
            clawback: asset.params.clawback,
            unit_name: asset.params.unit_name,
            unit_name_b64: asset.params.unit_name_b64,
            asset_name: asset.params.name,
            asset_name_b64: asset.params.name_b64,
            url: asset.params.url,
            url_b64: asset.params.url_b64,
            metadata_hash: asset.params.metadata_hash,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AssetValidationError {
    pub asset_id: u64,
    pub error: String,
}

/// Manages Algorand Standard Assets.
#[derive(Clone)]
pub struct AssetManager {
    algod_client: Arc<AlgodClient>,
}

impl AssetManager {
    pub fn new(algod_client: Arc<AlgodClient>) -> Self {
        Self { algod_client }
    }

    /// Get asset information by asset ID
    /// Returns a convenient, flattened view of the asset information.
    pub async fn get_by_id(&self, asset_id: u64) -> Result<AssetInformation, AssetManagerError> {
        let asset = self
            .algod_client
            .get_asset_by_id(asset_id)
            .await
            .map_err(|e| AssetManagerError::AlgodClientError { source: e })?;

        Ok(asset.into())
    }

    /// Get account's asset information.
    /// Returns the raw algod AccountAssetInformation type.
    /// Access asset holding via `account_info.asset_holding` and asset params via `account_info.asset_params`.
    pub async fn get_account_information(
        &self,
        sender: &Address,
        asset_id: u64,
    ) -> Result<AlgodAccountAssetInformation, AssetManagerError> {
        self.algod_client
            .account_asset_information(&sender.to_string(), asset_id, None)
            .await
            .map_err(|e| AssetManagerError::AlgodClientError { source: e })
    }

    pub async fn bulk_opt_in(
        &self,
        account: &Address,
        asset_ids: &[u64],
        signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<Vec<BulkAssetOptInOutResult>, AssetManagerError> {
        if asset_ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut composer = Composer::new(self.algod_client.clone(), signer_getter);

        // Add asset opt-in transactions for each asset
        for &asset_id in asset_ids {
            let opt_in_params = AssetOptInParams {
                common_params: CommonParams {
                    sender: account.clone(),
                    ..Default::default()
                },
                asset_id,
            };

            composer
                .add_asset_opt_in(opt_in_params)
                .map_err(|e| AssetManagerError::ComposerError { source: e })?;
        }

        // Send the transaction group
        let results = composer
            .send(Default::default())
            .await
            .map_err(|e| AssetManagerError::ComposerError { source: e })?;

        // Map transaction IDs back to assets
        let bulk_results: Vec<BulkAssetOptInOutResult> = asset_ids
            .iter()
            .zip(results.transaction_ids.iter())
            .map(|(&asset_id, transaction_id)| BulkAssetOptInOutResult {
                asset_id,
                transaction_id: transaction_id.clone(),
            })
            .collect();

        Ok(bulk_results)
    }

    pub async fn bulk_opt_out(
        &self,
        account: &Address,
        asset_ids: &[u64],
        ensure_zero_balance: Option<bool>,
        signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<Vec<BulkAssetOptInOutResult>, AssetManagerError> {
        if asset_ids.is_empty() {
            return Ok(Vec::new());
        }

        let should_check_balance = ensure_zero_balance.unwrap_or(false);

        // If we need to check balances, verify they are all zero
        if should_check_balance {
            for &asset_id in asset_ids {
                let account_info = self.get_account_information(account, asset_id).await?;
                let balance = account_info
                    .asset_holding
                    .as_ref()
                    .map(|h| h.amount)
                    .unwrap_or(0);
                if balance > 0 {
                    return Err(AssetManagerError::NonZeroBalance {
                        address: account.to_string(),
                        asset_id,
                        balance,
                    });
                }
            }
        }

        // Fetch asset information to get creators
        let mut asset_creators = Vec::new();
        for &asset_id in asset_ids {
            let asset_info = self.get_by_id(asset_id).await?;
            let creator = Address::from_str(&asset_info.creator)
                .map_err(|_| AssetManagerError::AssetNotFound { asset_id })?;
            asset_creators.push(creator);
        }

        let mut composer = Composer::new(self.algod_client.clone(), signer_getter);

        // Add asset opt-out transactions for each asset
        for (i, &asset_id) in asset_ids.iter().enumerate() {
            let opt_out_params = AssetOptOutParams {
                common_params: CommonParams {
                    sender: account.clone(),
                    ..Default::default()
                },
                asset_id,
                close_remainder_to: Some(asset_creators[i].clone()),
            };

            composer
                .add_asset_opt_out(opt_out_params)
                .map_err(|e| AssetManagerError::ComposerError { source: e })?;
        }

        // Send the transaction group
        let results = composer
            .send(Default::default())
            .await
            .map_err(|e| AssetManagerError::ComposerError { source: e })?;

        // Map transaction IDs back to assets
        let bulk_results: Vec<BulkAssetOptInOutResult> = asset_ids
            .iter()
            .zip(results.transaction_ids.iter())
            .map(|(&asset_id, transaction_id)| BulkAssetOptInOutResult {
                asset_id,
                transaction_id: transaction_id.clone(),
            })
            .collect();

        Ok(bulk_results)
    }
}

#[derive(Debug, Snafu)]
pub enum AssetManagerError {
    #[snafu(display("Algod client error: {source}"))]
    AlgodClientError { source: AlgodError },

    #[snafu(display("Composer error: {source}"))]
    ComposerError { source: ComposerError },

    #[snafu(display("Asset not found: {asset_id}"))]
    AssetNotFound { asset_id: u64 },

    #[snafu(display("Account not found: {address}"))]
    AccountNotFound { address: String },

    #[snafu(display("Account {address} is not opted into asset {asset_id}"))]
    NotOptedIn { address: String, asset_id: u64 },

    #[snafu(display("Account {address} has non-zero balance {balance} for asset {asset_id}"))]
    NonZeroBalance {
        address: String,
        asset_id: u64,
        balance: u64,
    },

    #[snafu(display("Asset {asset_id} is frozen for account {address}"))]
    AssetFrozen { address: String, asset_id: u64 },

    #[snafu(display("Method '{method}' not implemented: {reason}"))]
    NotImplemented { method: String, reason: String },
}
