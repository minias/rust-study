// src/main.rs

// example1 패키지의 lib.rs 에 정의된 max_even_sum() 함수를 사용한다.
use example1::max_even_sum;

/// CLI 인자 기반으로 max_even_sum()을 호출하는 메인 엔트리.
///
/// 사용 예:
/// ```bash
/// ./example1 4 9 8 2 6 3 3
/// # ↑ 숫자들 ... 마지막 숫자(K)
/// ```
///
/// 내부 처리:
/// - 최소 3개 인자 필요 (프로그램명 제외 2개 → 숫자들 + K)
/// - 마지막 인자는 K
/// - 나머지 모든 인자는 i32 리스트
/// - 오류는 eprintln!() 을 사용하여 출력
fn main() {
    // 모든 CLI 인자 (0번은 실행파일 이름)
    let args: Vec<String> = std::env::args().collect();

    // 최소 <숫자 1개 + K> = 2개 이상 필요 → args.len() >= 3
    if args.len() < 3 {
        eprintln!("Usage: <numbers...> <K>");
        std::process::exit(1);
    }

    // 마지막 인자가 K
    let k: usize = match args.last().unwrap().parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("K must be a valid number");
            std::process::exit(1);
        }
    };

    // 나머지는 모두 i32 배열
    let a: Vec<i32> = args[1..args.len() - 1]
        .iter()
        .map(|s| {
            s.parse::<i32>()
                .unwrap_or_else(|_| {
                    eprintln!("All numbers must be integers");
                    std::process::exit(1);
                })
        })
        .collect();

    // 핵심 알고리즘은 lib.rs 의 max_even_sum()
    let result = max_even_sum(a, k);

    // 정상 결과 출력
    println!("{}", result);
}
