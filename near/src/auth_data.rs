use std::{collections::HashMap, convert::TryFrom};

use crate::primitives::types::AccountId;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthData {
    pub account_id: AccountId,
    pub all_keys: Vec<String>,
}

impl TryFrom<HashMap<String, String>> for AuthData {
    type Error = &'static str;

    fn try_from(map: HashMap<String, String>) -> Result<Self, Self::Error> {
        let account_id_value = map.get("account_id");
        let all_keys_value = map.get("all_keys");

        if account_id_value.is_none() || all_keys_value.is_none() {
            Err("Missing values")
        } else {
            let account_id = account_id_value.unwrap().to_string().parse().unwrap();
            let all_keys = all_keys_value
                .unwrap()
                .to_string()
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            Ok(Self {
                account_id,
                all_keys,
            })
        }
    }
}
