use crate::*;

#[ffi_record]
pub struct PaymentTransactionFields {
    receiver: String,

    amount: u64,

    close_remainder_to: Option<String>,
}

impl From<algokit_transact::PaymentTransactionFields> for PaymentTransactionFields {
    fn from(tx: algokit_transact::PaymentTransactionFields) -> Self {
        Self {
            receiver: tx.receiver.as_str(),
            amount: tx.amount,
            close_remainder_to: tx.close_remainder_to.map(|addr| addr.as_str()),
        }
    }
}

impl TryFrom<Transaction> for algokit_transact::PaymentTransactionFields {
    type Error = AlgoKitTransactError;

    fn try_from(tx: Transaction) -> Result<Self, Self::Error> {
        if tx.transaction_type != TransactionType::Payment || tx.payment.is_none() {
            return Err(Self::Error::DecodingError {
                message: "Payment data missing".to_string(),
            });
        }

        let data = tx.clone().payment.unwrap();
        let header: algokit_transact::TransactionHeader = tx.try_into()?;

        Ok(Self {
            header,
            amount: data.amount,
            receiver: data.receiver.parse()?,
            close_remainder_to: data
                .close_remainder_to
                .map(|addr| addr.parse())
                .transpose()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::{TestDataMother, TransactionMother};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_encoded_payment_transaction_type() {
        let txn: Transaction = TransactionMother::simple_payment().build().unwrap().into();

        // Encode the transaction
        let encoded = encode_transaction(txn).unwrap();

        // Test the get_encoded_transaction_type function
        let tx_type = get_encoded_transaction_type(&encoded).unwrap();
        assert_eq!(tx_type, TransactionType::Payment);
    }

    #[test]
    fn test_payment_transaction_id_ffi() {
        let data = TestDataMother::simple_payment();
        let tx_ffi: Transaction = data.transaction.into();

        let actual_id = get_transaction_id(tx_ffi.clone()).unwrap();
        let actual_id_raw = get_transaction_id_raw(tx_ffi.clone()).unwrap();

        assert_eq!(actual_id, data.id);
        assert_eq!(actual_id_raw, data.id_raw);
    }
}
