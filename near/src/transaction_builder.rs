use std::str::FromStr;

use crate::{
    crypto::PublicKey,
    primitives::{
        hash::CryptoHash,
        types::{AccountId, Nonce},
    },
    Action,
    Transaction,
};

#[derive(Default)]
pub struct TransactionBuilder {
    /// An account on which behalf transaction is signed
    signer_id: Option<AccountId>,
    /// A public key of the access key which was used to sign an account.
    /// Access key holds permissions for calling certain kinds of actions.
    public_key: Option<PublicKey>,
    /// Nonce is used to determine order of transaction in the pool.
    /// It increments for a combination of `signer_id` and `public_key`
    nonce: Option<Nonce>,
    /// Receiver account for this transaction
    receiver_id: Option<AccountId>,
    /// The hash of the block in the blockchain on top of which the given transaction is valid
    block_hash: Option<CryptoHash>,
    // A list of actions to be applied
    actions: Option<Vec<Action>>,
}

impl TransactionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn signer_id(mut self, signer_id: &str) -> Self {
        self.signer_id = Some(signer_id.parse::<AccountId>().unwrap());
        self
    }

    pub fn public_key(mut self, public_key: &str) -> Self {
        self.public_key = Some(PublicKey::from_str(public_key).unwrap());
        self
    }

    pub fn nonce(mut self, nonce: Nonce) -> Self {
        self.nonce = Some(nonce);
        self
    }

    pub fn receiver_id(mut self, receiver_id: &str) -> Self {
        self.receiver_id = Some(receiver_id.parse::<AccountId>().unwrap());
        self
    }

    pub fn block_hash(mut self, block_hash: &str) -> Self {
        self.block_hash = Some(CryptoHash::from_str(block_hash).unwrap());
        self
    }

    pub fn actions(mut self, actions: Vec<Action>) -> Self {
        self.actions = Some(actions);
        self
    }

    pub fn build(self) -> Transaction {
        let signer_id = self.signer_id.expect("Missing signer");
        let public_key = self.public_key.expect("Missing public key");
        let nonce = self.nonce.expect("Missing nounce");
        let receiver_id = self.receiver_id.expect("Missing receiver");
        let block_hash = self.block_hash.expect("Missing block hash");
        let actions = self.actions.unwrap_or(vec![]);

        Transaction {
            signer_id,
            public_key,
            nonce,
            receiver_id,
            block_hash,
            actions,
        }
    }
}
