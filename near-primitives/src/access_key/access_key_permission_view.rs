use borsh::{BorshDeserialize, BorshSerialize};

use crate::{
    account::{AccessKeyPermission, FunctionCallPermission},
    serialize::dec_format,
    types::Balance,
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum AccessKeyPermissionView {
    FunctionCall {
        #[serde(with = "dec_format")]
        allowance: Option<Balance>,
        receiver_id: String,
        method_names: Vec<String>,
    },
    FullAccess,
}

impl From<AccessKeyPermission> for AccessKeyPermissionView {
    fn from(permission: AccessKeyPermission) -> Self {
        match permission {
            AccessKeyPermission::FunctionCall(func_call) => AccessKeyPermissionView::FunctionCall {
                allowance: func_call.allowance,
                receiver_id: func_call.receiver_id,
                method_names: func_call.method_names,
            },
            AccessKeyPermission::FullAccess => AccessKeyPermissionView::FullAccess,
        }
    }
}

impl From<AccessKeyPermissionView> for AccessKeyPermission {
    fn from(view: AccessKeyPermissionView) -> Self {
        match view {
            AccessKeyPermissionView::FunctionCall {
                allowance,
                receiver_id,
                method_names,
            } => AccessKeyPermission::FunctionCall(FunctionCallPermission {
                allowance,
                receiver_id,
                method_names,
            }),
            AccessKeyPermissionView::FullAccess => AccessKeyPermission::FullAccess,
        }
    }
}
