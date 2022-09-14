use borsh::{BorshDeserialize, BorshSerialize};

use crate::{
    serialize::dec_format,
    types::{AccountId, Balance, NumBlocks},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ValidatorKickoutReason {
    Slashed,
    NotEnoughBlocks {
        produced: NumBlocks,
        expected: NumBlocks,
    },
    NotEnoughChunks {
        produced: NumBlocks,
        expected: NumBlocks,
    },
    Unstaked,
    NotEnoughStake {
        #[serde(with = "dec_format", rename = "stake_u128")]
        stake: Balance,
        #[serde(with = "dec_format", rename = "threshold_u128")]
        threshold: Balance,
    },
    DidNotGetASeat,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ValidatorKickoutView {
    pub account_id: AccountId,
    pub reason: ValidatorKickoutReason,
}
