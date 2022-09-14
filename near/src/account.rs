use crate::{near_rpc_user::NearRpcUser, primitives::types::AccountId};

#[derive(Clone, PartialEq, Debug)]
pub struct Account {
    account_id: AccountId,
    near_rpc_user: NearRpcUser,
}

impl Account {
    pub fn new(account_id: AccountId, near_rpc_user: NearRpcUser) -> Self {
        Self {
            account_id,
            near_rpc_user,
        }
    }
}
