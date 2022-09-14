use borsh::{BorshDeserialize, BorshSerialize};

use super::{EncodedShardChunk, ShardChunk};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone, Debug)]
pub enum MaybeEncodedShardChunk {
    Encoded(EncodedShardChunk),
    Decoded(ShardChunk),
}
