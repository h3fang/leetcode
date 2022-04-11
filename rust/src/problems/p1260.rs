pub struct Solution;

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let total = m * n;
        let k = k as usize % total;
        let mut result = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let idx = (total + i * n + j - k) % total;
                result[i][j] = grid[idx / n][idx % n];
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
        let grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let k = 1;
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        let expected = [[9, 1, 2], [3, 4, 5], [6, 7, 8]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::shift_grid(grid, k));
    }

    #[test]
    fn case2() {
        let grid = [[3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10], [12, 0, 21, 13]];
        let k = 4;
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        let expected = [[12, 0, 21, 13], [3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::shift_grid(grid, k));
    }
}
