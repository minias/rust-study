// src/cli.rs
// CLI 파싱, 에러 처리 및 도움말 구현

use std::env;
use example3::constants::Constants; // 상수 구조체 선언

pub fn parse_args() -> Result<(String, usize), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Err("인자가 부족합니다.".to_string());
    }

    let day = args[1].clone();
    if !Constants::DAYS.contains(&day.as_str()) {
        return Err(format!("지원되지 않는 요일입니다: {}", day));
    }

    let k: usize = match args[2].parse() {
        Ok(v) => {
            if v == 0 {
                return Err("일수(K)는 0보다 커야 합니다.".to_string());
            }
            if v > Constants::MAX_K {
                return Err(format!("일수(K)는 {} 이하이어야 합니다.", Constants::MAX_K));
            }
            v
        }
        Err(_) => return Err("일수(K)는 정수여야 합니다.".to_string()),
    };

    Ok((day, k))
}

pub fn print_help() {
    println!("사용법: cargo run <요일> <일수>");
    println!("예시: cargo run Wed 2");
    println!("빌드파일 실행: example3 <요일> <일수>");
    println!("예시: example3 Wed 2");    
    println!("지원 요일 목록: {:?}", Constants::DAYS);
    println!("일수(K) 최소값: 1, 최대값: {}", Constants::MAX_K);
}