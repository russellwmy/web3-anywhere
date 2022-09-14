use borsh::{BorshDeserialize, BorshSerialize};

use super::Action;

/// Create account action
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct CreateAccountAction {}

impl From<CreateAccountAction> for Action {
    fn from(create_account_action: CreateAccountAction) -> Self {
        Self::CreateAccount(create_account_action)
    }
}
