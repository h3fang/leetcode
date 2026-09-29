pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn minimum_possible_sum(n: i32, target: i32) -> i32 {
        let (n, target) = (n as i64, target as i64);
        let t = target / 2;
        let sum = if n <= t {
            (1 + n) * n / 2
        } else {
            (1 + t) * t / 2 + (2 * target + n - t - 1) * (n - t) / 2
        };
        (sum % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::minimum_possible_sum(2, 3));
    }

    #[test]
    fn case2() {
        assert_eq!(8, Solution::minimum_possible_sum(3, 3));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::minimum_possible_sum(1, 1));
    }
}
