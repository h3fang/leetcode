pub struct Solution;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let primes = 0b10100010100010101100;
        let mut result = 0;
        for n in left..=right {
            let d = n.count_ones();
            if (primes >> d) & 1 == 1 {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::count_prime_set_bits(6, 10));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::count_prime_set_bits(10, 15));
    }
}
