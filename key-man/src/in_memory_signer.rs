use std::io::{Error, ErrorKind};

use crate::{
    crypto::{KeyType, PublicKey, Signature},
    KeyPair,
    KeyStore,
    StorageKey,
};

#[derive(Debug, Clone, PartialEq)]
pub struct InMemorySigner {
    pub key_store: KeyStore,
}

impl InMemorySigner {
    pub fn new(key_store: KeyStore) -> Self {
        Self { key_store }
    }
}

impl InMemorySigner {
    pub fn create_key(&self, storage_key: StorageKey) -> PublicKey {
        let key_pair = KeyPair::from_random(KeyType::ED25519);
        let public_key = key_pair.public_key();
        self.key_store.set_key(storage_key, key_pair);

        public_key
    }

    pub fn from_key_pair(storage_key: StorageKey, keypair: KeyPair) -> Self {
        let key_store = KeyStore::new_in_memory_key_store();

        key_store.set_key(storage_key, keypair);

        Self { key_store }
    }

    pub fn get_public_key(&self, storage_key: StorageKey) -> Result<PublicKey, Error> {
        let key_pair = self.key_store.get_key(storage_key);

        match key_pair {
            Some(key_pair) => Ok(key_pair.public_key()),
            None => Err(Error::new(
                ErrorKind::NotFound,
                "No key found for this account",
            )),
        }
    }

    pub fn sign_message(&self, message: &[u8], storage_key: StorageKey) -> Signature {
        let key_pair = self
            .key_store
            .get_key(storage_key)
            .expect("keypair not found");

        key_pair.sign(message)
    }
}
