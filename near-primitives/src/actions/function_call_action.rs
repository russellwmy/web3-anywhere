use core::fmt;

use borsh::{BorshDeserialize, BorshSerialize};

use super::Action;
use crate::{
    logging,
    serialize::{base64_format, dec_format},
    types::{Balance, Gas},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct FunctionCallAction {
    pub method_name: String,
    #[serde(with = "base64_format")]
    pub args: Vec<u8>,
    pub gas: Gas,
    #[serde(with = "dec_format")]
    pub deposit: Balance,
}

impl From<FunctionCallAction> for Action {
    fn from(function_call_action: FunctionCallAction) -> Self {
        Self::FunctionCall(function_call_action)
    }
}

impl fmt::Debug for FunctionCallAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionCallAction")
            .field("method_name", &format_args!("{}", &self.method_name))
            .field(
                "args",
                &format_args!("{}", logging::pretty_utf8(&self.args)),
            )
            .field("gas", &format_args!("{}", &self.gas))
            .field("deposit", &format_args!("{}", &self.deposit))
            .finish()
    }
}
