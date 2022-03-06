pub struct Solution;

impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        const MOD: i64 = 1000000007;
        let mut f = 1i64;
        for i in 2..=n as i64 {
            f = ((f * (2 * i - 1) % MOD) * i) % MOD;
        }
        f as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::count_orders(1));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::count_orders(2));
    }

    #[test]
    fn case3() {
        assert_eq!(90, Solution::count_orders(3));
    }
}
