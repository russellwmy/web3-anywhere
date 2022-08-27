use crate::key_man::{KeyStore, Signer};

#[derive(Debug, Clone, PartialEq)]
pub struct WalletConfig {
    pub explorer_url: Option<String>,
    pub helper_url: Option<String>,
    pub wallet_url: Option<String>,
    pub network: Option<String>,
    pub key_store: KeyStore,
    pub signer: Option<Signer>,
}

impl WalletConfig {
    pub fn new(key_store: KeyStore) -> Self {
        Self {
            explorer_url: None,
            helper_url: None,
            wallet_url: None,
            network: None,
            key_store,
            signer: None,
        }
    }
}
