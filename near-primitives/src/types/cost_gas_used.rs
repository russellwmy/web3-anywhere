use borsh::{BorshDeserialize, BorshSerialize};

use crate::{serialize::dec_format, types::Gas};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Clone, Eq, Debug)]
pub struct CostGasUsed {
    pub cost_category: String,
    pub cost: String,
    #[serde(with = "dec_format")]
    pub gas_used: Gas,
}
