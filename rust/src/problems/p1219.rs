pub struct Solution;

impl Solution {
    pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut [Vec<i32>], x: usize, y: usize) -> i32 {
            if grid[y][x] > 0 {
                let c = grid[y][x];
                let mut r = 0;
                grid[y][x] = 0;
                if x > 0 {
                    r = r.max(dfs(grid, x - 1, y));
                }
                if x + 1 < grid[0].len() {
                    r = r.max(dfs(grid, x + 1, y));
                }
                if y > 0 {
                    r = r.max(dfs(grid, x, y - 1));
                }
                if y + 1 < grid.len() {
                    r = r.max(dfs(grid, x, y + 1));
                }
                grid[y][x] = c;
                c + r
            } else {
                0
            }
        }

        let mut result = 0;
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                result = result.max(dfs(&mut grid, x, y));
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[0, 6, 0], [5, 8, 7], [0, 9, 0]];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(24, Solution::get_maximum_gold(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 0, 7], [2, 0, 6], [3, 4, 5], [0, 3, 0], [9, 0, 20]];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(28, Solution::get_maximum_gold(grid));
    }
}
