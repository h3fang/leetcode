pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &[Vec<i32>], f: &mut [Vec<i64>], i: usize, j: usize) -> i64 {
            if f[i][j] > -1 {
                return f[i][j];
            }

            let mut result = 1;
            for d in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let i1 = i as i32 + d.0;
                let j1 = j as i32 + d.1;
                if i1 < 0 || j1 < 0 || i1 == grid.len() as i32 || j1 == grid[0].len() as i32 {
                    continue;
                }
                let i1 = i1 as usize;
                let j1 = j1 as usize;
                if grid[i1][j1] > grid[i][j] {
                    result = (result + dfs(grid, f, i1, j1)) % MOD;
                }
            }

            f[i][j] = result;
            result
        }
        let m = grid.len();
        let n = grid[0].len();
        let mut f = vec![vec![-1; n]; m];
        let mut result = 0;
        for i in 0..m {
            for j in 0..n {
                result = (result + dfs(&grid, &mut f, i, j)) % MOD;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 1], [3, 4]];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(8, Solution::count_paths(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1], [2]];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(3, Solution::count_paths(grid));
    }
}
