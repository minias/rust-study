// rust-study/example5/src/main.rs

/// K개의 원소로 만들 수 있는 최대 짝수 합을 계산.
/// 조건을 만족하는 조합이 없다면 -1 반환.
fn max_even_sum(a: &[i32], k: usize) -> i32 {
    // K가 전체 N보다 크면 애초에 선택 불가
    if k > a.len() {
        return -1;
    }

    // 내림차순 정렬
    let mut nums = a.to_vec();
    nums.sort_by(|x, y| y.cmp(x));

    // 짝수/홀수를 분리
    let mut evens: Vec<i32> = nums.iter().cloned().filter(|v| v % 2 == 0).collect();
    let mut odds: Vec<i32>  = nums.iter().cloned().filter(|v| v % 2 == 1).collect();

    // 짝수, 홀수 둘 다 내림차순 정렬(이미 되어있지만 명확성을 위해)
    evens.sort_by(|x, y| y.cmp(x));
    odds.sort_by(|x, y| y.cmp(x));

    // 가능한 모든 "홀수를 짝수개 선택" 조합을 시도
    let mut best_sum = -1;

    // 홀수 개수 y는 짝수, x + y = K
    for y in (0..=k).step_by(2) {
        if y > odds.len() { 
            continue; 
        }
        let x = k - y;
        if x > evens.len() {
            continue;
        }

        // 상위 x개의 짝수 + y개의 홀수 합산
        let sum_x: i32 = evens.iter().take(x).sum();
        let sum_y: i32 = odds.iter().take(y).sum();
        let total = sum_x + sum_y;

        if total % 2 == 0 && total > best_sum {
            best_sum = total;
        }
    }

    best_sum
}

fn main() {
    let samples = vec![
        (vec![4, 9, 8, 2, 6], 3),
        (vec![5, 6, 3, 4, 2], 5),
        (vec![7, 7, 7, 7, 7], 1),
        (vec![2], 2),
        (vec![2, 3, 3, 5, 5], 3),
    ];

    for (arr, k) in samples {
        println!("A={:?}, K={} → {}", arr, k, max_even_sum(&arr, k));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        // 1) A=[4,9,8,2,6], K=3 → 18
        assert_eq!(max_even_sum(&[4, 9, 8, 2, 6], 3), 18);

        // 2) A=[5,6,3,4,2], K=5 → 20
        assert_eq!(max_even_sum(&[5, 6, 3, 4, 2], 5), 20);

        // 3) A=[7,7,7,7,7], K=1 → -1 (짝수 없음)
        assert_eq!(max_even_sum(&[7, 7, 7, 7, 7], 1), -1);

        // 4) A=[2], K=2 → -1 (선택 불가)
        assert_eq!(max_even_sum(&[2], 2), -1);

        // 5) A=[2,3,3,5,5], K=3 → 12
        assert_eq!(max_even_sum(&[2, 3, 3, 5, 5], 3), 12);
    }
}
