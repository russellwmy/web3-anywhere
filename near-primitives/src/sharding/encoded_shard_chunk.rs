use borsh::{BorshDeserialize, BorshSerialize};

use super::{EncodedShardChunkBody, ShardChunkHeader};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct EncodedShardChunk {
    pub header: ShardChunkHeader,
    pub content: EncodedShardChunkBody,
}
