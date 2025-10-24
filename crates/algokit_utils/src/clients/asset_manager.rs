use algod_client::apis::{
    AlgodApiError, AlgodClient, Error as AlgodError,
    account_asset_information::AccountAssetInformationError, get_asset_by_id::GetAssetByIdError,
};
use algod_client::models::{AccountAssetInformation as AlgodAccountAssetInformation, Asset};
use algokit_http_client::HttpError;
use algokit_transact::{Address, constants::MAX_TX_GROUP_SIZE};
use snafu::Snafu;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    sync::Arc,
};

use crate::transactions::{
    AssetOptInParams, AssetOptOutParams, ComposerError, TransactionComposer,
    TransactionComposerConfig,
};

#[derive(Debug, Clone)]
pub struct BulkAssetOptInOutResult {
    pub asset_id: u64,
    pub transaction_id: String,
}

/// Information about an Algorand Standard Asset (ASA).
///
/// This type provides a flattened, developer-friendly interface to asset information.
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
    pub unit_name_bytes: Option<Vec<u8>>,

    /// The optional name of the asset.
    ///
    /// Max size is 32 bytes.
    pub asset_name: Option<String>,

    /// The optional name of the asset as bytes.
    ///
    /// Max size is 32 bytes.
    pub asset_name_bytes: Option<Vec<u8>>,

    /// Optional URL where more information about the asset can be retrieved (e.g. metadata).
    ///
    /// Max size is 96 bytes.
    pub url: Option<String>,

    /// Optional URL where more information about the asset can be retrieved as bytes.
    ///
    /// Max size is 96 bytes.
    pub url_bytes: Option<Vec<u8>>,

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
            unit_name_bytes: asset.params.unit_name_b64,
            asset_name: asset.params.name,
            asset_name_bytes: asset.params.name_b64,
            url: asset.params.url,
            url_bytes: asset.params.url_b64,
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
    new_composer: Arc<dyn Fn(Option<TransactionComposerConfig>) -> TransactionComposer>,
}

impl AssetManager {
    pub fn new(
        algod_client: Arc<AlgodClient>,
        new_composer: impl Fn(Option<TransactionComposerConfig>) -> TransactionComposer + 'static,
    ) -> Self {
        Self {
            algod_client,
            new_composer: Arc::new(new_composer),
        }
    }

    /// Get asset information by asset ID
    /// Returns a convenient, flattened view of the asset information.
    pub async fn get_by_id(&self, asset_id: u64) -> Result<AssetInformation, AssetManagerError> {
        let asset = self
            .algod_client
            .get_asset_by_id(asset_id)
            .await
            .map_err(|error| {
                map_get_asset_by_id_error(&error, asset_id)
                    .unwrap_or(AssetManagerError::AlgodClientError { source: error })
            })?;

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
        let sender_str = sender.to_string();
        self.algod_client
            .account_asset_information(sender_str.as_str(), asset_id)
            .await
            .map_err(|error| {
                map_account_asset_information_error(&error, sender_str.as_str(), asset_id)
                    .unwrap_or(AssetManagerError::AlgodClientError { source: error })
            })
    }

    pub async fn bulk_opt_in(
        &self,
        account: &Address,
        asset_ids: &[u64],
    ) -> Result<Vec<BulkAssetOptInOutResult>, AssetManagerError> {
        if asset_ids.is_empty() {
            return Ok(Vec::new());
        }

        // Ignore duplicate asset IDs while preserving input order
        let mut seen: HashSet<u64> = HashSet::with_capacity(asset_ids.len());
        let unique_ids: Vec<u64> = asset_ids
            .iter()
            .copied()
            .filter(|id| seen.insert(*id))
            .collect();

        let mut bulk_results = Vec::with_capacity(unique_ids.len());

        for asset_chunk in unique_ids.chunks(MAX_TX_GROUP_SIZE) {
            let mut composer = (self.new_composer)(None);

            for &asset_id in asset_chunk {
                let opt_in_params = AssetOptInParams {
                    sender: account.clone(),
                    asset_id,
                    ..Default::default()
                };

                composer
                    .add_asset_opt_in(opt_in_params)
                    .map_err(|e| AssetManagerError::ComposerError { source: e })?;
            }

            let composer_result = composer
                .send(Default::default())
                .await
                .map_err(|e| AssetManagerError::ComposerError { source: e })?;

            bulk_results.extend(asset_chunk.iter().zip(composer_result.results.iter()).map(
                |(&asset_id, result)| BulkAssetOptInOutResult {
                    asset_id,
                    transaction_id: result.transaction_id.clone(),
                },
            ));
        }

        Ok(bulk_results)
    }

