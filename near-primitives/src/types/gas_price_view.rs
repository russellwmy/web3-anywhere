use crate::{serialize::dec_format, types::Balance};

#[derive(Serialize, Deserialize, Debug)]
pub struct GasPriceView {
    #[serde(with = "dec_format")]
    pub gas_price: Balance,
}
