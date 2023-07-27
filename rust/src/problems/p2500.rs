pub struct Solution;

impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        grid.iter_mut().for_each(|r| r.sort_unstable());
        let m = grid.len();
        (0..grid[0].len())
            .map(|j| (0..m).map(|i| grid[i][j]).max().unwrap())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 2, 4], [3, 3, 1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(8, Solution::delete_greatest_value(grid));
    }

    #[test]
    fn case2() {
        let grid = [[10]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(10, Solution::delete_greatest_value(grid));
    }
}
