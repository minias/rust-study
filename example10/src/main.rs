/// 엔트리포인트: 공통 seed → 각 코인 주소 도출

use common::{generate_mnemonic, mnemonic_to_seed, seed_to_master_key};
use wallet_bitcoin::generate_btc_address;
use wallet_ethereum::generate_eth_address;
use wallet_tron::generate_tron_address;

fn main() {
    // 니모닉 생성
    let mnemonic = generate_mnemonic();
    println!("Mnemonic: {}", mnemonic);

    // Seed 생성
    let seed = mnemonic_to_seed(&mnemonic);

    // Master key 도출
    let master = seed_to_master_key(&seed);

    // 코인별 주소 생성
    println!("BTC: {}", generate_btc_address(&master));
    println!("ETH: {}", generate_eth_address(&master));
    println!("TRON: {}", generate_tron_address(&master));
}
