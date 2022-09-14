use web3_anywhere_crypto::PublicKey;

use crate::{
    serialize::dec_format,
    types::{AccountId, Balance, NumBlocks, ShardId},
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct CurrentEpochValidatorInfo {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    pub is_slashed: bool,
    #[serde(with = "dec_format")]
    pub stake: Balance,
    pub shards: Vec<ShardId>,
    pub num_produced_blocks: NumBlocks,
    pub num_expected_blocks: NumBlocks,
    #[serde(default)]
    pub num_produced_chunks: NumBlocks,
    #[serde(default)]
    pub num_expected_chunks: NumBlocks,
}
