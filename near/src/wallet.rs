#[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
use std::collections::HashMap;

#[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
use borsh::BorshSerialize;
use url::Url;

use crate::{
    auth_data::AuthData,
    crypto::{serialize::to_base64, KeyType, PublicKey, Signature},
    key_man::{KeyPair, KeyStore, Signer, StorageKey},
    primitives::types::AccountId,
    Account,
    Transaction,
    WalletConfig,
};

const LOGIN_WALLET_URL_SUFFIX: &str = "/login/";
// const MULTISIG_HAS_METHOD: &str = "add_request_and_confirm";
const LOCAL_STORAGE_KEY_SUFFIX: &str = "_wallet_auth_key";
const PENDING_ACCESS_KEY_PREFIX: &str = "pending_key";

#[derive(Debug, Clone, PartialEq)]
pub struct Wallet {
    config: WalletConfig,
    auth_data_key: String,
    wallet_base_url: String,
    key_store: KeyStore,
    auth_data: Option<AuthData>,
    network: String,
    connected_account: Option<Account>,
}

impl Wallet {
    pub fn new(config: WalletConfig) -> Self {
        Self::new_with_prefix(config, "web3_anywhere:near")
    }

    pub fn new_with_prefix(config: WalletConfig, prefix: &str) -> Self {
        let wallet_config = config.clone();
        let auth_data_key = format!("{}{}", prefix, LOCAL_STORAGE_KEY_SUFFIX);
        let wallet_base_url = wallet_config.wallet_url.clone().unwrap_or("".to_string());
        let network = wallet_config.network.unwrap_or("".to_string());
        let auth_data = {
            #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
            {
                let storage = crate::browser::local_storage();
                let data = storage.get_item(&auth_data_key).unwrap();
                match data {
                    Some(data) => {
                        let data: AuthData = serde_json::from_str(&data).unwrap();

                        Some(data)
                    }
                    _ => None,
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            None
        };
        let key_store = match wallet_config.signer.clone() {
            Some(signer) => match signer {
                Signer::InMemorySigner(signer) => signer.key_store,
                _ => KeyStore::new(),
            },
            _ => KeyStore::new(),
        };

        Self {
            config,
            wallet_base_url,
            key_store,
            auth_data_key,
            auth_data,
            network,
            connected_account: None,
        }
    }
    pub fn get_signer(&self) -> Option<Signer> {
        self.config.signer.clone()
    }

    pub fn get_public_key(&self) -> Option<PublicKey> {
        let account_id = self.get_account_id();
        match account_id {
            Some(account_id) => match self.config.signer.clone() {
                Some(signer) => Some(
                    signer
                        .get_public_key(StorageKey::new_near_key(&self.network, &account_id))
                        .unwrap(),
                ),
                None => None,
            },
            None => None,
        }
    }

    pub fn get_account_id(&self) -> Option<AccountId> {
        match self.auth_data.clone() {
            Some(auth_data) => Some(auth_data.account_id),
            None => None,
        }
    }

    pub fn is_signed_in(&self) -> bool {
        match &self.auth_data {
            Some(auth_data) => !auth_data.account_id.is_empty(),
            None => false,
        }
    }

    pub fn sign_out(&mut self) {
        #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
        {
            let network = self.network.clone();
            let account_id = self.get_account_id();
            if account_id.is_some() {
                let key = StorageKey::new_near_key(&network, &account_id.unwrap());
                self.key_store.remove_key(key);
            }
            crate::browser::local_storage().remove_item(&self.auth_data_key);
        }
        self.auth_data = None;
    }

    pub async fn request_sign_in(
        &mut self,
        contract_id: Option<String>,
        method_names: Option<Vec<String>>,
        success_url: Option<String>,
        failure_url: Option<String>,
    ) {
        let mut url = Url::parse(&format!(
            "{}{}",
            self.wallet_base_url, LOGIN_WALLET_URL_SUFFIX
        ))
        .unwrap();
        if let Some(contract_id) = contract_id {
            url.query_pairs_mut()
                .append_pair("contract_id", &contract_id);
            let key_pair = KeyPair::from_random(KeyType::ED25519);
            let public_key = key_pair.public_key().to_string();
            url.query_pairs_mut().append_pair("public_key", &public_key);

            let network = self.network.clone();
            let temp_account = format!("{}{}", PENDING_ACCESS_KEY_PREFIX, public_key);
            let temp_key = StorageKey::new_near_key(&network, &temp_account);
            self.key_store.set_key(temp_key, key_pair);
        }
        if let Some(method_names) = method_names {
            for method_name in method_names {
                url.query_pairs_mut()
                    .append_pair("methodNames", &method_name);
            }
        }
        if let Some(success_url) = success_url {
            url.query_pairs_mut()
                .append_pair("success_url", &success_url);
        } else {
            #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
            {
                let current_url = crate::browser::current_url();
                url.query_pairs_mut()
                    .append_pair("success_url", &current_url);
            }
        }
        if let Some(failure_url) = failure_url {
            url.query_pairs_mut()
                .append_pair("failure_url", &failure_url);
        } else {
            #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
            {
                let current_url = crate::browser::current_url();
                url.query_pairs_mut()
                    .append_pair("failure_url", &current_url);
            }
        }

        let url = url.to_string();

        #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
        {
            crate::browser::location().assign(&url);
        }
    }

    pub async fn request_sign_transactions(
        &self,
        transactions: Vec<Transaction>,
        callback_url: Option<String>,
        meta: Option<String>,
    ) {
        #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
        {
            let current_url = crate::browser::current_url();
            let callback_url = callback_url.unwrap_or(current_url);
            let sign_url = format!("{}/sign", self.wallet_base_url);
            let mut new_url = url::Url::parse(&sign_url).unwrap();
            let transactions = transactions
                .iter()
                .map(|t| to_base64(t.try_to_vec().unwrap()))
                .collect::<Vec<String>>();

            let transaction_data = transactions.join(",");

            new_url
                .query_pairs_mut()
                .append_pair("transactions", &transaction_data);

            new_url
                .query_pairs_mut()
                .append_pair("callbackUrl", &callback_url);
            if let Some(meta) = meta {
                new_url.query_pairs_mut().append_pair("meta", &meta);
            }
            let new_url = new_url.to_string();

            crate::browser::location().assign(&new_url);
        }
    }

    fn _move_key_from_temp_to_permanent(&self, account: &str, public_key: &str) {
        let temp_account = format!("{}{}", PENDING_ACCESS_KEY_PREFIX, public_key);
        let network = self.network.clone();
        let temp_key = StorageKey::new_near_key(&network, &temp_account);
        let key_pair = self.key_store.get_key(temp_key.clone());
        if let Some(key_pair) = key_pair {
            let account_key = StorageKey::new_near_key(&network, &account);
            self.key_store.set_key(account_key, key_pair);
            self.key_store.remove_key(temp_key);
        }
    }

    pub fn complete_sign_in_with_access_key(&mut self) {
        #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
        {
            let storage = crate::browser::local_storage();
            let url = crate::browser::current_url();
            let mut parsed_url = Url::parse(&url).unwrap();
            let count = parsed_url.query_pairs().count();
            let query_pairs = parsed_url.query_pairs().into_owned();

            if count > 0 {
                let params = query_pairs.collect::<HashMap<String, String>>();
                let auth_data = AuthData::parse(params.clone());
                let result = serde_json::to_string(&auth_data).unwrap();
                let account_id = auth_data.account_id.clone();
                storage
                    .set_item(&self.auth_data_key, &result)
                    .expect("fail to save data");
                if let Some(public_key) = params.clone().get("public_key") {
                    self._move_key_from_temp_to_permanent(&account_id, public_key);
                }
                self.auth_data = Some(auth_data);
                let mut new_params = params.clone();
                new_params.remove("public_key");
                new_params.remove("all_keys");
                new_params.remove("account_id");
                new_params.remove("meta");
                new_params.remove("transactionHashes");
                parsed_url.query_pairs_mut().clear();

                if !new_params.is_empty() {
                    parsed_url.query_pairs_mut().extend_pairs(new_params.iter());
                } else {
                    parsed_url.set_query(None);
                }

                crate::browser::replace_url(parsed_url.as_str());
            }
        }
    }
    pub fn sign_message(self, message: &[u8]) -> Signature {
        let signer = self.config.signer.clone().expect("Missing signer");
        let account_id = self.get_account_id().expect("Missing account id");
        let signature = signer.sign_message(
            message,
            StorageKey::new_near_key(&self.network, &account_id),
        );

        signature
    }
}
