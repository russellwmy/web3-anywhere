use borsh::{BorshDeserialize, BorshSerialize};

use crate::{hash::CryptoHash, types::BlockHeight};

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockId {
    Height(BlockHeight),
    Hash(CryptoHash),
}
