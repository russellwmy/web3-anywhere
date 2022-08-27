use std::io::Error;

use crate::{
    crypto::{PublicKey, Signature},
    InMemorySigner,
    KeyStore,
    StorageKey,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Signer {
    InMemorySigner(InMemorySigner),
}

impl Signer {
    pub fn new_in_memory_signer(key_store: KeyStore) -> Self {
        Signer::InMemorySigner(InMemorySigner::new(key_store))
    }

    pub fn create_key(&self, storage_key: StorageKey) -> PublicKey {
        match self {
            Signer::InMemorySigner(signer) => signer.create_key(storage_key),
        }
    }

    pub fn get_public_key(&self, storage_key: StorageKey) -> Result<PublicKey, Error> {
        match self {
            Signer::InMemorySigner(signer) => signer.get_public_key(storage_key),
        }
    }

    pub fn sign_message(&self, message: &[u8], storage_key: StorageKey) -> Signature {
        match self {
            Signer::InMemorySigner(signer) => signer.sign_message(message, storage_key),
        }
    }
}
