use algokit_transact::{KeyRegistrationTransactionFields, Transaction, TransactionHeader};

use super::common::CommonTransactionParams;

#[derive(Debug, Default, Clone)]
/// Parameters for creating an online key registration transaction.
pub struct OnlineKeyRegistrationParams {
    /// Common parameters used across all transaction types
    pub common_params: CommonTransactionParams,
    /// The root participation public key
    pub vote_key: [u8; 32],
    /// The VRF public key
    pub selection_key: [u8; 32],
    ///  The first round that the participation key is valid. Not to be confused with the first valid round of the keyreg transaction
    pub vote_first: u64,
    /// The last round that the participation key is valid. Not to be confused with the last valid round of the keyreg transaction
    pub vote_last: u64,
    /// This is the dilution for the 2-level participation key. It determines the interval (number of rounds) for generating new ephemeral keys
    pub vote_key_dilution: u64,
    /// The 64 byte state proof public key commitment
    pub state_proof_key: Option<[u8; 64]>,
}

#[derive(Debug, Default, Clone)]
/// Parameters for creating an offline key registration transaction.
pub struct OfflineKeyRegistrationParams {
    /// Common parameters used across all transaction types
    pub common_params: CommonTransactionParams,
}

#[derive(Debug, Default, Clone)]
/// Parameters for creating an non participation key registration transaction.
///
/// **Warning:** This will prevent the sender account from ever participating again. The account will also no longer earn rewards.
pub struct NonParticipationKeyRegistrationParams {
    /// Common parameters used across all transaction types
    pub common_params: CommonTransactionParams,
}

pub fn build_online_key_registration(
    params: &OnlineKeyRegistrationParams,
    header: TransactionHeader,
) -> Transaction {
    Transaction::KeyRegistration(KeyRegistrationTransactionFields {
        header,
        vote_key: Some(params.vote_key),
        selection_key: Some(params.selection_key),
        vote_first: Some(params.vote_first),
        vote_last: Some(params.vote_last),
        vote_key_dilution: Some(params.vote_key_dilution),
        state_proof_key: params.state_proof_key,
        non_participation: None,
    })
}

pub fn build_offline_key_registration(
    _params: &OfflineKeyRegistrationParams,
    header: TransactionHeader,
) -> Transaction {
    Transaction::KeyRegistration(KeyRegistrationTransactionFields {
        header,
        vote_key: None,
        selection_key: None,
        vote_first: None,
        vote_last: None,
        vote_key_dilution: None,
        state_proof_key: None,
        non_participation: None,
    })
}

pub fn build_non_participation_key_registration(
    _params: &NonParticipationKeyRegistrationParams,
    header: TransactionHeader,
) -> Transaction {
    Transaction::KeyRegistration(KeyRegistrationTransactionFields {
        header,
        vote_key: None,
        selection_key: None,
        vote_first: None,
        vote_last: None,
        vote_key_dilution: None,
        state_proof_key: None,
        non_participation: Some(true),
    })
}
