use std::vec;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let n = obstacle_grid[0].len();
        let mut dp = vec![vec![0; n], vec![0; n]];
        dp[0][0] = 1 - obstacle_grid[0][0];
        for j in 1..n {
            dp[0][j] = if obstacle_grid[0][j] == 1 {
                0
            } else {
                dp[0][j - 1]
            };
        }

        for row in &obstacle_grid[1..] {
            dp[1][0] = if row[0] == 1 { 0 } else { dp[0][0] };
            for j in 1..n {
                if row[j] == 0 {
                    dp[1][j] = dp[0][j] + dp[1][j - 1];
                } else {
                    dp[1][j] = 0;
                }
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
        let grid = [[0, 0, 0], [0, 1, 0], [0, 0, 0]];
        let grid = grid.iter().map(|row| row.to_vec()).collect::<Vec<_>>();
        let n_paths = Solution::unique_paths_with_obstacles(grid);
        assert_eq!(n_paths, 2);
    }

    #[test]
    fn case2() {
        let grid = [[0, 1], [0, 0]];
        let grid = grid.iter().map(|row| row.to_vec()).collect::<Vec<_>>();
        let n_paths = Solution::unique_paths_with_obstacles(grid);
        assert_eq!(n_paths, 1);
    }
}
