use algokit_transact::{Address, AssetTransferTransactionFields, Transaction, TransactionHeader};

use super::common::CommonParams;

#[derive(Debug, Clone)]
pub struct AssetTransferParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset transfers
    /// without additional side effects.
    /// Only in the case where the receiver is equal to the sender and the amount is zero,
    /// this is an asset opt-in transaction.
    pub common_params: CommonParams,
    pub asset_id: u64,
    pub amount: u64,
    pub receiver: Address,
}

#[derive(Debug, Clone)]
pub struct AssetOptInParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset opt-in
    /// without additional side effects.
    pub common_params: CommonParams,
    pub asset_id: u64,
}

#[derive(Debug, Clone)]
pub struct AssetOptOutParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset opt-out
    /// without additional side effects.
    pub common_params: CommonParams,
    pub asset_id: u64,
    /// The address to close the remainder to. If None, defaults to the asset creator.
    pub close_remainder_to: Option<Address>,
}

#[derive(Debug, Clone)]
pub struct AssetClawbackParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset clawback
    /// without additional side effects.
    pub common_params: CommonParams,
    pub asset_id: u64,
    pub amount: u64,
    pub receiver: Address,
    // The address from which ASAs are taken.
    pub clawback_target: Address,
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
            common_params: CommonParams {
                sender: sender.clone(),
                ..Default::default()
            },
            asset_id: 123,
            close_remainder_to: Some(creator.clone()),
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
            common_params: CommonParams {
                sender: sender.clone(),
                ..Default::default()
            },
            asset_id: 456,
            close_remainder_to: None,
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
