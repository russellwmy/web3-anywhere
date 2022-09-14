use borsh::{BorshDeserialize, BorshSerialize};
use derive_more::AsRef as DeriveAsRef;

use crate::hash::CryptoHash;

#[derive(
    Debug,
    Clone,
    Default,
    Hash,
    Eq,
    PartialEq,
    PartialOrd,
    DeriveAsRef,
    BorshSerialize,
    BorshDeserialize,
    Serialize,
    Deserialize,
)]
#[as_ref(forward)]
pub struct EpochId(pub CryptoHash);
