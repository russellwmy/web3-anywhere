#![allow(clippy::integer_arithmetic)]
#[macro_use]
extern crate serde_derive;

use web3_anywhere_crypto as crypto;
use web3_anywhere_json_rpc_client as json_rpc_client;
use web3_anywhere_key_man as key_man;
#[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
pub mod browser;

mod account;
mod account_authorized_app;
mod account_balance;
mod auth_data;
mod near_config;
mod near_rpc_client;
mod near_rpc_user;
mod url_account_creator;
mod wallet;

pub use account::Account;
pub use account_authorized_app::AccountAuthorizedApp;
pub use account_balance::AccountBalance;
pub use near_config::NearConfig;
pub use near_rpc_client::NearRpcClient;
pub use near_rpc_user::NearRpcUser;
pub use wallet::Wallet;
pub use web3_anywhere_near_primitives as primitives;
