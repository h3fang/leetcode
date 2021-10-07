use std::vec;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        let mut dp = vec![vec![0; n], vec![0; n]];
        dp[0][0] = grid[0][0];
        for j in 1..n {
            dp[0][j] = dp[0][j - 1] + grid[0][j];
        }

        for row in &grid[1..] {
            dp[1][0] = dp[0][0] + row[0];
            for j in 1..n {
                dp[1][j] = row[j] + dp[1][j - 1].min(dp[0][j]);
            }
            dp.swap(0, 1);
        }
        dp[0][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 3, 1], [1, 5, 1], [4, 2, 1]];
        let grid = grid.iter().map(|row| row.to_vec()).collect::<Vec<_>>();
        assert_eq!(Solution::min_path_sum(grid), 7);
    }

    #[test]
    fn case2() {
        let grid = [[1, 2, 3], [4, 5, 6]];
        let grid = grid.iter().map(|row| row.to_vec()).collect::<Vec<_>>();
        assert_eq!(Solution::min_path_sum(grid), 12);
    }
}
