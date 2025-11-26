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

    if total & 1 == 0 {
        return total;
    }

    // chosen → 최소값 찾아야 하므로 rev = true
    let (min_even_in_chosen, min_odd_in_chosen) = find_parity_targets(chosen, true);
    // others → 최대값 찾아야 하므로 rev = false
    let (max_even_in_others, max_odd_in_others) = find_parity_targets(others, false);

    let mut best = -1;

    if let (Some(ch_odd), Some(oth_even)) = (min_odd_in_chosen, max_even_in_others) {
        if let Some(val) = try_swap(total, ch_odd, oth_even) {
            best = best.max(val);
        }
    }

    if let (Some(ch_even), Some(oth_odd)) = (min_even_in_chosen, max_odd_in_others) {
        if let Some(val) = try_swap(total, ch_even, oth_odd) {
            best = best.max(val);
        }
    }

    best
}

// 최적화로 공통 스왑 함수 생성
// 두 교환 케이스 중 유효한 최대 짝수 합을 계산
#[inline]
fn try_swap(total: i32, out: i32, inn: i32) -> Option<i32> {
    // 짝수 합 조건: (total - out + inn) % 2 == 0 ⇔ (out % 2) == (inn % 2)
    // => modulo 연산 제거 가능 (branch + bitwise)
    if (out & 1) == (inn & 1) {
        Some(total - out + inn)
    } else {
        None
    }
}

/// 정렬된 slice에서 parity 타겟을 찾음.
/// rev == true  -> slice.iter().rev() : "작은 값부터" 탐색 (chosen)
/// rev == false -> slice.iter()     : "큰 값부터" 탐색 (others)
#[inline]
fn find_parity_targets(slice: &[i32], rev: bool) -> (Option<i32>, Option<i32>) {
    let mut even: Option<i32> = None;
    let mut odd: Option<i32> = None;

    let iter: Box<dyn Iterator<Item=&i32>> = if rev {
        Box::new(slice.iter().rev())
    } else {
        Box::new(slice.iter())
    };

    for &v in iter {
        if v & 1 == 0 && even.is_none() {
            even = Some(v);
        } else if v & 1 == 1 && odd.is_none() {
            odd = Some(v);
        }

        if even.is_some() && odd.is_some() {
            break;
        }
    }

    (even, odd)
}

/// 테스트 코드는 아래처럼 사용
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
