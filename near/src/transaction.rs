use borsh::{BorshDeserialize, BorshSerialize};

use crate::{
    crypto::PublicKey,
    primitives::{
        hash::CryptoHash,
        types::{AccountId, Nonce},
    },
    Action,
    TransactionBuilder,
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Transaction {
    /// An account on which behalf transaction is signed
    pub signer_id: AccountId,
    /// A public key of the access key which was used to sign an account.
    /// Access key holds permissions for calling certain kinds of actions.
    pub public_key: PublicKey,
    /// Nonce is used to determine order of transaction in the pool.
    /// It increments for a combination of `signer_id` and `public_key`
    pub nonce: Nonce,
    /// Receiver account for this transaction
    pub receiver_id: AccountId,
    /// The hash of the block in the blockchain on top of which the given transaction is valid
    pub block_hash: CryptoHash,
    // A list of actions to be applied
    pub actions: Vec<Action>,
}

impl Transaction {
    pub fn builder() -> TransactionBuilder {
        TransactionBuilder::default()
    }
}
