//! Represents a key registration transaction.
//!
//! This module provides FFI-compatible structures and conversions for key registration
//! transactions that can be used across language bindings.
use crate::*;

#[ffi_record]
pub struct KeyRegistrationTransactionFields {
    /// Root participation public key (32 bytes)
    pub vote_key: Option<Vec<u8>>,

    /// VRF public key (32 bytes)
    pub selection_key: Option<Vec<u8>>,

    /// State proof key (64 bytes)
    pub state_proof_key: Option<Vec<u8>>,

    /// First round for which the participation key is valid
    pub vote_first: Option<u64>,

    /// Last round for which the participation key is valid
    pub vote_last: Option<u64>,

    /// Key dilution for the 2-level participation key
    pub vote_key_dilution: Option<u64>,

    /// Mark account as non-reward earning
    pub non_participation: Option<bool>,
}

impl From<algokit_transact::KeyRegistrationTransactionFields> for KeyRegistrationTransactionFields {
    fn from(tx: algokit_transact::KeyRegistrationTransactionFields) -> Self {
        Self {
            vote_key: tx.vote_key.map(Into::into),
            selection_key: tx.selection_key.map(Into::into),
            state_proof_key: tx.state_proof_key.map(Into::into),
            vote_first: tx.vote_first,
            vote_last: tx.vote_last,
            vote_key_dilution: tx.vote_key_dilution,
            non_participation: tx.non_participation,
        }
    }
}

impl TryFrom<crate::Transaction> for algokit_transact::KeyRegistrationTransactionFields {
    type Error = AlgoKitTransactError;

    fn try_from(tx: crate::Transaction) -> Result<Self, Self::Error> {
        if tx.transaction_type != crate::TransactionType::KeyRegistration
            || tx.key_registration.is_none()
        {
            return Err(Self::Error::DecodingError {
                message: "Key Registration data missing".to_string(),
            });
        }

        let data = tx.clone().key_registration.unwrap();
        let header: algokit_transact::TransactionHeader = tx.try_into()?;

        let transaction_fields = algokit_transact::KeyRegistrationTransactionFields {
            header,
            vote_key: data
                .vote_key
                .map(|buf| vec_to_array::<32>(&buf, "vote key"))
                .transpose()?,
            selection_key: data
                .selection_key
                .map(|buf| vec_to_array::<32>(&buf, "selection key"))
                .transpose()?,
            state_proof_key: data
                .state_proof_key
                .map(|buf| vec_to_array::<64>(&buf, "state proof key"))
                .transpose()?,
            vote_first: data.vote_first,
            vote_last: data.vote_last,
            vote_key_dilution: data.vote_key_dilution,
            non_participation: data.non_participation,
        };

        transaction_fields
            .validate()
            .map_err(|errors| AlgoKitTransactError::DecodingError {
                message: format!("Key registration validation failed: {}", errors.join("\n")),
            })?;

        Ok(transaction_fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::TestDataMother;

    #[test]
    fn test_encode_transaction_validation_integration() {
        // invalid
        let mut tx: Transaction = TestDataMother::online_key_registration().transaction.into();
        tx.key_registration.as_mut().unwrap().vote_key = None;
        let result = encode_transaction(tx);
        assert!(result.is_err());

        // valid
        let result =
            encode_transaction(TestDataMother::online_key_registration().transaction.into());
        assert!(result.is_ok());
    }
}
