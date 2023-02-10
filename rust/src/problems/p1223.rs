pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0; 6]; n + 1];
        let mut sum = vec![0; n + 1];
        sum[0] = 1;
        for i in 1..=n {
            for (j, &r_max) in roll_max.iter().enumerate() {
                let pos = (i as i32 - r_max - 1).max(0) as usize;
                let sub = ((sum[pos] - dp[pos][j]) % MOD + MOD) % MOD;
                dp[i][j] = ((sum[i - 1] - sub) % MOD + MOD) % MOD;
                if i <= r_max as usize {
                    dp[i][j] = (dp[i][j] + 1) % MOD;
                }
                sum[i] = (sum[i] + dp[i][j]) % MOD;
            }
        }
        sum[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(34, Solution::die_simulator(2, vec![1, 1, 2, 2, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(30, Solution::die_simulator(2, vec![1, 1, 1, 1, 1, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(181, Solution::die_simulator(3, vec![1, 1, 1, 2, 2, 3]));
    }
}
