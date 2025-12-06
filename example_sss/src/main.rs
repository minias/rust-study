mod sss;
mod seed;
use hex;

fn main() {
    // 향후 BIP-32/44 등 이용하여 시드를 만들수있음.
    // 또한 Salt등을 첨부하여 다양한 시드를 생성 가능.
    let secret = seed::generate();

    println!("==================================================");
    println!("🟦 [원본 암호키]");
    println!("{}", hex::encode(&secret));
    println!("==================================================\n");

    let shares = sss::split(&secret, 3, 2).unwrap();

    println!("🟩 [생성된 Shares - 총 3개]");
    for (i, s) in shares.iter().enumerate() {
        println!("  share[{}] (len={}): {}", i, s.len(), hex::encode(s));
    }
    println!("==================================================\n");
    println!("🟨 [복호화에 사용할 Share 2개]");
    println!("  share[0]: {}", hex::encode(shares[0].clone()));
    println!("  share[1]: {}", hex::encode(shares[1].clone()));    
    println!("  share[2]: {}", hex::encode(shares[2].clone()));
    println!("==================================================\n");

    let selected1 = vec![shares[0].clone(), shares[1].clone()];    
    let selected2 = vec![shares[0].clone(), shares[2].clone()];        
    let selected3 = vec![shares[1].clone(), shares[2].clone()];            
    // 4) 복호화 수행
    let restored1 = sss::combine(&selected1)
        .expect("복원 실패")
        .expect("threshold 부족");
    let restored2 = sss::combine(&selected2)
        .expect("복원 실패")
        .expect("threshold 부족");
    let restored3 = sss::combine(&selected3)
        .expect("복원 실패")
        .expect("threshold 부족");

    println!("🟪 [복호화된 시크릿]");
    println!("{}", hex::encode(&restored1)); // 0,1 검증키
    println!("{}", hex::encode(&restored2)); // 0,2 검증키
    println!("{}", hex::encode(&restored3)); // 1,2 검증키
    println!("==================================================\n");

}
