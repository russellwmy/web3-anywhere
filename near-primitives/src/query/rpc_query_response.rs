use near_primitives_core::hash::CryptoHash;

use super::QueryResponseKind;
use crate::types::BlockHeight;

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcQueryResponse {
    #[serde(flatten)]
    pub kind: QueryResponseKind,
    pub block_height: BlockHeight,
    pub block_hash: CryptoHash,
}
