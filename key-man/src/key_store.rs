#[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
use crate::BrowserLocalStorageKeyStore;
use crate::{InMemoryKeyStore, KeyPair, StorageKey};

#[derive(Debug, Clone, PartialEq)]
pub enum KeyStore {
    /// A keystore that stores keys in a local storage.
    #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
    LocalStorage(BrowserLocalStorageKeyStore),
    InMemory(InMemoryKeyStore),
}

impl KeyStore {
    pub fn new() -> Self {
        Self::new_in_memory_key_store()
    }

    pub fn new_in_memory_key_store() -> Self {
        KeyStore::InMemory(InMemoryKeyStore::new())
    }

    #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
    pub fn new_browser_local_storage_key_store() -> Self {
        KeyStore::LocalStorage(BrowserLocalStorageKeyStore::new())
    }

    #[allow(unused_variables)]
    pub fn set_key(&self, storage_key: StorageKey, keypair: KeyPair) {
        match self {
            #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
            KeyStore::LocalStorage(key_store) => {
                key_store.set_key(storage_key, keypair);
            }
            _ => {}
        }
    }

    #[allow(unused_variables)]
    pub fn get_key(&self, storage_key: StorageKey) -> Option<KeyPair> {
        match self {
            #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
            KeyStore::LocalStorage(key_store) => key_store.get_key(storage_key),
            _ => None,
        }
    }

    #[allow(unused_variables)]
    pub fn remove_key(&self, storage_key: StorageKey) {
        match self {
            #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
            KeyStore::LocalStorage(key_store) => {
                key_store.remove_key(storage_key);
            }
            _ => {}
        }
    }

    #[allow(dead_code)]
    pub fn clear(&self) {
        match self {
            #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
            KeyStore::LocalStorage(key_store) => {
                key_store.clear();
            }
            _ => {}
        }
    }
}
