pub mod applications;
pub mod clients;
pub mod testing;
pub mod transactions;

// Re-exports for clean UniFFI surface
pub use clients::{
    AlgoClientConfig, AlgoConfig, AlgorandClient, AlgorandNetwork, AlgorandService, AppManager,
    AppManagerError, AssetInformation, AssetManager, AssetManagerError, BulkAssetOptInOutResult,
    ClientManager, NetworkDetails, TokenHeader, genesis_id_is_localnet,
};
// Re-export ABI types for convenience
pub use algokit_abi::ABIReturn;
pub use testing::{
    AlgorandFixture, AlgorandTestContext, algorand_fixture, algorand_fixture_with_config,
};
pub use transactions::{
    AccountCloseParams, AppCallMethodCallParams, AppCallParams, AppCreateMethodCallParams,
    AppCreateParams, AppDeleteMethodCallParams, AppDeleteParams, AppMethodCallArg,
    AppUpdateMethodCallParams, AppUpdateParams, AssetClawbackParams, AssetCreateParams,
    AssetDestroyParams, AssetFreezeParams, AssetOptInParams, AssetOptOutParams,
    AssetReconfigureParams, AssetTransferParams, AssetUnfreezeParams, BuiltTransactions,
    CommonParams, Composer, ComposerError, ComposerTransaction, EmptySigner,
    NonParticipationKeyRegistrationParams, OfflineKeyRegistrationParams,
    OnlineKeyRegistrationParams, PaymentParams, ResourcePopulation, SendAppCallResult,
    SendAppCreateResult, SendAppUpdateResult, SendAssetCreateResult, SendParams,
    SendTransactionComposerResults, SendTransactionResult, TransactionCreator,
    TransactionResultError, TransactionSender, TransactionSenderError, TransactionSigner,
    TransactionSignerGetter, TransactionWithSigner,
};
