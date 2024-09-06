pub struct Solution;

impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        for (i, row) in grid.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if i == j || (i + j == n - 1) {
                    if c == 0 {
                        return false;
                    }
                } else if c != 0 {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[2, 0, 0, 1], [0, 3, 1, 0], [0, 5, 2, 0], [4, 0, 0, 2]];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert!(Solution::check_x_matrix(grid));
    }
}
