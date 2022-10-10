pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![vec![0; k as usize]; n + 1]; m + 1];
        dp[0][1][0] = 1;
        for (i, r) in grid.iter().enumerate() {
            for (j, e) in r.iter().enumerate() {
                for k1 in 0..k {
                    let k2 = (k1 + e) % k;
                    dp[i + 1][j + 1][k2 as usize] =
                        (dp[i][j + 1][k1 as usize] + dp[i + 1][j][k1 as usize]) % MOD;
                }
            }
        }
        dp[m][n][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[5, 2, 4], [3, 0, 5], [0, 7, 2]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let k = 3;
        assert_eq!(2, Solution::number_of_paths(grid, k));
    }

    #[test]
    fn case2() {
        let grid = [[0, 0]].iter().map(|r| r.to_vec()).collect();
        let k = 5;
        assert_eq!(1, Solution::number_of_paths(grid, k));
    }

    #[test]
    fn case3() {
        let grid = [[7, 3, 4, 9], [2, 3, 6, 2], [2, 3, 7, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let k = 1;
        assert_eq!(10, Solution::number_of_paths(grid, k));
    }
}
