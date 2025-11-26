// src/main.rs
use env::load_config;

fn main() {
    let config = load_config().expect("환경설정 로딩 실패");

    println!("DB Host = {}", config.database.host);
}