pub struct Solution;

impl Solution {
    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut [Vec<i32>], i: usize, j: usize) {
            if grid[i][j] == 1 {
                grid[i][j] = 0;
                if i > 0 {
                    dfs(grid, i - 1, j);
                }
                if i + 1 < grid.len() {
                    dfs(grid, i + 1, j);
                }
                if j > 0 {
                    dfs(grid, i, j - 1);
                }
                if j + 1 < grid[0].len() {
                    dfs(grid, i, j + 1);
                }
            }
        }

        for i in [0, grid.len() - 1] {
            for j in 0..grid[0].len() {
                dfs(&mut grid, i, j);
            }
        }

        for j in [0, grid[0].len() - 1] {
            for i in 1..grid.len() - 1 {
                dfs(&mut grid, i, j);
            }
        }

        grid.iter().flatten().filter(|c| **c == 1).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[0, 0, 0, 0], [1, 0, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(3, Solution::num_enclaves(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0, 1, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(0, Solution::num_enclaves(grid));
    }
}
