pub struct Solution;

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let mut result = i64::MAX;
        let mut top: i64 = grid[0].iter().map(|&x| x as i64).sum();
        let mut bottom = 0;
        for (i, &x) in grid[0].iter().enumerate() {
            top -= x as i64;
            result = top.max(bottom).min(result);
            bottom += grid[1][i] as i64;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[2, 5, 4], [1, 5, 1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(4, Solution::grid_game(grid));
    }

    #[test]
    fn case2() {
        let grid = [[3, 3, 1], [8, 5, 2]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(4, Solution::grid_game(grid));
    }

    #[test]
    fn case3() {
        let grid = [[1, 3, 1, 15], [1, 3, 3, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(7, Solution::grid_game(grid));
    }
}
