use crate::key_man::{KeyStore, Signer};

#[derive(Debug, Clone, PartialEq)]
pub struct NearConfig {
    pub explorer_url: String,
    pub helper_url: String,
    pub wallet_url: String,
    pub node_url: String,
    pub network: String,
    pub key_store: KeyStore,
    pub signer: Signer,
}
