/// 트론 주소 생성
/// - Ethereum Address와 동일하게 Keccak256 마지막 20바이트 사용
/// - 단 prefix 0x41 붙임

use common::master_to_secp256k1_key;
use secp256k1::Secp256k1;
use sha3::{Digest, Keccak256};
use hex;

pub fn generate_tron_address(master_key: &[u8; 32]) -> String {
    let secp = Secp256k1::new();

    let privkey = master_to_secp256k1_key(master_key);
    let pubkey = secp256k1::PublicKey::from_secret_key(&secp, &privkey);

    let pub_bytes = pubkey.serialize_uncompressed();
    let hash = Keccak256::digest(&pub_bytes[1..]);

    let addr = &hash[12..];

    // Tron은 0x41 prefix
    format!("41{}", hex::encode(addr))
}
