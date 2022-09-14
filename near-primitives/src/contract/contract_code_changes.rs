use borsh::{BorshDeserialize, BorshSerialize};

use crate::{serialize::base64_format, types::AccountId};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Eq, Debug)]
pub struct ContractCodeChanges {
    account_id: AccountId,
    #[serde(alias = "code_base64", with = "base64_format")]
    pub code: Vec<u8>,
}
