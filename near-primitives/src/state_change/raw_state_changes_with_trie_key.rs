use borsh::{BorshDeserialize, BorshSerialize};

use super::RawStateChange;
use crate::trie_key::TrieKey;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct RawStateChangesWithTrieKey {
    pub trie_key: TrieKey,
    pub changes: Vec<RawStateChange>,
}
