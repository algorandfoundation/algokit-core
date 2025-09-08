use crate::create_transaction_params;
use algokit_transact::{Address, AssetTransferTransactionFields, Transaction, TransactionHeader};

create_transaction_params! {
    /// Parameters for creating an asset transfer transaction.
    #[derive(Clone, Default)]
    pub struct AssetTransferParams {
        /// ID of the asset to transfer.
        pub asset_id: u64,
        /// The amount of the asset to transfer (in smallest divisible (decimal) units).
        pub amount: u64,
        /// The address of the account that will receive the asset unit(s).
        pub receiver: Address,
    }
}

create_transaction_params! {
    /// Parameters for creating an asset opt-in transaction.
    #[derive(Clone, Default)]
    pub struct AssetOptInParams {
        /// ID of the asset that will be opted-in to.
        pub asset_id: u64,
    }
}

create_transaction_params! {
    /// Parameters for creating an asset opt-out transaction.
    #[derive(Clone, Default)]
    pub struct AssetOptOutParams {
        /// ID of the asset that will be opted-out of.
        pub asset_id: u64,
        /// Optional address of an account to close the remaining asset position to. We recommend setting this to the asset creator.
        ///
        /// **Warning:** Be careful with this parameter as it can lead to loss of funds if not used correctly.
        pub close_remainder_to: Option<Address>,
    }
}

create_transaction_params! {
    #[derive(Clone, Default)]
    /// Parameters for creating an asset clawback transaction.
    pub struct AssetClawbackParams {
        /// ID of the asset to clawback.
        pub asset_id: u64,
        /// Amount of the asset to transfer (in smallest divisible (decimal) units).
        pub amount: u64,
        /// The address of the account that will receive the asset unit(s).
        pub receiver: Address,
        /// Address of an account to clawback the asset from.
        ///
        /// Requires the sender to be the clawback account.
        ///
        /// **Warning:** Be careful with this parameter as it can lead to unexpected loss of funds if not used correctly.
        pub clawback_target: Address,
    }
}
pub fn build_asset_transfer(
    params: &AssetTransferParams,
    header: TransactionHeader,
) -> Transaction {
    Transaction::AssetTransfer(AssetTransferTransactionFields {
        header,
        asset_id: params.asset_id,
        amount: params.amount,
        receiver: params.receiver.clone(),
        asset_sender: None,
        close_remainder_to: None,
    })
}

pub fn build_asset_opt_in(params: &AssetOptInParams, header: TransactionHeader) -> Transaction {
    let sender = header.sender.clone();
    Transaction::AssetTransfer(AssetTransferTransactionFields {
        header,
        asset_id: params.asset_id,
        amount: 0,
        receiver: sender,
        asset_sender: None,
        close_remainder_to: None,
    })
}

pub fn build_asset_opt_out(params: &AssetOptOutParams, header: TransactionHeader) -> Transaction {
    let sender: Address = header.sender.clone();
    Transaction::AssetTransfer(AssetTransferTransactionFields {
        header,
        asset_id: params.asset_id,
        amount: 0,
        receiver: sender,
        asset_sender: None,
        close_remainder_to: params.close_remainder_to.clone(),
    })
}

pub fn build_asset_clawback(
    params: &AssetClawbackParams,
    header: TransactionHeader,
) -> Transaction {
    Transaction::AssetTransfer(AssetTransferTransactionFields {
        header,
        asset_id: params.asset_id,
        amount: params.amount,
        receiver: params.receiver.clone(),
        asset_sender: Some(params.clawback_target.clone()),
        close_remainder_to: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::{TransactionHeader, test_utils::AccountMother};

    #[test]
    fn test_asset_opt_out_with_optional_close_remainder_to() {
        // Use valid test addresses
        let sender = AccountMother::neil().address();
        let creator = AccountMother::neil().address();

        // Test with Some(creator) - explicit close_remainder_to
        let params_with_creator = AssetOptOutParams {
            sender: sender.clone(),
            asset_id: 123,
            close_remainder_to: Some(creator.clone()),
            ..Default::default()
        };

        let header = TransactionHeader {
            sender: sender.clone(),
            fee: Some(1000),
            first_valid: 1000,
            last_valid: 1100,
            genesis_hash: Some([0; 32]),
            genesis_id: Some("test".to_string()),
            lease: None,
            note: None,
            rekey_to: None,
            group: None,
        };

        let tx = build_asset_opt_out(&params_with_creator, header.clone());

        if let Transaction::AssetTransfer(fields) = tx {
            assert_eq!(fields.asset_id, 123);
            assert_eq!(fields.amount, 0);
            assert_eq!(fields.receiver, sender);
            assert_eq!(fields.close_remainder_to, Some(creator));
        } else {
            panic!("Expected AssetTransfer transaction");
        }

        // Test with None - should pass None through (resolution happens at TransactionSender level)
        let params_without_creator = AssetOptOutParams {
            sender: sender.clone(),
            asset_id: 456,
            close_remainder_to: None,
            ..Default::default()
        };

        let tx2 = build_asset_opt_out(&params_without_creator, header);

        if let Transaction::AssetTransfer(fields) = tx2 {
            assert_eq!(fields.asset_id, 456);
            assert_eq!(fields.amount, 0);
            assert_eq!(fields.receiver, sender);
            // When None is provided, build_asset_opt_out passes None through
            // The actual creator resolution happens at the TransactionSender level
            assert_eq!(fields.close_remainder_to, None);
        } else {
            panic!("Expected AssetTransfer transaction");
        }
    }
}
