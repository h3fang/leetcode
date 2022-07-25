pub struct Solution;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut result = 0;
        for i in 0..n {
            for j in 0..n {
                if (0..n).all(|k| grid[i][k] == grid[k][j]) {
                    result += 1;
                }
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
        let grid = [[3, 2, 1], [1, 7, 6], [2, 7, 7]];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(1, Solution::equal_pairs(grid));
    }
}
