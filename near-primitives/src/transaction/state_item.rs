use borsh::{BorshDeserialize, BorshSerialize};

use crate::types::TrieProofPath;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct StateItem {
    pub key: String,
    pub value: String,
    pub proof: TrieProofPath,
}
