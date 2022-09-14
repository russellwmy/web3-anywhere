use borsh::{BorshDeserialize, BorshSerialize};

use crate::{
    block::BlockDoubleSign,
    sharding::{ChunkProofs, ChunkState},
};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone, Debug)]
pub enum ChallengeBody {
    BlockDoubleSign(BlockDoubleSign),
    ChunkProofs(ChunkProofs),
    ChunkState(ChunkState),
}
