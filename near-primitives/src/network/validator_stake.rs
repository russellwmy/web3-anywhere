use borsh::{BorshDeserialize, BorshSerialize};
use web3_anywhere_crypto::PublicKey;

use crate::types::{AccountId, Balance};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ValidatorStake {
    /// Account that stakes money.
    pub account_id: AccountId,
    /// Public key of the proposed validator.
    pub public_key: PublicKey,
    /// Stake / weight of the validator.
    pub stake: Balance,
}
