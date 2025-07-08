use super::common::CommonParams;

#[derive(Debug, Clone)]
pub struct OnlineKeyRegistrationParams {
    pub common_params: CommonParams,
    pub vote_key: [u8; 32],
    pub selection_key: [u8; 32],
    pub vote_first: u64,
    pub vote_last: u64,
    pub vote_key_dilution: u64,
    pub state_proof_key: Option<[u8; 64]>,
}

#[derive(Debug, Clone)]
pub struct OfflineKeyRegistrationParams {
    pub common_params: CommonParams,
    pub non_participation: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct NonParticipationKeyRegistrationParams {
    pub common_params: CommonParams,
}
