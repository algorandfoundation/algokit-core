//! Payment transaction module for AlgoKit Core.
//!
//! This module provides functionality for creating and managing payment transactions,
//! which are used to transfer ALGO between accounts.

use crate::transactions::common::TransactionHeader;
use crate::utils::{is_zero, is_zero_addr, is_zero_addr_opt};
use crate::{Address, Transaction};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

/// Represents a payment transaction that transfers ALGO between accounts.
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
#[builder(
    name = "PaymentTransactionBuilder",
    setter(strip_option),
    build_fn(name = "build_fields")
)]
pub struct PaymentTransactionFields {
    /// Common transaction header fields.
    #[serde(flatten)]
    pub header: TransactionHeader,

    /// The address of the account receiving the ALGO payment.
    #[serde(rename = "rcv")]
    #[serde(skip_serializing_if = "is_zero_addr")]
    #[serde(default)]
    pub receiver: Address,

    /// The amount of microALGO to send.
    ///
    /// Specified in microALGO (1 ALGO = 1,000,000 microALGO).
    #[serde(rename = "amt")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub amount: u64,

    /// Optional address to send all remaining funds to after the transfer.
    ///
    /// If specified, this indicates that the sender account should be closed after the transaction,
    /// and all remaining funds (minus fees) should be transferred to the specified address.
    /// This effectively removes the sender account from the ledger.
    #[serde(rename = "close")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    #[serde(default)]
    #[builder(default)]
    pub close_remainder_to: Option<Address>,
}

impl PaymentTransactionBuilder {
    pub fn build(&self) -> Result<Transaction, PaymentTransactionBuilderError> {
        self.build_fields().map(Transaction::Payment)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        FeeParams, Transaction,
        test_utils::{
            AccountMother, TestDataMother, TransactionMother,
            check_multisigned_transaction_encoding, check_signed_transaction_encoding,
            check_transaction_encoding, check_transaction_id,
        },
    };

    #[test]
    fn test_simple_payment_snapshot() {
        let data = TestDataMother::simple_payment();
        assert_eq!(
            data.id,
            String::from("TZM3P4ZL4DLIEZ3WOEP67MQ6JITTO4D3NJN3RCA5YDBC3V4LA5LA")
        );
        assert_eq!(
            data.multisig_signed_bytes,
            [
                130, 164, 109, 115, 105, 103, 131, 166, 115, 117, 98, 115, 105, 103, 146, 130, 162,
                112, 107, 196, 32, 230, 185, 154, 253, 65, 13, 19, 221, 14, 138, 126, 148, 184,
                121, 29, 48, 92, 117, 6, 238, 183, 225, 250, 65, 14, 118, 26, 59, 98, 44, 225, 20,
                161, 115, 196, 64, 198, 56, 196, 15, 176, 92, 85, 96, 205, 178, 248, 28, 27, 215,
                149, 74, 22, 18, 122, 228, 98, 34, 13, 202, 109, 58, 242, 134, 31, 206, 195, 29,
                110, 250, 219, 67, 240, 62, 47, 253, 200, 132, 24, 36, 210, 17, 97, 97, 165, 32,
                154, 49, 102, 252, 16, 157, 51, 135, 216, 86, 41, 198, 47, 15, 130, 162, 112, 107,
                196, 32, 110, 60, 51, 144, 133, 183, 247, 29, 54, 150, 199, 125, 170, 105, 173, 84,
                148, 21, 87, 24, 235, 157, 2, 172, 123, 102, 144, 142, 198, 141, 84, 114, 161, 115,
                196, 64, 198, 56, 196, 15, 176, 92, 85, 96, 205, 178, 248, 28, 27, 215, 149, 74,
                22, 18, 122, 228, 98, 34, 13, 202, 109, 58, 242, 134, 31, 206, 195, 29, 110, 250,
                219, 67, 240, 62, 47, 253, 200, 132, 24, 36, 210, 17, 97, 97, 165, 32, 154, 49,
                102, 252, 16, 157, 51, 135, 216, 86, 41, 198, 47, 15, 163, 116, 104, 114, 2, 161,
                118, 1, 163, 116, 120, 110, 137, 163, 97, 109, 116, 206, 0, 1, 138, 136, 163, 102,
                101, 101, 205, 3, 232, 162, 102, 118, 206, 3, 5, 0, 212, 163, 103, 101, 110, 172,
                116, 101, 115, 116, 110, 101, 116, 45, 118, 49, 46, 48, 162, 103, 104, 196, 32, 72,
                99, 181, 24, 164, 179, 200, 78, 200, 16, 242, 45, 79, 16, 129, 203, 15, 113, 240,
                89, 167, 172, 32, 222, 198, 47, 127, 112, 229, 9, 58, 34, 162, 108, 118, 206, 3, 5,
                4, 188, 163, 114, 99, 118, 196, 32, 173, 207, 218, 63, 201, 93, 52, 35, 35, 15,
                161, 115, 204, 245, 211, 90, 68, 182, 3, 164, 184, 247, 131, 205, 149, 104, 201,
                215, 253, 11, 206, 245, 163, 115, 110, 100, 196, 32, 138, 24, 8, 153, 89, 167, 60,
                236, 255, 238, 91, 198, 115, 190, 137, 254, 3, 35, 198, 98, 195, 33, 65, 123, 138,
                200, 132, 194, 74, 0, 44, 25, 164, 116, 121, 112, 101, 163, 112, 97, 121
            ]
        )
    }

