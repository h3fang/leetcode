pub struct Solution;

impl Solution {
    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize, result: &mut i32) {
            match grid[i][j] {
                2 if grid.iter().flatten().all(|c| *c != 0) => *result += 1,
                0 => {
                    grid[i][j] = -2;
                    if i > 0 {
                        dfs(grid, i - 1, j, result);
                    }
                    if i + 1 < grid.len() {
                        dfs(grid, i + 1, j, result);
                    }
                    if j > 0 {
                        dfs(grid, i, j - 1, result);
                    }
                    if j + 1 < grid[0].len() {
                        dfs(grid, i, j + 1, result);
                    }
                    grid[i][j] = 0;
                }
                _ => {}
            }
        }
        fn find_start(grid: &mut [Vec<i32>]) -> (usize, usize) {
            for (i, row) in grid.iter_mut().enumerate() {
                for (j, c) in row.iter_mut().enumerate() {
                    if *c == 1 {
                        *c = 0;
                        return (i, j);
                    }
                }
            }
            unreachable!()
        }
        let mut result = 0;
        let (i, j) = find_start(&mut grid);
        dfs(&mut grid, i, j, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 2, -1]];
        let grid = grid.iter().map(|row| row.to_vec()).collect::<Vec<_>>();
        let n_paths = Solution::unique_paths_iii(grid);
        assert_eq!(2, n_paths);
    }

    #[test]
    fn case2() {
        let grid = [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 2]];
        let grid = grid.iter().map(|row| row.to_vec()).collect::<Vec<_>>();
        let n_paths = Solution::unique_paths_iii(grid);
        assert_eq!(4, n_paths);
    }

    #[test]
    fn case3() {
        let grid = [[0, 1], [2, 0]];
        let grid = grid.iter().map(|row| row.to_vec()).collect::<Vec<_>>();
        let n_paths = Solution::unique_paths_iii(grid);
        assert_eq!(0, n_paths);
    }
}
