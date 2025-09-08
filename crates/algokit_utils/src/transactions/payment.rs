use crate::create_transaction_params;
use algokit_transact::{Address, PaymentTransactionFields, Transaction, TransactionHeader};

create_transaction_params! {
    /// Parameters for creating a payment transaction
    #[derive(Clone, Default)]
    pub struct PaymentParams {
        /// The address of the account receiving the ALGO payment.
        pub receiver: Address,
        /// The amount of microALGO to send.
        ///
        /// Specified in microALGO (1 ALGO = 1,000,000 microALGO).
        pub amount: u64,
    }
}

create_transaction_params! {
    /// Parameters for creating an account close transaction.
    #[derive(Clone, Default)]
    pub struct AccountCloseParams {
        /// Close the sender account and send the remaining balance to this address
        ///
        /// *Warning:* Be careful this can lead to loss of funds if not used correctly.
        pub close_remainder_to: Address,
    }
}

pub fn build_payment(params: &PaymentParams, header: TransactionHeader) -> Transaction {
    Transaction::Payment(PaymentTransactionFields {
        header,
        receiver: params.receiver.clone(),
        amount: params.amount,
        close_remainder_to: None,
    })
}

pub fn build_account_close(params: &AccountCloseParams, header: TransactionHeader) -> Transaction {
    let sender = header.sender.clone();
    Transaction::Payment(PaymentTransactionFields {
        header,
        receiver: sender,
        amount: 0,
        close_remainder_to: Some(params.close_remainder_to.clone()),
    })
}
