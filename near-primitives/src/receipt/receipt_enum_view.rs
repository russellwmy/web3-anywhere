use borsh::{BorshDeserialize, BorshSerialize};
use near_primitives_core::{
    hash::CryptoHash,
    types::{AccountId, Balance},
};
use web3_anywhere_crypto::PublicKey;

use super::DataReceiverView;
use crate::{
    actions::ActionView,
    serialize::{dec_format, option_base64_format},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ReceiptEnumView {
    Action {
        signer_id: AccountId,
        signer_public_key: PublicKey,
        #[serde(with = "dec_format")]
        gas_price: Balance,
        output_data_receivers: Vec<DataReceiverView>,
        input_data_ids: Vec<CryptoHash>,
        actions: Vec<ActionView>,
    },
    Data {
        data_id: CryptoHash,
        #[serde(with = "option_base64_format")]
        data: Option<Vec<u8>>,
    },
}
