// src/lib.rs

/// K개의 원소를 선택하여 최대 짝수 합을 만드는 함수
/// 짝수 합이 불가능하면 -1 반환
pub fn max_even_sum(mut a: Vec<i32>, k: usize) -> i32 {
    // --- 유효성 체크 ---
    if k == 0 || k > a.len() {
        return -1;
    }

    // 큰 값 우선 사용 → 내림차순 정렬
    a.sort_unstable_by(|x, y| y.cmp(x));

    let chosen = &a[..k];
    let others = &a[k..];

    let total: i32 = chosen.iter().sum();

    // 이미 짝수면 바로 반환
    if total % 2 == 0 {
        return total;
    }

    // 선택된 요소 중: 가장 작은 홀수, 가장 작은 짝수
    let mut min_odd_in_chosen: Option<i32> = None;
    let mut min_even_in_chosen: Option<i32> = None;

    for &v in chosen.iter().rev() {
        if v % 2 == 0 {
            min_even_in_chosen = Some(v);
            break;
        }
    }
    for &v in chosen.iter().rev() {
        if v % 2 != 0 {
            min_odd_in_chosen = Some(v);
            break;
        }
    }

    // 선택되지 않은 요소 중: 가장 큰 짝수, 가장 큰 홀수
    let mut max_even_in_others: Option<i32> = None;
    let mut max_odd_in_others: Option<i32> = None;

    for &v in others.iter() {
        if v % 2 == 0 {
            max_even_in_others = Some(v);
            break;
        }
    }
    for &v in others.iter() {
        if v % 2 != 0 {
            max_odd_in_others = Some(v);
            break;
        }
    }

    let mut best = -1;

    // case 1: (선택된 최소 홀수 ↔ 선택되지 않은 최대 짝수)
    if let (Some(ch_odd), Some(oth_even)) = (min_odd_in_chosen, max_even_in_others) {
        let new_sum = total - ch_odd + oth_even;
        if new_sum % 2 == 0 {
            best = best.max(new_sum);
        }
    }

    // case 2: (선택된 최소 짝수 ↔ 선택되지 않은 최대 홀수)
    if let (Some(ch_even), Some(oth_odd)) = (min_even_in_chosen, max_odd_in_others) {
        let new_sum = total - ch_even + oth_odd;
        if new_sum % 2 == 0 {
            best = best.max(new_sum);
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(max_even_sum(vec![4, 9, 8, 2, 6], 3), 18);
    }

    #[test]
    fn example2() {
        assert_eq!(max_even_sum(vec![5, 6, 3, 4, 2], 5), 20);
    }

    #[test]
    fn example3() {
        assert_eq!(max_even_sum(vec![7, 7, 7, 7, 7], 1), -1);
    }

    #[test]
    fn example4() {
        assert_eq!(max_even_sum(vec![10000], 2), -1);
    }

    #[test]
    fn example5() {
        assert_eq!(max_even_sum(vec![2, 3, 3, 5, 5], 3), 12);
    }

    #[test]
    fn custom_parity_fix() {
        assert_eq!(max_even_sum(vec![9, 8, 4, 3], 3), 21); // 9+8+4 = 21(홀) → swap → 9+8+3 = 20
    }
}