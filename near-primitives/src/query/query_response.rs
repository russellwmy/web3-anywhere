use borsh::{BorshDeserialize, BorshSerialize};

use super::QueryResponseKind;
use crate::{hash::CryptoHash, types::BlockHeight};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Eq, Clone)]
pub struct QueryResponse {
    pub kind: QueryResponseKind,
    pub block_height: BlockHeight,
    pub block_hash: CryptoHash,
}
