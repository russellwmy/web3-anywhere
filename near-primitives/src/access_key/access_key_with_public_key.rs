use borsh::{BorshDeserialize, BorshSerialize};
use web3_anywhere_crypto::PublicKey;

use super::AccessKeyView;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct AccessKeyWithPublicKey {
    pub public_key: PublicKey,
    pub access_key: AccessKeyView,
}
