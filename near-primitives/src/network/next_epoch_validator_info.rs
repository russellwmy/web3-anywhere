use web3_anywhere_crypto::PublicKey;

use crate::{
    serialize::dec_format,
    types::{AccountId, Balance, ShardId},
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct NextEpochValidatorInfo {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    #[serde(with = "dec_format")]
    pub stake: Balance,
    pub shards: Vec<ShardId>,
}
