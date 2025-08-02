use algokit_transact::Address;

use super::common::CommonParams;

/// Parameters for creating a payment transaction
#[derive(Debug, Default, Clone)]
pub struct PaymentParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// The address of the account receiving the ALGO payment.
    pub receiver: Address,

    /// The amount of microALGO to send.
    ///
    /// Specified in microALGO (1 ALGO = 1,000,000 microALGO).
    pub amount: u64,
}

/// Parameters for creating an account close transaction.
#[derive(Debug, Default, Clone)]
pub struct AccountCloseParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// The address to receive the remaining funds.
    pub close_remainder_to: Address,
}
