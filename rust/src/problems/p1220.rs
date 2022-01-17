pub struct Solution;

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        const MOD: i64 = 10_0000_0007;
        let n = n as usize;
        let mut dp = [1i64; 5];
        for _ in 1..n {
            let mut curr = [0; 5];
            curr[0] = (dp[1] + dp[2] + dp[4]) % MOD;
            curr[1] = (dp[0] + dp[2]) % MOD;
            curr[2] = (dp[1] + dp[3]) % MOD;
            curr[3] = dp[2];
            curr[4] = (dp[2] + dp[3]) % MOD;
            dp = curr;
        }

        (dp.iter().sum::<i64>() % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::count_vowel_permutation(1));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::count_vowel_permutation(2));
    }

    #[test]
    fn case3() {
        assert_eq!(68, Solution::count_vowel_permutation(5));
    }
}
