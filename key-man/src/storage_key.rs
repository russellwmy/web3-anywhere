use core::{fmt, str::FromStr};

use crate::LOCAL_STORAGE_KEY_PREFIX;

#[derive(Clone)]
pub struct StorageKey {
    prefix: String,
    blockchain: String,
    network: String,
    account: String,
}

impl StorageKey {
    pub fn new_near_key(network: &str, account: &str) -> Self {
        Self {
            prefix: LOCAL_STORAGE_KEY_PREFIX.to_string(),
            blockchain: "near".to_string(),
            network: network.to_string(),
            account: account.to_string(),
        }
    }

    pub fn new(blockchain: &str, network: &str, account: &str) -> Self {
        Self {
            prefix: LOCAL_STORAGE_KEY_PREFIX.to_string(),
            blockchain: blockchain.to_string(),
            network: network.to_string(),
            account: account.to_string(),
        }
    }
}

impl FromStr for StorageKey {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(":").collect::<Vec<&str>>();

        Ok(Self {
            prefix: parts[0].to_string(),
            blockchain: parts[1].to_string(),
            network: parts[2].to_string(),
            account: parts[3].to_string(),
        })
    }
}

impl fmt::Display for StorageKey {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&format!(
            "{}:{}:{}:{}",
            self.prefix, self.blockchain, self.network, self.account
        ))?;

        Ok(())
    }
}
