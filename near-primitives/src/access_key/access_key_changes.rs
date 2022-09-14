use borsh::{BorshDeserialize, BorshSerialize};
use web3_anywhere_crypto::PublicKey;

use crate::{account::AccessKey, types::AccountId};
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct AccessKeyChanges {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    pub access_key: AccessKey,
}
