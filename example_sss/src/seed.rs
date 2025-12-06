use rand::RngCore;
const DATA_SIZE: usize = 64; // sss-rs 의존을 제거하기위해 별도 사용
const DATA_SIZE_128: usize = 128; // 128바이트용 시드 사이즈

/// 64바이트 고정 길이 시크릿 생성
pub fn generate() -> Vec<u8> {
    let mut secret = vec![0u8; DATA_SIZE as usize];
    rand::thread_rng().fill_bytes(&mut secret);
    secret
}

/// 128바이트 랜덤 시드 생성
#[allow(dead_code)]
pub fn generate_128() -> Vec<u8> {
    let mut seed = vec![0u8; DATA_SIZE_128];
    rand::thread_rng().fill_bytes(&mut seed);
    seed
}