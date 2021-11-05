pub struct Solution;

impl Solution {
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        let mut dp = grid[0].clone();
        for i in 1..n {
            dp[i] += dp[i - 1];
        }

        for row in grid.iter().skip(1) {
            dp[0] += row[0];
            for j in 1..n {
                dp[j] = dp[j - 1].max(dp[j]) + row[j];
            }
        }

        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(12, Solution::max_value(grid));
    }
}