    pub async fn bulk_opt_out(
        &self,
        account: &Address,
        asset_ids: &[u64],
        ensure_zero_balance: Option<bool>,
    ) -> Result<Vec<BulkAssetOptInOutResult>, AssetManagerError> {
        if asset_ids.is_empty() {
            return Ok(Vec::new());
        }

        // Ignore duplicate asset IDs while preserving input order
        let mut seen: HashSet<u64> = HashSet::with_capacity(asset_ids.len());
        let unique_ids: Vec<u64> = asset_ids
            .iter()
            .copied()
            .filter(|id| seen.insert(*id))
            .collect();

        let should_check_balance = ensure_zero_balance.unwrap_or(false);

        // If we need to check balances, verify they are all zero
        if should_check_balance {
            for &asset_id in unique_ids.iter() {
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
        let mut asset_creators = HashMap::with_capacity(unique_ids.len());
        for &asset_id in unique_ids.iter() {
            let asset_info = self.get_by_id(asset_id).await?;
            let creator = Address::from_str(&asset_info.creator)
                .map_err(|_| AssetManagerError::AssetNotFound { asset_id })?;
            asset_creators.insert(asset_id, creator);
        }

        let asset_creator_pairs: Vec<(u64, Address)> = unique_ids
            .iter()
            .map(|&asset_id| {
                let creator = asset_creators
                    .remove(&asset_id)
                    .expect("Creator information should be available for all asset IDs");
                (asset_id, creator)
            })
            .collect();

        let mut bulk_results = Vec::with_capacity(asset_creator_pairs.len());

        for asset_chunk in asset_creator_pairs.chunks(MAX_TX_GROUP_SIZE) {
            let mut composer = (self.new_composer)(None);

            for (asset_id, creator) in asset_chunk.iter() {
                let opt_out_params = AssetOptOutParams {
                    sender: account.clone(),
                    asset_id: *asset_id,
                    close_remainder_to: Some(creator.clone()),
                    ..Default::default()
                };

                composer
                    .add_asset_opt_out(opt_out_params)
                    .map_err(|e| AssetManagerError::ComposerError { source: e })?;
            }

            let composer_result = composer
                .send(Default::default())
                .await
                .map_err(|e| AssetManagerError::ComposerError { source: e })?;

            bulk_results.extend(asset_chunk.iter().zip(composer_result.results.iter()).map(
                |((asset_id, _), result)| BulkAssetOptInOutResult {
                    asset_id: *asset_id,
                    transaction_id: result.transaction_id.clone(),
                },
            ));
        }

        Ok(bulk_results)
    }
}

fn map_get_asset_by_id_error(error: &AlgodError, asset_id: u64) -> Option<AssetManagerError> {
    match error {
        AlgodError::Api {
            source:
                AlgodApiError::GetAssetById {
                    error: GetAssetByIdError::Status404(_),
                },
        } => Some(AssetManagerError::AssetNotFound { asset_id }),
        AlgodError::Api { .. } => None,
        AlgodError::Http { source } => {
            // Prefer structured status when available, fallback to message matching for older clients
            match source {
                HttpError::StatusError { status, .. } if *status == 404 => {
                    Some(AssetManagerError::AssetNotFound { asset_id })
                }
                _ => http_error_message(source).and_then(|message| {
                    if message.contains("status 404") {
                        Some(AssetManagerError::AssetNotFound { asset_id })
                    } else {
                        None
                    }
                }),
            }
        }
        _ => None,
    }
}

fn map_account_asset_information_error(
    error: &AlgodError,
    address: &str,
    asset_id: u64,
) -> Option<AssetManagerError> {
    match error {
        AlgodError::Api {
            source:
                AlgodApiError::AccountAssetInformation {
                    error: AccountAssetInformationError::Status400(_),
                },
        } => Some(AssetManagerError::AccountNotFound {
            address: address.to_string(),
        }),
        AlgodError::Api { .. } => None,
        AlgodError::Http { source } => {
            // Prefer structured status when available, fallback to message matching for older clients
            match source {
                HttpError::StatusError { status, .. } if *status == 404 => {
                    Some(AssetManagerError::NotOptedIn {
                        address: address.to_string(),
                        asset_id,
                    })
                }
                HttpError::StatusError { status, .. } if *status == 400 => {
                    Some(AssetManagerError::AccountNotFound {
                        address: address.to_string(),
                    })
                }
                _ => http_error_message(source).and_then(|message| {
                    if message.contains("status 404") {
                        Some(AssetManagerError::NotOptedIn {
                            address: address.to_string(),
                            asset_id,
                        })
                    } else if message.contains("status 400")
                        || message.to_ascii_lowercase().contains("account not found")
                    {
                        Some(AssetManagerError::AccountNotFound {
                            address: address.to_string(),
                        })
                    } else {
                        None
                    }
                }),
            }
        }
        _ => None,
    }
}

fn http_error_message(error: &HttpError) -> Option<&str> {
    match error {
        HttpError::RequestError { message } => Some(message.as_str()),
        HttpError::StatusError { message, .. } => Some(message.as_str()),
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
