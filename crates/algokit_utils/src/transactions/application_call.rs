use super::common::CommonParams;
use algokit_transact::{Address, BoxReference, OnApplicationComplete, StateSchema};

/// Parameters for application call transactions.
#[derive(Debug, Default, Clone)]
pub struct ApplicationCallParams {
    pub common_params: CommonParams,
    /// ID of the application being called.
    ///
    /// Set this to 0 to indicate an application creation call.
    pub app_id: u64,
    /// Defines what additional actions occur with the transaction.
    pub on_complete: OnApplicationComplete,
    /// Transaction specific arguments available in the application's
    /// approval program and clear state program.
    pub args: Option<Vec<Vec<u8>>>,
    /// List of accounts in addition to the sender that may be accessed
    /// from the application's approval program and clear state program.
    pub account_references: Option<Vec<Address>>,
    /// List of applications in addition to the current application that may be called
    /// from the application's approval program and clear state program.
    pub app_references: Option<Vec<u64>>,
    /// Lists the assets whose parameters may be accessed by this application's
    /// approval program and clear state program.
    ///
    /// The access is read-only.
    pub asset_references: Option<Vec<u64>>,
    /// The boxes that should be made available for the runtime of the program.
    pub box_references: Option<Vec<BoxReference>>,
}

/// Parameters for application create transactions.
#[derive(Debug, Default, Clone)]
pub struct ApplicationCreateParams {
    pub common_params: CommonParams,
    /// Defines what additional actions occur with the transaction.
    pub on_complete: OnApplicationComplete,
    /// Logic executed for every application call transaction, except when
    /// on-completion is set to "clear".
    ///
    /// Approval programs may reject the transaction.
    /// Only required for application creation and update transactions.
    pub approval_program: Vec<u8>,
    /// Logic executed for application call transactions with on-completion set to "clear".
    ///
    /// Clear state programs cannot reject the transaction.
    /// Only required for application creation and update transactions.
    pub clear_state_program: Vec<u8>,
    /// Holds the maximum number of global state values.
    ///
    /// Only required for application creation transactions.
    /// This cannot be changed after creation.
    pub global_state_schema: Option<StateSchema>,
    /// Holds the maximum number of local state values.
    ///
    /// Only required for application creation transactions.
    /// This cannot be changed after creation.
    pub local_state_schema: Option<StateSchema>,
    /// Number of additional pages allocated to the application's approval
    /// and clear state programs.
    ///
    /// Each extra program page is 2048 bytes. The sum of approval program
    /// and clear state program may not exceed 2048*(1+extra_program_pages) bytes.
    /// Currently, the maximum value is 3.
    /// This cannot be changed after creation.
    pub extra_program_pages: Option<u64>,
    /// Transaction specific arguments available in the application's
    /// approval program and clear state program.
    pub args: Option<Vec<Vec<u8>>>,
    /// List of accounts in addition to the sender that may be accessed
    /// from the application's approval program and clear state program.
    pub account_references: Option<Vec<Address>>,
    /// List of applications in addition to the current application that may be called
    /// from the application's approval program and clear state program.
    pub app_references: Option<Vec<u64>>,
    /// Lists the assets whose parameters may be accessed by this application's
    /// approval program and clear state program.
    ///
    /// The access is read-only.
    pub asset_references: Option<Vec<u64>>,
    /// The boxes that should be made available for the runtime of the program.
    pub box_references: Option<Vec<BoxReference>>,
}

/// Parameters for application delete transactions.
#[derive(Debug, Default, Clone)]
pub struct ApplicationDeleteParams {
    pub common_params: CommonParams,
    /// ID of the application being deleted.
    pub app_id: u64,
    /// Transaction specific arguments available in the application's
    /// approval program and clear state program.
    pub args: Option<Vec<Vec<u8>>>,
    /// List of accounts in addition to the sender that may be accessed
    /// from the application's approval program and clear state program.
    pub account_references: Option<Vec<Address>>,
    /// List of applications in addition to the current application that may be called
    /// from the application's approval program and clear state program.
    pub app_references: Option<Vec<u64>>,
    /// Lists the assets whose parameters may be accessed by this application's
    /// approval program and clear state program.
    ///
    /// The access is read-only.
    pub asset_references: Option<Vec<u64>>,
    /// The boxes that should be made available for the runtime of the program.
    pub box_references: Option<Vec<BoxReference>>,
}

/// Parameters for application update transactions.
#[derive(Debug, Default, Clone)]
pub struct ApplicationUpdateParams {
    pub common_params: CommonParams,
    /// ID of the application being updated.
    pub app_id: u64,
    /// Logic executed for every application call transaction, except when
    /// on-completion is set to "clear".
    ///
    /// Approval programs may reject the transaction.
    /// Only required for application creation and update transactions.
    pub approval_program: Vec<u8>,
    /// Logic executed for application call transactions with on-completion set to "clear".
    ///
    /// Clear state programs cannot reject the transaction.
    /// Only required for application creation and update transactions.
    pub clear_state_program: Vec<u8>,
    /// Transaction specific arguments available in the application's
    /// approval program and clear state program.
    pub args: Option<Vec<Vec<u8>>>,
    /// List of accounts in addition to the sender that may be accessed
    /// from the application's approval program and clear state program.
    pub account_references: Option<Vec<Address>>,
    /// List of applications in addition to the current application that may be called
    /// from the application's approval program and clear state program.
    pub app_references: Option<Vec<u64>>,
    /// Lists the assets whose parameters may be accessed by this application's
    /// approval program and clear state program.
    ///
    /// The access is read-only.
    pub asset_references: Option<Vec<u64>>,
    /// The boxes that should be made available for the runtime of the program.
    pub box_references: Option<Vec<BoxReference>>,
}
