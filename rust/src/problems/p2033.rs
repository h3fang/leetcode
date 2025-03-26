pub struct Solution;

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut grid = grid.into_iter().flatten().collect::<Vec<_>>();
        if grid.iter().any(|y| (y - grid[0]) % x != 0) {
            return -1;
        }
        grid.sort_unstable();
        let m = grid[grid.len() / 2];
        grid.iter().map(|y| (y - m).abs() / x).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[2, 4], [6, 8]];
        let grid = grid.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(4, Solution::min_operations(grid, 2));
    }

    #[test]
    fn case2() {
        let grid = [[1, 5], [2, 3]];
        let grid = grid.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(5, Solution::min_operations(grid, 1));
    }

    #[test]
    fn case3() {
        let grid = [[1, 2], [3, 4]];
        let grid = grid.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(-1, Solution::min_operations(grid, 2));
    }

    #[test]
    fn case4() {
        let grid = [[931, 128], [639, 712]];
        let grid = grid.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(12, Solution::min_operations(grid, 73));
    }

    #[test]
    fn case5() {
        let grid = [
            [596, 904, 960, 232, 120, 932, 176],
            [372, 792, 288, 848, 960, 960, 764],
            [652, 92, 904, 120, 680, 904, 120],
            [372, 960, 92, 680, 876, 624, 904],
            [176, 652, 64, 344, 316, 764, 316],
            [820, 624, 848, 596, 960, 960, 372],
            [708, 120, 456, 92, 484, 932, 540],
        ];
        let grid = grid.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(473, Solution::min_operations(grid, 28));
    }
}
