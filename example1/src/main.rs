// src/main.rs
// use 를 사용하기 위해선 다음과 같이 선행 작업이 필요하다.
// cargo new exam1
// 위의 명령어 입력후 Cargo.toml이 자동생성된다.
use exam1::max_even_sum;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: <numbers...> <K>");
        std::process::exit(1);
    }

    // K는 마지막 인자
    let k: usize = args.last().unwrap().parse().expect("K must be a number");

    // 배열은 나머지 인자
    let a: Vec<i32> = args[1..args.len()-1]
        .iter()
        .map(|s| s.parse().expect("All numbers must be integers"))
        .collect();

    let result = max_even_sum(a, k);
    println!("{}", result);
}
