use borsh::{BorshDeserialize, BorshSerialize};

use super::StateItem;
use crate::types::TrieProofPath;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ViewStateResult {
    pub values: Vec<StateItem>,
    pub proof: TrieProofPath,
}
