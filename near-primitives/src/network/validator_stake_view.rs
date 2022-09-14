use borsh::{BorshDeserialize, BorshSerialize};
use web3_anywhere_crypto::PublicKey;

use super::ValidatorStake;
use crate::{
    serialize::dec_format,
    types::{AccountId, Balance},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ValidatorStakeView {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    #[serde(with = "dec_format")]
    pub stake: Balance,
}

impl From<ValidatorStake> for ValidatorStakeView {
    fn from(stake: ValidatorStake) -> Self {
        Self {
            account_id: stake.account_id,
            public_key: stake.public_key,
            stake: stake.stake,
        }
    }
}

impl From<ValidatorStakeView> for ValidatorStake {
    fn from(view: ValidatorStakeView) -> Self {
        Self {
            account_id: view.account_id,
            public_key: view.public_key,
            stake: view.stake,
        }
    }
}
