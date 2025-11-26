// src/main.rs
// use 를 사용하기 위해선 다음과 같이 선행 작업이 필요하다.
// cargo new exam1
// 위의 명령어 입력후 Cargo.toml이 자동생성된다.
use exam1::max_even_sum;

/// 이것은 러스트 문서화를 위한 내용입니다.
///
/// 핵심로직은 lib.rs에 있는 max_even_sum() 함수입니다.
///
/// ```
/// fn main() {
///     // cli에서 인자값을 3개 이상 받지 못하면 종료
///     let args: Vec<String> = std::env::args().collect(); 
///     if args.len() < 3 {
///         eprintln!("Usage: <numbers...> <K>");
///         std::process::exit(1);
///     }
///     // K는 마지막 인자
///     let k: usize = args.last().unwrap().parse().expect("K must be a number");
///     // ...
///     let result = max_even_sum(a, k);
///     println!("{}", result);
/// }
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        // eprintln!() 오직 오류일때 사용        
        eprintln!("Usage: <numbers...> <K>");
        std::process::exit(1);
    }

    // K는 마지막 인자
    let k: usize = args.last().unwrap().parse().expect("K must be a number");

    // 배열은 나머지 인자 <i32> 형타입은 숫자 기본.
    let a: Vec<i32> = args[1..args.len()-1] // args[1..] 표시는 배열 시작과 끝 표시
        .iter()
        .map(|s| s.parse().expect("All numbers must be integers"))
        .collect();

    let result = max_even_sum(a, k);
    // println!() 는 표준 출력, "{}" 는 외부 인자를 받아 출력
    println!("{}", result);
}
