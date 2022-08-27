const LOCAL_STORAGE_KEY_PREFIX: &str = "web3_anywhere";

use web3_anywhere_crypto as crypto;

mod in_memory_key_store;
mod in_memory_signer;
mod key_pair;
mod key_store;
mod signer;
mod storage_key;

pub use in_memory_key_store::InMemoryKeyStore;
pub use in_memory_signer::InMemorySigner;
pub use key_pair::KeyPair;
pub use key_store::KeyStore;
pub use signer::Signer;
pub use storage_key::StorageKey;

#[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
mod browser_local_storage_key_store;

#[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
pub use browser_local_storage_key_store::BrowserLocalStorageKeyStore;
