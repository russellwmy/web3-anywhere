use std::collections::HashMap;

use crate::crypto::PublicKey;

#[derive(Clone)]
pub struct UrlAccountCreator {
    help_url: String,
}

impl UrlAccountCreator {
    pub fn new(help_url: String) -> Self {
        Self { help_url }
    }

    pub async fn create_account(&self, account_id: &str, public_key: PublicKey) {
        let public_key = public_key.to_string();
        let mut data = HashMap::new();
        data.insert("newAccountId", account_id);
        data.insert("newAccountPublicKey", &public_key);

        let client = reqwest::Client::new();
        let res = client.post(&self.help_url).json(&data).send().await;
    }
}
