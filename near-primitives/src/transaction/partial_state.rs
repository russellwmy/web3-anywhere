use borsh::{BorshDeserialize, BorshSerialize};

use super::StateItem;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct PartialState(pub Vec<StateItem>);
