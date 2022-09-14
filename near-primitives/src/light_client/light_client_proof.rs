use borsh::{BorshDeserialize, BorshSerialize};

use super::LightClientBlockLiteView;
use crate::merkle::MerklePath;

#[derive(Serialize, Deserialize, Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct LightClientProof {
    pub outcome_root_proof: MerklePath,
    pub block_header_lite: LightClientBlockLiteView,
    pub block_proof: MerklePath,
}
