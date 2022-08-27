use web_sys::Storage;

use crate::{KeyPair, StorageKey};

#[derive(Debug, Clone, PartialEq)]
pub struct BrowserLocalStorageKeyStore {
    storage: Storage,
}

impl BrowserLocalStorageKeyStore {
    pub fn new() -> Self {
        let storage = web_sys::window()
            .expect("no global `window` exists")
            .local_storage()
            .unwrap()
            .expect("no `localStorage` exists");
        Self { storage }
    }
}

impl BrowserLocalStorageKeyStore {
    pub fn set_key(&self, storage_key: StorageKey, keypair: KeyPair) {
        let key = storage_key.to_string();
        self.storage
            .set_item(&key, keypair.to_string().as_str())
            .expect("failed to set key pair");
    }

    pub fn get_key(&self, storage_key: StorageKey) -> Option<KeyPair> {
        let key = storage_key.to_string();
        let value = self.storage.get_item(&key).unwrap();

        match value {
            Some(value) => Some(KeyPair::from_secret_key(&value).unwrap()),
            None => None,
        }
    }

    pub fn remove_key(&self, storage_key: StorageKey) {
        let key = storage_key.to_string();
        self.storage.remove_item(&key).unwrap();
    }

    pub fn clear(&self) {
        let keys = self.storage_keys();

        for key in keys {
            self.storage.remove_item(&key).unwrap();
        }
    }

    fn storage_keys(&self) -> Vec<String> {
        let len = self.storage.length().unwrap();
        let mut keys = vec![];

        for i in 0..len {
            let key = self.storage.key(i).unwrap();
            match key {
                Some(key) => {
                    let storage_key = key.parse::<StorageKey>();
                    match storage_key {
                        Ok(_) => keys.push(key.to_string()),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        keys
    }
}
