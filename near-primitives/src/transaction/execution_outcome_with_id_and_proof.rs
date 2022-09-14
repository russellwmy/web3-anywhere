use borsh::{BorshDeserialize, BorshSerialize};

use super::ExecutionOutcomeWithId;
use crate::{hash::CryptoHash, merkle::MerklePath};

#[derive(PartialEq, Clone, Default, Debug, BorshSerialize, BorshDeserialize, Eq)]
pub struct ExecutionOutcomeWithIdAndProof {
    pub proof: MerklePath,
    pub block_hash: CryptoHash,
    /// Should be the latest field since contains unparsable by light client ExecutionStatus::Failure
    pub outcome_with_id: ExecutionOutcomeWithId,
}

impl ExecutionOutcomeWithIdAndProof {
    pub fn id(&self) -> &CryptoHash {
        &self.outcome_with_id.id
    }
}
