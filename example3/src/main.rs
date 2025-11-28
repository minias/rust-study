// src/main.rs
// example3 크레이트 실행용 바이너리 엔트리포인트
// day_after 함수를 사용해 간단한 실행 결과를 확인할 수 있습니다.
mod cli;
use cli::{parse_args, print_help};
use example3::day_after;

fn main() {
    let (day, k) = match parse_args() {
        Ok(values) => values,
        Err(err_msg) => {
            eprintln!("{}", err_msg);
            print_help();
            return;
        }
    };

    let result = day_after(&day, k);
    println!("{} + {} days = {}", day, k, result);
}
