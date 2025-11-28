// src/lib.rs

/*
요일은 3자리 문자열로 표현됩니다.
    입력 :
        S (Mon,...Sun) //3자리 문자열
        K (0...500)  // 0~500 정수
    리턴 :
        3자리 문자열 (Mon,...Sun)
*/

/// constants.rs로 분리해서 전역변수,상수등을 관리합니다.
//const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

// constants.rs 파일을 모듈로 선언
pub mod constants;  // pub 추가
// 모듈 안에 있는 구조체 Constants를 가져오기
use crate::constants::Constants;
/*
library (crate)
 └── constants (module)
      └── Constants (struct)
           ├── DAYS
           ├── MAX_K
           └── DAY_SIZE
*/

/// 주어진 요일 문자열 S와 K일 후의 요일을 반환합니다.
pub fn day_after(s: &str, k: usize) -> String {
    // 요일 문자열을 인덱스로 변환
    let current_index = Constants::DAYS
        .iter()
        .position(|&day| day == s)
        .expect("Invalid day string provided");

    // K일 후의 요일 인덱스 계산
    // 결과요일번호 = (현재요일번호 + 정수k % 7) % 7
    let next_index = (current_index + k % Constants::DAY_SIZE) % Constants::DAY_SIZE;

    // 새로운 요일 반환
    Constants::DAYS[next_index].to_string()
}