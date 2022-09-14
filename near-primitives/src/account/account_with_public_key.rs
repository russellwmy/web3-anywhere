use web3_anywhere_crypto::PublicKey;

use crate::types::AccountId;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountWithPublicKey {
    pub account_id: AccountId,
    pub public_key: PublicKey,
}
