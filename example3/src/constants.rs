// src/constants.rs

/// 다음과 같이 필요한곳에서 사용선언을 하고 사용하면 됩니다.
/// use example3::constants;
/// println!("{:?}", Constants::DAYS);
/// println!("{}", Constants::MAX_K);
pub struct Constants;

impl Constants {
    /// 배열 인덱스를 통해 계산을 단순화합니다.
    pub const DAYS: [&'static str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    pub const MAX_K: usize = 500; // 최대 사이즈
    pub const DAY_SIZE: usize = Self::DAYS.len(); // 요일 사이즈 자동 계산
}
