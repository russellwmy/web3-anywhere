use borsh::{BorshDeserialize, BorshSerialize};

use super::TransactionOrReceiptId;
use crate::hash::CryptoHash;

#[derive(Serialize, Deserialize, Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct LightClientProofRequest {
    #[serde(flatten)]
    id: TransactionOrReceiptId,
    light_client_head: CryptoHash,
}
