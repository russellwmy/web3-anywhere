use borsh::{BorshDeserialize, BorshSerialize};

use super::BlockHeader;
use crate::{challenge::Challenges, sharding::ShardChunkHeader};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Eq, PartialEq)]
pub struct Block {
    pub header: BlockHeader,
    pub chunks: Vec<ShardChunkHeader>,
    pub challenges: Challenges,
    // Data to confirm the correctness of randomness beacon output
    // pub vrf_value: near_crypto::vrf::Value,
    // pub vrf_proof: near_crypto::vrf::Proof,
}
