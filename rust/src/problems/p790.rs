pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        match n as usize {
            1 => 1,
            2 => 2,
            n => {
                let mut dp = vec![0i64; n + 1];
                dp[0] = 1;
                dp[1] = 1;
                dp[2] = 2;
                for i in 3..=n {
                    dp[i] = (2 * dp[i - 1] + dp[i - 3]) % MOD;
                }
                dp[n] as i32
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(24, Solution::num_tilings(5));
    }

    #[test]
    fn case2() {
        assert_eq!(190242381, Solution::num_tilings(100));
    }

    #[test]
    fn case3() {
        assert_eq!(5, Solution::num_tilings(3));
    }
}
