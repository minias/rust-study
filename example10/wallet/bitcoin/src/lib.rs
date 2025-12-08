/// 비트코인 주소 생성 모듈
/// - P2PKH 방식

use common::{master_to_secp256k1_key};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
use secp256k1::Secp256k1;
use hex;

pub fn generate_btc_address(master_key: &[u8; 32]) -> String {
    let secp = Secp256k1::new();

    let privkey = master_to_secp256k1_key(master_key);
    let pubkey = secp256k1::PublicKey::from_secret_key(&secp, &privkey);

    // 1. SHA256(PUBKEY)
    let sha = Sha256::digest(&pubkey.serialize());

    // 2. RIPEMD160(SHA256)
    let ripe = Ripemd160::digest(&sha);

    // 3. 메인넷 prefix 0x00 붙임
    let mut payload = vec![0x00];
    payload.extend_from_slice(&ripe);

    // 4. double-SHA256 checksum
    let checksum = Sha256::digest(&Sha256::digest(&payload));
    let checksum = &checksum[0..4];

    // 5. Base58Check (여기서는 Hex 형태로만 반환)
    format!("00{}{}", hex::encode(ripe), hex::encode(checksum))
}
