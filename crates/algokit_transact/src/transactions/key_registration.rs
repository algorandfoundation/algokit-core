//! Key registration transaction module for AlgoKit Core.
//!
//! This module provides functionality for creating and managing key registration transactions,
//! which are used to register accounts online or offline for participation in Algorand consensus.

use crate::Transaction;
use crate::traits::Validate;
use crate::transactions::common::{TransactionHeader, TransactionValidationError};
use crate::utils::{is_false_opt, is_zero_opt};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as, skip_serializing_none};

/// Represents a key registration transaction that registers an account online or offline
/// for participation in Algorand consensus.
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
#[builder(
    name = "KeyRegistrationTransactionBuilder",
    setter(strip_option),
    build_fn(name = "build_fields")
)]
pub struct KeyRegistrationTransactionFields {
    /// Common transaction header fields.
    #[serde(flatten)]
    pub header: TransactionHeader,

    /// Root participation public key (32 bytes).
    #[serde(rename = "votekey")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub vote_key: Option<[u8; 32]>,

    /// VRF public key (32 bytes).
    #[serde(rename = "selkey")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub selection_key: Option<[u8; 32]>,

    /// State proof key (64 bytes).
    #[serde(rename = "sprfkey")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub state_proof_key: Option<[u8; 64]>,

    /// First round for which the participation key is valid.
    #[serde(rename = "votefst")]
    #[serde(skip_serializing_if = "is_zero_opt")]
    #[serde(default)]
    #[builder(default)]
    pub vote_first: Option<u64>,

    /// Last round for which the participation key is valid.
    #[serde(rename = "votelst")]
    #[serde(skip_serializing_if = "is_zero_opt")]
    #[serde(default)]
    #[builder(default)]
    pub vote_last: Option<u64>,

    /// Key dilution for the 2-level participation key.
    #[serde(rename = "votekd")]
    #[serde(skip_serializing_if = "is_zero_opt")]
    #[serde(default)]
    #[builder(default)]
    pub vote_key_dilution: Option<u64>,

    /// Mark account as non-reward earning.
    #[serde(rename = "nonpart")]
    #[serde(skip_serializing_if = "is_false_opt")]
    #[serde(default)]
    #[builder(default)]
    pub non_participation: Option<bool>,
}

impl KeyRegistrationTransactionFields {
    pub fn validate_for_online(&self) -> Result<(), Vec<TransactionValidationError>> {
        let mut errors = Vec::new();

        if self.vote_key.is_none() {
            errors.push(TransactionValidationError::RequiredField(
                "Vote key".to_string(),
            ));
        }
        if self.selection_key.is_none() {
            errors.push(TransactionValidationError::RequiredField(
                "Selection key".to_string(),
            ));
        }
        if self.state_proof_key.is_none() {
            errors.push(TransactionValidationError::RequiredField(
                "State proof key".to_string(),
            ));
        }
        if self.vote_first.is_none() {
            errors.push(TransactionValidationError::RequiredField(
                "Vote first".to_string(),
            ));
        }
        if self.vote_last.is_none() {
            errors.push(TransactionValidationError::RequiredField(
                "Vote last".to_string(),
            ));
        }
        if let (Some(first), Some(last)) = (self.vote_first, self.vote_last) {
            if first >= last {
                errors.push(TransactionValidationError::ArbitraryConstraint(
                    "Vote first must be less than vote last".to_string(),
                ));
            }
        }
        if self.vote_key_dilution.is_none() {
            errors.push(TransactionValidationError::RequiredField(
                "Vote key dilution".to_string(),
            ));
        }

        if self.non_participation.is_some_and(|v| v) {
            errors.push(TransactionValidationError::ArbitraryConstraint(
                "Online key registration cannot have non participation flag set".to_string(),
            ));
        }

        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors),
        }
    }
}

impl KeyRegistrationTransactionBuilder {
    pub fn build(&self) -> Result<Transaction, KeyRegistrationTransactionBuilderError> {
        let d = self.build_fields()?;
        d.validate().map_err(|errors| {
            KeyRegistrationTransactionBuilderError::ValidationError(format!(
                "Key registration validation failed: {}",
                errors.join("\n")
            ))
        })?;
        Ok(Transaction::KeyRegistration(d))
    }
}

