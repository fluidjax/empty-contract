
#[cfg(test)]
mod tests {
    use crate::bitcoin::*;
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