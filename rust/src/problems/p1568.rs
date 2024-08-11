pub struct Solution;

impl Solution {
    pub fn min_days(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut [Vec<i32>], i: i32, j: i32) {
            if i < 0
                || j < 0
                || i == grid.len() as i32
                || j == grid[0].len() as i32
                || grid[i as usize][j as usize] != 1
            {
                return;
            }
            grid[i as usize][j as usize] = 2;
            dfs(grid, i + 1, j);
            dfs(grid, i - 1, j);
            dfs(grid, i, j + 1);
            dfs(grid, i, j - 1);
        }

        fn count(grid: &mut [Vec<i32>]) -> i32 {
            let mut result = 0;
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if grid[i][j] == 1 {
                        result += 1;
                        dfs(grid, i as i32, j as i32);
                    }
                }
            }
            for r in grid.iter_mut() {
                for c in r.iter_mut() {
                    if *c == 2 {
                        *c = 1;
                    }
                }
            }
            result
        }

        if count(&mut grid) != 1 {
            return 0;
        }
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    grid[i][j] = 0;
                    if count(&mut grid) != 1 {
                        return 1;
                    }
                    grid[i][j] = 1;
                }
            }
        }
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(2, Solution::min_days(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(2, Solution::min_days(grid));
    }
}
