use crate::msg::{GreetResp, QueryMsg};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use bech32::{ToBase32, Variant, encode};
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
        Greet {} => to_binary(&query::greet()?),
    }
}

mod query {
    use super::*;


    /* Usage
    start chain

    export NODE=(--node http://127.0.0.1:26657)
    export TXFLAG=($NODE --chain-id zenrock --gas-prices 0.25urock --gas auto --gas-adjustment 1.3)
    export INIT="{}"
    export CODE_ID=1
    cargo wasm
    docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer:0.16.0
    zenrockd tx wasm store artifacts/empty_contract.wasm --from alice $TXFLAG -y  -b sync
    zenrockd tx wasm instantiate 1 $INIT --from alice --label "name service" --no-admin $TXFLAG -y
    CONTRACT=$(zenrockd query wasm list-contract-by-code $CODE_ID $NODE --output json | jq -r '.contracts[-1]')
    zenrockd query wasm contract-state smart $CONTRACT '{"Greet":{}}'
     */



    pub fn greet() -> StdResult<GreetResp> {
        let pub_key_hex = "021004687a9e5b290b55383eaffa7fd41ce59a27d96d34e7be71e3d85910d0649c";
        let pub_key_bytes = hex::decode(pub_key_hex).expect("Invalid hex");
        // Perform SHA-256 hashing on the public key
        let sha256_hash = Sha256::digest(&pub_key_bytes);
        // Perform RIPEMD-160 hashing on the SHA-256 hash
        let ripemd160_hash = Ripemd160::digest(&sha256_hash);
        // Prepare the scriptPubKey: 0x0014 followed by the RIPEMD-160 hash
        let mut script_pub_key = vec![0x00, 0x14];
        script_pub_key.extend(&ripemd160_hash);

        // Encode the scriptPubKey using Bech32
        let address = encode("bc", script_pub_key.to_base32(), Variant::Bech32).expect("Bech32 encoding failed");
        let resp = GreetResp {


            message: address.to_owned(),
        };

        Ok(resp)
    }
}