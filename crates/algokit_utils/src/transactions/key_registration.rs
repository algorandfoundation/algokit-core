use algokit_transact::{KeyRegistrationTransactionFields, Transaction, TransactionHeader};

use super::common::CommonParams;

#[derive(Debug, Default, Clone)]
pub struct OnlineKeyRegistrationParams {
    pub common_params: CommonParams,
    pub vote_key: [u8; 32],
    pub selection_key: [u8; 32],
    pub vote_first: u64,
    pub vote_last: u64,
    pub vote_key_dilution: u64,
    pub state_proof_key: Option<[u8; 64]>,
}

#[derive(Debug, Default, Clone)]
pub struct OfflineKeyRegistrationParams {
    pub common_params: CommonParams,
    pub non_participation: Option<bool>,
}

#[derive(Debug, Default, Clone)]
pub struct NonParticipationKeyRegistrationParams {
    pub common_params: CommonParams,
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
    params: &OfflineKeyRegistrationParams,
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
        non_participation: params.non_participation,
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