    #[test]
    fn test_min_fee() {
        let txn: Transaction = TransactionMother::simple_payment().build().unwrap();

        let updated_transaction = txn
            .assign_fee(FeeParams {
                fee_per_byte: 0,
                min_fee: 1000,
                extra_fee: None,
                max_fee: None,
            })
            .unwrap();
        assert_eq!(updated_transaction.header().fee, Some(1000));
    }

    #[test]
    fn test_extra_fee() {
        let txn: Transaction = TransactionMother::simple_payment().build().unwrap();

        let updated_transaction = txn
            .assign_fee(FeeParams {
                fee_per_byte: 1,
                min_fee: 1000,
                extra_fee: Some(500),
                max_fee: None,
            })
            .unwrap();
        assert_eq!(updated_transaction.header().fee, Some(1500));
    }

    #[test]
    fn test_max_fee() {
        let txn: Transaction = TransactionMother::simple_payment().build().unwrap();

        let result = txn.assign_fee(FeeParams {
            fee_per_byte: 10,
            min_fee: 500,
            extra_fee: None,
            max_fee: Some(1000),
        });

        assert!(result.is_err());
        let err: crate::AlgoKitTransactError = result.unwrap_err();
        let msg = format!("{}", err);
        assert!(
            msg == "Transaction fee 2470 µALGO is greater than max fee 1000 µALGO",
            "Unexpected error message: {}",
            msg
        );
    }

    #[test]
    fn test_calculate_fee() {
        let txn: Transaction = TransactionMother::simple_payment().build().unwrap();

        let updated_transaction = txn
            .assign_fee(FeeParams {
                fee_per_byte: 5,
                min_fee: 1000,
                extra_fee: None,
                max_fee: None,
            })
            .unwrap();

        assert_eq!(updated_transaction.header().fee, Some(1235));
    }

    #[test]
    fn test_payment_transaction_encoding() {
        let payment_tx = TransactionMother::observed_payment().build().unwrap();

        check_transaction_id(
            &payment_tx,
            "VTADY3NGJGE4DVZ4CKLX43NTEE3C2J4JJANZ5TPBR4OYJ2D4F2CA",
        );
        check_transaction_encoding(&payment_tx, 212);
        check_multisigned_transaction_encoding(&payment_tx, 449);
    }

    #[test]
    fn test_payment_signed_transaction_encoding() {
        let payment_tx = TransactionMother::simple_payment().build().unwrap();
        check_signed_transaction_encoding(&payment_tx, 247, None);
        check_signed_transaction_encoding(&payment_tx, 286, Some(AccountMother::account().clone()));
    }
}
