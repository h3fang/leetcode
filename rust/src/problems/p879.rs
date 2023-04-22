pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let m = group.len();
        let mut dp = vec![[[0; 101]; 101]; 101];
        dp[m].iter_mut().for_each(|a| a[min_profit as usize] = 1);
        for i in (0..m).rev() {
            for c in 0..=n as usize {
                for p in 0..=min_profit as usize {
                    dp[i][c][p] = dp[i + 1][c][p];
                    if c as i32 + group[i] <= n {
                        dp[i][c][p] = (dp[i][c][p]
                            + dp[i + 1][c + group[i] as usize]
                                [min_profit.min(p as i32 + profit[i]) as usize])
                            % MOD;
                    }
                }
            }
        }
        dp[0][0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::profitable_schemes(5, 3, vec![2, 2], vec![2, 3])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            7,
            Solution::profitable_schemes(10, 5, vec![2, 3, 5], vec![6, 7, 8])
        );
    }
}
