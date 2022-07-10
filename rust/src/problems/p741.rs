pub struct Solution;

impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dp = vec![vec![vec![i32::MIN; n]; n]; 2 * n - 1];
        dp[0][0][0] = grid[0][0];
        for k in 1..2 * n - 1 {
            let a = if k + 1 >= n { k + 1 - n } else { 0 };
            let b = (n - 1).min(k);
            for x1 in a..=b {
                let y1 = k - x1;
                if grid[x1][y1] == -1 {
                    continue;
                }
                for x2 in x1..=b {
                    let y2 = k - x2;
                    if grid[x2][y2] == -1 {
                        continue;
                    }
                    let mut max = dp[k - 1][x1][x2];
                    if x1 > 0 {
                        max = max.max(dp[k - 1][x1 - 1][x2]);
                    }
                    if x2 > 0 {
                        max = max.max(dp[k - 1][x1][x2 - 1]);
                    }
                    if x1 > 0 && x2 > 0 {
                        max = max.max(dp[k - 1][x1 - 1][x2 - 1]);
                    }
                    dp[k][x1][x2] = max + grid[x1][y1];
                    if x1 != x2 {
                        dp[k][x1][x2] += grid[x2][y2];
                    }
                }
            }
        }
        dp[2 * n - 2][n - 1][n - 1].max(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[0, 1, -1], [1, 0, -1], [1, 1, 1]];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(5, Solution::cherry_pickup(grid));
    }
}
