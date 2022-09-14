use borsh::{BorshDeserialize, BorshSerialize};

use super::Direction;
use crate::types::MerkleHash;

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct MerklePathItem {
    pub hash: MerkleHash,
    pub direction: Direction,
}
