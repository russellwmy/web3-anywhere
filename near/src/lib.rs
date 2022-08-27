#![allow(clippy::integer_arithmetic)]
#[macro_use]
extern crate serde_derive;

use web3_anywhere_crypto as crypto;
use web3_anywhere_key_man as key_man;

#[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
pub mod browser;

mod account;
mod account_authorized_app;
mod account_balance;
mod action;
mod auth_data;
mod transaction;
mod transaction_builder;
mod url_account_creator;
mod wallet;
mod wallet_config;

pub use account::Account;
pub use account_authorized_app::AccountAuthorizedApp;
pub use account_balance::AccountBalance;
pub use action::{
    Action,
    AddKeyAction,
    CreateAccountAction,
    DeleteAccountAction,
    DeleteKeyAction,
    DeployContractAction,
    FunctionCallAction,
    StakeAction,
    TransferAction,
};
pub use near_primitives_core as primitives;
pub use transaction::Transaction;
pub use transaction_builder::TransactionBuilder;
pub use wallet::Wallet;
pub use wallet_config::WalletConfig;
