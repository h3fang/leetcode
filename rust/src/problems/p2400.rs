pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![vec![0; 3003]; k + 1];
        dp[0][(start_pos + 1001) as usize] = 1;
        for i in 1..=k {
            for p in -1000..2001 {
                let j = (p + 1001) as usize;
                if dp[i - 1][j] > 0 {
                    dp[i][j + 1] = (dp[i][j + 1] + dp[i - 1][j]) % MOD;
                    dp[i][j - 1] = (dp[i][j - 1] + dp[i - 1][j]) % MOD;
                }
            }
        }
        dp[k][(end_pos + 1001) as usize] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::number_of_ways(1, 2, 3));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::number_of_ways(2, 5, 10));
    }
}
