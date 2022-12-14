use borsh::{BorshDeserialize, BorshSerialize};

use crate::{hash::CryptoHash, serialize::option_base64_format};

#[derive(
    BorshSerialize, BorshDeserialize, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Debug,
)]
pub struct DataReceipt {
    pub data_id: CryptoHash,
    #[serde(with = "option_base64_format")]
    pub data: Option<Vec<u8>>,
}
