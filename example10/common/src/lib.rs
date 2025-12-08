/// common crate - HD Wallet 기반 모듈
/// Rust 2024 edition + bip39 2.x API 대응

use bip39::{Language, Mnemonic};
use rand::RngCore;
use sha2::{Sha256, Digest};
use secp256k1::SecretKey;

/// 24단어 니모닉 생성
pub fn generate_mnemonic() -> Mnemonic {
    // 24단어 = 256bit entropy (32바이트)
    let mut entropy = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut entropy);

    Mnemonic::from_entropy_in(Language::English, &entropy)
        .expect("entropy invalid")
}

/// 니모닉 → 시드 (Seed 타입 없음 → Vec<u8> 반환)
pub fn mnemonic_to_seed(m: &Mnemonic) -> Vec<u8> {
    // bip39 2.x: to_seed() → [u8; 64]
    m.to_seed("").to_vec()     // Vec<u8> 로 변환
}

/// 시드 → 마스터키 (BIP32 아님, 단순 SHA256)
pub fn seed_to_master_key(seed: &[u8]) -> [u8; 32] {
    let hash = Sha256::digest(seed);
    let mut out = [0u8; 32];
    out.copy_from_slice(&hash[..32]);
    out
}

/// Secp256k1 Private Key 생성
pub fn master_to_secp256k1_key(master: &[u8; 32]) -> SecretKey {
    SecretKey::from_slice(master).expect("secret key must be 32 bytes")
}
