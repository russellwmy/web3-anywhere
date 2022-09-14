use borsh::{BorshDeserialize, BorshSerialize};

use super::{ShardChunk, ShardChunkHeader};
use crate::{merkle::MerklePath, transaction::PartialState};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone, Debug)]
pub struct ChunkState {
    /// Encoded prev block header.
    pub prev_block_header: Vec<u8>,
    /// Encoded block header that contains invalid chunnk.
    pub block_header: Vec<u8>,
    /// Merkle proof in inclusion of prev chunk.
    pub prev_merkle_proof: MerklePath,
    /// Previous chunk that contains transactions.
    pub prev_chunk: ShardChunk,
    /// Merkle proof of inclusion of this chunk.
    pub merkle_proof: MerklePath,
    /// Invalid chunk header.
    pub chunk_header: ShardChunkHeader,
    /// Partial state that was affected by transactions of given chunk.
    pub partial_state: PartialState,
}
