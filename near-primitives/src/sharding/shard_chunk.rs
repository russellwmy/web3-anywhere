use borsh::{BorshDeserialize, BorshSerialize};

use super::{ChunkHash, ShardChunkHeader};
use crate::{receipt::Receipt, transaction::SignedTransaction};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Eq, PartialEq)]
pub struct ShardChunk {
    pub chunk_hash: ChunkHash,
    pub header: ShardChunkHeader,
    pub transactions: Vec<SignedTransaction>,
    pub receipts: Vec<Receipt>,
}
