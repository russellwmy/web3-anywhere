use web3_anywhere_crypto::PublicKey;

use crate::types::AccountId;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct KnownProducerView {
    pub account_id: AccountId,
    pub peer_id: PublicKey,
    pub next_hops: Option<Vec<PublicKey>>,
}
