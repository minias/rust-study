// src/sss.rs
use shamirsecretsharing::{create_shares, combine_shares};
use std::error::Error;

/// 비밀을 n개로 분할 (threshold = k)
pub fn split(secret: &[u8], n: u8, k: u8) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let shares = create_shares(secret, n, k)?;
    Ok(shares)
}

/// shares 를 이용해 복원
pub fn combine(shares: &[Vec<u8>]) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
    let restored = combine_shares(shares)?;
    Ok(restored)
}
