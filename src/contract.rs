use crate::msg::{GreetResp, QueryMsg};
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
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
        Greet {} => to_json_binary(&query::greet()?),
    }
}

mod query {
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


fn generate_p2pkh(pub_key: &[u8], testnet: bool) -> String {
    let sha256_hash = Sha256::digest(pub_key);
    let ripemd160_hash = Ripemd160::digest(&sha256_hash);

    // Version byte for mainnet or testnet
    let version_byte = if testnet { 0x6F } else { 0x00 };

    // Prepare the address: version byte + RIPEMD-160 hash
    let mut address_bytes = vec![version_byte];
    address_bytes.extend(&ripemd160_hash);

    // Calculate checksum
    let checksum = calculate_checksum(&address_bytes);
    address_bytes.extend(&checksum);

    // Base58 encoding
    bs58::encode(address_bytes).into_string()
}

fn generate_p2sh(redeem_script: &[u8], testnet: bool) -> String {
    let sha256_hash = Sha256::digest(redeem_script);
    let ripemd160_hash = Ripemd160::digest(&sha256_hash);

    // Version byte for mainnet or testnet
    let version_byte = if testnet { 0xC4 } else { 0x05 };

    // Prepare the address: version byte + RIPEMD-160 hash
    let mut address_bytes = vec![version_byte];
    address_bytes.extend(&ripemd160_hash);

    // Calculate checksum
    let checksum = calculate_checksum(&address_bytes);
    address_bytes.extend(&checksum);

    // Base58 encoding
    bs58::encode(address_bytes).into_string()
}

fn generate_p2wpkh(pub_key: &[u8], testnet: bool) -> String {
    let sha256_hash = Sha256::digest(pub_key);
    let ripemd160_hash = Ripemd160::digest(&sha256_hash);

    // Prepare the scriptPubKey: 0x0014 followed by the RIPEMD-160 hash
    let mut script_pub_key = vec![0x00, 0x14];
    script_pub_key.extend(&ripemd160_hash);

    // Bech32 encoding
    let hrp = if testnet { "tb" } else { "bc" };
    encode(hrp, script_pub_key.to_base32(), Variant::Bech32).expect("Bech32 encoding failed")
}

fn generate_p2wsh(redeem_script: &[u8], testnet: bool) -> String {
    let sha256_hash = Sha256::digest(redeem_script);

    // Prepare the scriptPubKey: 0x0020 followed by the SHA-256 hash
    let mut script_pub_key = vec![0x00, 0x20];
    script_pub_key.extend(&sha256_hash);

    // Bech32 encoding
    let hrp = if testnet { "tb" } else { "bc" };
    encode(hrp, script_pub_key.to_base32(), Variant::Bech32).expect("Bech32 encoding failed")
}

fn calculate_checksum(data: &[u8]) -> Vec<u8> {
    let sha256_hash1 = Sha256::digest(data);
    let sha256_hash2 = Sha256::digest(&sha256_hash1);
    sha256_hash2[0..4].to_vec()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_addresses() {
        let pub_key_hex = "021004687a9e5b290b55383eaffa7fd41ce59a27d96d34e7be71e3d85910d0649c";
        let pub_key_bytes = hex::decode(pub_key_hex).expect("Invalid hex");

        let p2pkh_mainnet = generate_p2pkh(&pub_key_bytes, false);
        println!("P2PKH Mainnet Address: {}", p2pkh_mainnet);

        let p2pkh_testnet = generate_p2pkh(&pub_key_bytes, true);
        println!("P2PKH Testnet Address: {}", p2pkh_testnet);

        let redeem_script_hex = "a914748284390f9e263a4b766a75d0633c50426eb87587";
        let redeem_script_bytes = hex::decode(redeem_script_hex).expect("Invalid hex");

        let p2sh_mainnet = generate_p2sh(&redeem_script_bytes, false);
        println!("P2SH Mainnet Address: {}", p2sh_mainnet);

        let p2sh_testnet = generate_p2sh(&redeem_script_bytes, true);
        println!("P2SH Testnet Address: {}", p2sh_testnet);

        let p2wpkh_mainnet = generate_p2wpkh(&pub_key_bytes, false);
        println!("P2WPKH Mainnet Address: {}", p2wpkh_mainnet);

        let p2wpkh_testnet = generate_p2wpkh(&pub_key_bytes, true);
        println!("P2WPKH Testnet Address: {}", p2wpkh_testnet);

        let p2wsh_mainnet = generate_p2wsh(&redeem_script_bytes, false);
        println!("P2WSH Mainnet Address: {}", p2wsh_mainnet);

        let p2wsh_testnet = generate_p2wsh(&redeem_script_bytes, true);
        println!("P2WSH Testnet Address: {}", p2wsh_testnet);
    }
}