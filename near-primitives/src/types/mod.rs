use crate::hash::CryptoHash;

mod block_id;
mod block_reference;
mod block_shard_id;
mod cost_gas_used;
mod epoch_id;
mod finality;
mod gas_price_view;
mod store_key;
mod store_value;

pub use block_id::BlockId;
pub use block_reference::BlockReference;
pub use block_shard_id::BlockShardId;
pub use chrono::DateTime;
pub use cost_gas_used::CostGasUsed;
pub use epoch_id::EpochId;
pub use finality::Finality;
pub use gas_price_view::GasPriceView;
pub use near_primitives_core::types::*;
pub use store_key::StoreKey;
pub use store_value::StoreValue;

pub type BlockHash = CryptoHash;
pub type LogEntry = String;
pub type StateRoot = CryptoHash;
pub type TrieProofPath = Vec<String>;
pub type MaybeBlockId = Option<BlockId>;
