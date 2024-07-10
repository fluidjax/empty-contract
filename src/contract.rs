use crate::msg::{DeriveAddressResp, QueryMsg};
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, StdError
};

use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use bech32::{ToBase32, Variant, encode};
extern crate bs58;

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        DeriveAddress { public_key, address_type, testnet, chain } => to_json_binary(&query::derive_address(public_key.as_str(), address_type.as_str(), testnet, chain.as_str())?),
    }
}

pub mod query {
    use std::ascii::AsciiExt;
    use crate::bitcoin::*;
    use crate::msg::DeriveAddressResp;
    use super::*;


    /* Usage
    This is an quick start to get this contract into the chain and test it
    ignite chain serve --reset-once

    export NODE=(--node http://127.0.0.1:26657)
    export TXFLAG=($NODE --chain-id zenrock --gas-prices 0.25urock --gas auto --gas-adjustment 1.3)
    export INIT="{}"
    export CODE_ID=1

    //Build the code
    cargo wasm
    docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer:0.16.0

    //Store in the chain
    zenrockd tx wasm store artifacts/empty_contract.wasm --from alice $TXFLAG -y  -b sync

    //Instantiate an instance
    zenrockd tx wasm instantiate 1 $INIT --from alice --label "name service" --no-admin $TXFLAG -y

    //Get the contract ID
    CONTRACT=$(zenrockd query wasm list-contract-by-code $CODE_ID $NODE --output json | jq -r '.contracts[-1]')

    //run the Query function
    zenrockd query wasm contract-state smart $CONTRACT '{"Greet":{}}'
    zenrockd query wasm contract-state smart $CONTRACT '{"Greet":{"public_key": "021004687a9e5b290b55383eaffa7fd41ce59a27d96d34e7be71e3d85910d0649c", "address_type": "Bitcoin"}}'
     */



    pub fn derive_address(public_key: &str, address_type: &str, testnet: bool, chain: &str) -> StdResult<DeriveAddressResp> {
        //"021004687a9e5b290b55383eaffa7fd41ce59a27d96d34e7be71e3d85910d0649c";
        let pub_key_hex = public_key.to_string();
        let pub_key_bytes = hex::decode(pub_key_hex).expect("Invalid hex");
        let address;

        match chain.to_ascii_lowercase().as_str(){
           "bitcoin" => {

               match address_type.to_ascii_lowercase().as_str() {
                   "p2pkh" => {
                       address = generate_p2pkh(&pub_key_bytes, testnet);
                   },
                    _ => {
                    return Err(StdError::generic_err("Invalid chain"))
                   }
               }



           },
           _ => {
               return Err(StdError::generic_err("Invalid chain"))
           }
        }



        let resp = DeriveAddressResp {
            address: address.to_owned(),
        };

        Ok(resp)
    }
}