impl Validate for KeyRegistrationTransactionFields {
    fn validate(&self) -> Result<(), Vec<String>> {
        let has_any_participation_fields = self.vote_key.is_some()
            || self.selection_key.is_some()
            || self.state_proof_key.is_some()
            || self.vote_first.is_some()
            || self.vote_last.is_some()
            || self.vote_key_dilution.is_some();

        match has_any_participation_fields {
            true => {
                // Online key registration
                self.validate_for_online()
                    .map_err(|errors| errors.iter().map(|e| e.to_string()).collect())
            }
            false => {
                // Offline key registration (including non-participating)
                // No participation fields present - inherently valid offline state
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        KeyRegistrationTransactionMother, TestDataMother, TransactionHeaderMother,
    };

    #[test]
    fn test_validate_valid_online_key_registration() {
        let online_key_reg = KeyRegistrationTransactionMother::online_key_registration()
            .build_fields()
            .unwrap();

        let result = online_key_reg.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_valid_offline_key_registration() {
        let offline_key_reg = KeyRegistrationTransactionMother::offline_key_registration()
            .build_fields()
            .unwrap();

        let result = offline_key_reg.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_valid_non_participation_key_registration() {
        let non_part_key_reg =
            KeyRegistrationTransactionMother::non_participation_key_registration()
                .build_fields()
                .unwrap();

        let result = non_part_key_reg.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_online_missing_vote_key() {
        let mut key_reg = KeyRegistrationTransactionMother::online_key_registration()
            .build_fields()
            .unwrap();
        key_reg.vote_key = None; // Missing required field

        let result = key_reg.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("Vote key is required")));
    }

    #[test]
    fn test_validate_online_multiple_missing_fields() {
        let key_reg = KeyRegistrationTransactionFields {
            header: TransactionHeaderMother::example().build().unwrap(),
            vote_key: None,      // Missing
            selection_key: None, // Missing
            state_proof_key: Some([3u8; 64]),
            vote_first: None, // Missing
            vote_last: Some(200),
            vote_key_dilution: None, // Missing
            non_participation: None,
        };

        let result = key_reg.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 4); // Should have 4 missing field errors

        let error_text = errors.join("\n");
        assert!(error_text.contains("Vote key is required"));
        assert!(error_text.contains("Selection key is required"));
        assert!(error_text.contains("Vote first is required"));
        assert!(error_text.contains("Vote key dilution is required"));
    }

    #[test]
    fn test_validate_invalid_vote_round_range() {
        let key_reg = KeyRegistrationTransactionMother::online_key_registration()
            .vote_first(200) // Greater than vote_last
            .vote_last(100)
            .build_fields()
            .unwrap();

        let result = key_reg.validate();
        assert!(result.is_err());
        let errors: Vec<String> = result.unwrap_err();
        assert!(
            errors
                .iter()
                .any(|e| e.contains("Vote first must be less than vote last"))
        );
    }

    #[test]
    fn test_validate_equal_vote_rounds() {
        let key_reg = KeyRegistrationTransactionMother::online_key_registration()
            .vote_first(100) // Equal to vote_last
            .vote_last(100)
            .build_fields()
            .unwrap();

        let result = key_reg.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(
            errors
                .iter()
                .any(|e| e.contains("Vote first must be less than vote last"))
        );
    }

    #[test]
    fn test_validate_online_with_non_participation_flag() {
        let key_reg: KeyRegistrationTransactionFields =
            KeyRegistrationTransactionMother::online_key_registration()
                .non_participation(true) // Invalid for online registration
                .build_fields()
                .unwrap();

        let result = key_reg.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(
            errors
                .iter()
                .any(|e| e
                    .contains("Online key registration cannot have non participation flag set"))
        );
    }

    #[test]
    fn test_validate_offline_with_participation_fields() {
        let key_reg: KeyRegistrationTransactionFields =
            KeyRegistrationTransactionMother::offline_key_registration()
                .vote_key([1u8; 32]) // Should not be set for offline
                .build_fields()
                .unwrap();

        let result = key_reg.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        // Since vote_key is set, it should be treated as online registration with missing required fields
        assert!(
            errors
                .iter()
                .any(|e| e.contains("Selection key is required"))
        );
        assert!(
            errors
                .iter()
                .any(|e| e.contains("State proof key is required"))
        );
        assert!(errors.iter().any(|e| e.contains("Vote first is required")));
        assert!(errors.iter().any(|e| e.contains("Vote last is required")));
        assert!(
            errors
                .iter()
                .any(|e| e.contains("Vote key dilution is required"))
        );
    }

    #[test]
    fn test_builder_validation_integration() {
        // Test that the builder properly validates
        let result = KeyRegistrationTransactionMother::offline_key_registration()
            .vote_key([1u8; 32])
            // Missing state_proof_key and other required fields
            .build();

        assert!(result.is_err());

        // Test valid builder
        let result = KeyRegistrationTransactionMother::online_key_registration().build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_non_participation_serialization_skipping() {
        use crate::AlgorandMsgpack;

        // Test that non_participation: Some(false) is skipped during serialization (same as None)
        let key_reg_none = KeyRegistrationTransactionMother::offline_key_registration()
            .build_fields()
            .unwrap();

        let mut key_reg_false = key_reg_none.clone();
        key_reg_false.non_participation = Some(false);

        let mut key_reg_true = key_reg_none.clone();
        key_reg_true.non_participation = Some(true);

        // Encode all three transactions
        let encoded_none = Transaction::KeyRegistration(key_reg_none).encode().unwrap();
        let encoded_false = Transaction::KeyRegistration(key_reg_false)
            .encode()
            .unwrap();
        let encoded_true = Transaction::KeyRegistration(key_reg_true).encode().unwrap();

        // None and Some(false) should produce identical serialization
        assert_eq!(
            encoded_none, encoded_false,
            "Serialization of non_participation: None should be identical to Some(false)"
        );

        // Some(true) should be different (larger, since it includes the field)
        assert_ne!(
            encoded_none, encoded_true,
            "Serialization of non_participation: Some(true) should be different from None/false"
        );

        // The true version should be larger since it includes the field
        assert!(
            encoded_true.len() > encoded_none.len(),
            "Serialization with non_participation: Some(true) should be larger"
        );
    }

    // Integration tests for encoding and transaction functionality
    mod integration_tests {
        use crate::{
            AlgorandMsgpack, SignedTransaction, Transaction, Transactions,
            constants::EMPTY_SIGNATURE,
            test_utils::{
                AccountMother, KeyRegistrationTransactionMother, TransactionHeaderMother,
                TransactionMother,
            },
            traits::TransactionId,
            transactions::FeeParams,
        };

        #[test]
        fn test_online_key_registration_transaction_encoding() {
            let tx_builder = KeyRegistrationTransactionMother::online_key_registration();
            let key_reg_tx_fields = tx_builder.build_fields().unwrap();
            let key_reg_tx = tx_builder.build().unwrap();

            let encoded = key_reg_tx.encode().unwrap();
            let decoded = Transaction::decode(&encoded).unwrap();
            assert_eq!(decoded, key_reg_tx);
            assert_eq!(decoded, Transaction::KeyRegistration(key_reg_tx_fields));

            let signed_tx = SignedTransaction {
                transaction: key_reg_tx.clone(),
                signature: Some(EMPTY_SIGNATURE),
                auth_address: None,
                multisignature: None,
            };
            let encoded_stx = signed_tx.encode().unwrap();
            let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
            assert_eq!(decoded_stx, signed_tx);
            assert_eq!(decoded_stx.transaction, key_reg_tx);

            let raw_encoded = key_reg_tx.encode_raw().unwrap();
            assert_eq!(encoded[0], b'T');
            assert_eq!(encoded[1], b'X');
            assert_eq!(encoded.len(), raw_encoded.len() + 2);
            assert_eq!(encoded[2..], raw_encoded);
        }

        #[test]
        fn test_offline_key_registration_transaction_encoding() {
            let tx_builder = KeyRegistrationTransactionMother::offline_key_registration();
            let key_reg_tx_fields = tx_builder.build_fields().unwrap();
            let key_reg_tx = tx_builder.build().unwrap();

            // Test focuses on encoding/decoding behavior, not transaction type categorization

            let encoded = key_reg_tx.encode().unwrap();
            let decoded = Transaction::decode(&encoded).unwrap();
            assert_eq!(decoded, key_reg_tx);
            assert_eq!(decoded, Transaction::KeyRegistration(key_reg_tx_fields));

            let signed_tx = SignedTransaction {
                transaction: key_reg_tx.clone(),
                signature: Some(EMPTY_SIGNATURE),
                auth_address: None,
                multisignature: None,
            };
            let encoded_stx = signed_tx.encode().unwrap();
            let decoded_stx = SignedTransaction::decode(&encoded_stx).unwrap();
            assert_eq!(decoded_stx, signed_tx);
            assert_eq!(decoded_stx.transaction, key_reg_tx);
        }

        #[test]
        fn test_non_participation_key_registration_transaction_encoding() {
            let tx_builder = KeyRegistrationTransactionMother::non_participation_key_registration();
            let key_reg_tx_fields = tx_builder.build_fields().unwrap();
            let key_reg_tx = tx_builder.build().unwrap();

            // Verify non-participation flag is set correctly
            assert_eq!(key_reg_tx_fields.non_participation, Some(true));

            let encoded = key_reg_tx.encode().unwrap();
            let decoded = Transaction::decode(&encoded).unwrap();
            assert_eq!(decoded, key_reg_tx);
            assert_eq!(decoded, Transaction::KeyRegistration(key_reg_tx_fields));
        }

        #[test]
        fn test_key_registration_transaction_id() {
            let tx_builder = KeyRegistrationTransactionMother::online_key_registration();
            let key_reg_tx = tx_builder.build().unwrap();

            let signed_tx = SignedTransaction {
                transaction: key_reg_tx.clone(),
                signature: Some(EMPTY_SIGNATURE),
                auth_address: None,
                multisignature: None,
            };

            // Test that transaction ID can be generated
            let tx_id = key_reg_tx.id().unwrap();
            let tx_id_raw = key_reg_tx.id_raw().unwrap();

            assert_eq!(signed_tx.id().unwrap(), tx_id);
            assert_eq!(signed_tx.id_raw().unwrap(), tx_id_raw);

            // Transaction ID should be non-empty
            assert!(!tx_id.is_empty());
            assert_ne!(tx_id_raw, [0u8; 32]);
        }

        #[test]
        fn test_key_registration_fee_calculation() {
            let key_reg_tx = KeyRegistrationTransactionMother::online_key_registration()
                .build()
                .unwrap();

            let updated_transaction = key_reg_tx
                .assign_fee(FeeParams {
                    fee_per_byte: 1,
                    min_fee: 1000,
                    extra_fee: None,
                    max_fee: None,
                })
                .unwrap();

            // Fee should be calculated based on transaction size
            assert!(updated_transaction.header().fee.unwrap() >= 1000);
        }

        #[test]
        fn test_key_registration_in_transaction_group() {
            let header_builder = TransactionHeaderMother::testnet()
                .sender(AccountMother::neil().address())
                .first_valid(51532821)
                .last_valid(51533021)
                .to_owned();

            let key_reg_tx = KeyRegistrationTransactionMother::online_key_registration()
                .header(header_builder.build().unwrap())
                .build()
                .unwrap();

            let payment_tx = TransactionMother::simple_payment()
                .header(header_builder.build().unwrap())
                .build()
                .unwrap();

            let txs = vec![key_reg_tx, payment_tx];
            let grouped_txs = txs.assign_group().unwrap();

            assert_eq!(grouped_txs.len(), 2);

            // Both transactions should have the same group ID
            let group_id = grouped_txs[0].header().group.unwrap();
            assert_eq!(grouped_txs[1].header().group.unwrap(), group_id);

            // Group ID should be non-zero
            assert_ne!(group_id, [0u8; 32]);
        }
    }

    #[test]
    fn test_online_key_registration_snapshot() {
        let data = TestDataMother::online_key_registration();
        assert_eq!(
            data.id,
            String::from("UCWQQKWB3CMPVK6EU2ML7CN5IDYZJVVSVS3RXYEOLJUURX44SUKQ")
        );
    }
    #[test]
    fn test_offline_key_registration_snapshot() {
        let data = TestDataMother::offline_key_registration();
        assert_eq!(
            data.id,
            String::from("WAXJLC44RILOSYX73PJULCAWC43DNBU4AXMWHIRARXK4GO2LHEDQ")
        );
    }
    #[test]
    fn test_non_participation_key_registration_snapshot() {
        let data = TestDataMother::non_participation_key_registration();
        assert_eq!(
            data.id,
            String::from("ACAP6ZGMGNTLUO3IQ26P22SRKYWTQQO3MF64GX7QO6NICDUFPM5A")
        );
    }
}
