pub struct Solution;

impl Solution {
    pub fn color_border(mut grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        fn dfs(grid: &mut [Vec<i32>], row: i32, col: i32, color: i32, result: &mut [Vec<i32>]) {
            let mut is_boundary = false;
            let original = grid[row as usize][col as usize];
            grid[row as usize][col as usize] *= -1;
            for (r, c) in [
                (row + 1, col),
                (row - 1, col),
                (row, col + 1),
                (row, col - 1),
            ] {
                if r < 0
                    || r >= grid.len() as i32
                    || c < 0
                    || c >= grid[0].len() as i32
                    || grid[r as usize][c as usize].abs() != original
                {
                    is_boundary = true;
                    continue;
                }
                if grid[r as usize][c as usize] > 0 {
                    dfs(grid, r, c, color, result);
                }
            }
            if is_boundary {
                result[row as usize][col as usize] = color;
            }
        }
        let mut result = grid.clone();
        dfs(&mut grid, row, col, color, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 1], [1, 2]];
        let grid = grid.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        let row = 0;
        let col = 0;
        let color = 3;
        let expected = [[3, 3], [3, 2]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::color_border(grid, row, col, color));
    }

    #[test]
    fn case2() {
        let grid = [[1, 2, 2], [2, 3, 2]];
        let grid = grid.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        let row = 0;
        let col = 1;
        let color = 3;
        let expected = [[1, 3, 3], [2, 3, 3]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::color_border(grid, row, col, color));
    }

    #[test]
    fn case3() {
        let grid = [[1, 1, 1], [1, 1, 1], [1, 1, 1]];
        let grid = grid.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        let row = 1;
        let col = 1;
        let color = 2;
        let expected = [[2, 2, 2], [2, 1, 2], [2, 2, 2]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::color_border(grid, row, col, color));
    }
}
