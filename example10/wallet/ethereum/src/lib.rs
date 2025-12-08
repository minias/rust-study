/// 이더리움 주소 생성
/// - Keccak256(pubkey[1..]) 마지막 20바이트

use common::master_to_secp256k1_key;
use secp256k1::Secp256k1;
use sha3::{Digest, Keccak256};
use hex;

pub fn generate_eth_address(master_key: &[u8; 32]) -> String {
    let secp = Secp256k1::new();

    let privkey = master_to_secp256k1_key(master_key);
    let pubkey = secp256k1::PublicKey::from_secret_key(&secp, &privkey);

    let pub_bytes = pubkey.serialize_uncompressed();

    // 첫 바이트 0x04 제거
    let hash = Keccak256::digest(&pub_bytes[1..]);

    // 마지막 20바이트가 주소
    let addr = &hash[12..];

    format!("0x{}", hex::encode(addr))
}
