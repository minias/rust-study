// tests/day_after_test.rs
// example3 라이브러리의 day_after 함수에 대한 통합 테스트
// cargo test 시 자동으로 실행됩니다.

use example3::day_after;

#[test]
fn test_day_after_basic() {
    assert_eq!(day_after("Wed", 2), "Fri");
    assert_eq!(day_after("Sat", 23), "Mon");
}

#[test]
fn test_day_after_edge_cases() {
    assert_eq!(day_after("Mon", 0), "Mon");
    assert_eq!(day_after("Sun", 7), "Sun");
    assert_eq!(day_after("Tue", 14), "Tue");
}
