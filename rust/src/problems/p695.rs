pub struct Solution;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        fn bt(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
            if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 {
                return 0;
            }

            if grid[i as usize][j as usize] == 1 {
                grid[i as usize][j as usize] = 2;
                1 + bt(grid, i - 1, j)
                    + bt(grid, i + 1, j)
                    + bt(grid, i, j - 1)
                    + bt(grid, i, j + 1)
            } else {
                0
            }
        }

        let mut r = 0;
        for i in 0..grid.len() as i32 {
            for j in 0..grid[0].len() as i32 {
                r = r.max(bt(&mut grid, i, j))
            }
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        let grid = grid.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(6, Solution::max_area_of_island(grid));
    }
}
