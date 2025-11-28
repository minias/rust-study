fn count_ones_in_product(a: u32, b: u32) -> u32 {
    let product = a as u64 * b as u64; // 오버플로우 방지
    product.count_ones() // u64 타입의 count_ones() 사용
}

fn main() {
    let a = 3;
    let b = 7;
    let result = count_ones_in_product(a, b);
    println!("A * B의 이진 표현에서 1의 개수: {}", result); // 3 출력
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(count_ones_in_product(3, 7), 3);       // 21 -> 10101 -> 1의 개수 3
        assert_eq!(count_ones_in_product(0, 123456), 0); // 0 -> 0
        assert_eq!(count_ones_in_product(1, 1), 1);      // 1 -> 1
        assert_eq!(count_ones_in_product(100_000_000, 100_000_000), 20); // 10^16 -> 1 비트 수 20
    }
}
