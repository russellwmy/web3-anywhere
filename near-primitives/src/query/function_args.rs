use borsh::{BorshDeserialize, BorshSerialize};
use derive_more::{AsRef as DeriveAsRef, From as DeriveFrom};

#[derive(
    Debug, Clone, PartialEq, Eq, DeriveAsRef, DeriveFrom, BorshSerialize, BorshDeserialize,
)]
#[as_ref(forward)]
pub struct FunctionArgs(pub Vec<u8>);
