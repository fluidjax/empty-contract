use bech32::{encode, ToBase32, Variant};
use ripemd::Ripemd160;
use sha2::{Sha256, Digest};

pub fn generate_p2pkh(pub_key: &[u8], testnet: bool) -> String {
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

pub fn generate_p2sh(redeem_script: &[u8], testnet: bool) -> String {
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

pub fn generate_p2wpkh(pub_key: &[u8], testnet: bool) -> String {
    let sha256_hash = Sha256::digest(pub_key);
    let ripemd160_hash = Ripemd160::digest(&sha256_hash);

    // Prepare the scriptPubKey: 0x0014 followed by the RIPEMD-160 hash
    let mut script_pub_key = vec![0x00, 0x14];
    script_pub_key.extend(&ripemd160_hash);

    // Bech32 encoding
    let hrp = if testnet { "tb" } else { "bc" };
    encode(hrp, script_pub_key.to_base32(), Variant::Bech32).expect("Bech32 encoding failed")
}

pub fn generate_p2wsh(redeem_script: &[u8], testnet: bool) -> String {
    let sha256_hash = Sha256::digest(redeem_script);

    // Prepare the scriptPubKey: 0x0020 followed by the SHA-256 hash
    let mut script_pub_key = vec![0x00, 0x20];
    script_pub_key.extend(&sha256_hash);

    // Bech32 encoding
    let hrp = if testnet { "tb" } else { "bc" };
    encode(hrp, script_pub_key.to_base32(), Variant::Bech32).expect("Bech32 encoding failed")
}

pub fn calculate_checksum(data: &[u8]) -> Vec<u8> {
    let sha256_hash1 = Sha256::digest(data);
    let sha256_hash2 = Sha256::digest(&sha256_hash1);
    sha256_hash2[0..4].to_vec()
}
