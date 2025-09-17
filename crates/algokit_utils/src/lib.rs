pub mod applications;
pub mod clients;
pub mod config;
pub mod transactions;

// Re-exports for clean UniFFI surface
pub use clients::{
    AccountManager, AlgoClientConfig, AlgoConfig, AlgorandClient, AlgorandNetwork, AlgorandService,
    AppManager, AppManagerError, AssetInformation, AssetManager, AssetManagerError,
    BulkAssetOptInOutResult, ClientManager, NetworkDetails, TokenHeader, genesis_id_is_localnet,
};
// Re-export ABI types for convenience
pub use algokit_abi::ABIReturn;
pub use transactions::{
    AccountCloseParams, AppCallMethodCallParams, AppCallParams, AppCreateMethodCallParams,
    AppCreateParams, AppDeleteMethodCallParams, AppDeleteParams, AppMethodCallArg,
    AppUpdateMethodCallParams, AppUpdateParams, AssetClawbackParams, AssetConfigParams,
    AssetCreateParams, AssetDestroyParams, AssetFreezeParams, AssetOptInParams, AssetOptOutParams,
    AssetTransferParams, AssetUnfreezeParams, Composer, ComposerError, ComposerTransaction,
    EmptySigner, NonParticipationKeyRegistrationParams, OfflineKeyRegistrationParams,
    OnlineKeyRegistrationParams, PaymentParams, ResourcePopulation, SendAppCallResult,
    SendAppCreateResult, SendAppUpdateResult, SendAssetCreateResult, SendParams,
    SendTransactionComposerResults, SendTransactionResult, TransactionCreator,
    TransactionResultError, TransactionSender, TransactionSenderError, TransactionSigner,
    TransactionWithSigner,
};

pub use applications::app_client::{AppClient, AppClientError, AppClientParams, AppSourceMaps};
pub use config::{Config, EventType};
