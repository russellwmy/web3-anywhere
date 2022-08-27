use hashbrown::HashMap;

use crate::{KeyPair, StorageKey};

#[derive(Debug, Clone, PartialEq)]
pub struct InMemoryKeyStore {
    storage: HashMap<String, String>,
}

impl InMemoryKeyStore {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
}

impl InMemoryKeyStore {
    pub fn set_key(&mut self, storage_key: StorageKey, keypair: KeyPair) {
        self.storage
            .insert(storage_key.to_string(), keypair.to_string())
            .expect("failed to set key pair");
    }

    pub fn get_key(&self, storage_key: StorageKey) -> Option<KeyPair> {
        let value = self.storage.get(&storage_key.to_string());

        match value {
            Some(value) => Some(KeyPair::from_secret_key(&value).unwrap()),
            None => None,
        }
    }

    pub fn remove_key(&mut self, storage_key: StorageKey) {
        self.storage.remove(&storage_key.to_string()).unwrap();
    }

    pub fn clear(&mut self) {
        self.storage.clear();
    }
}
