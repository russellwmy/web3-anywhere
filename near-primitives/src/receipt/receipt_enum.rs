use borsh::{BorshDeserialize, BorshSerialize};

use super::{ActionReceipt, DataReceipt};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ReceiptEnum {
    Action(ActionReceipt),
    Data(DataReceipt),
}
