use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DeriveAddressResp {
    pub address: String,
}



#[derive(Serialize, Deserialize)]
pub enum QueryMsg {
    DeriveAddress {
        public_key: String,
        address_type: String,
        testnet: bool,
        chain: String
    },
}