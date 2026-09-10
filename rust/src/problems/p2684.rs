pub struct Solution;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut f = vec![vec![true; m]; 2];
        let mut result = 0;
        for j in 0..n - 1 {
            let k = (j + 1) % 2;
            f[k].iter_mut().for_each(|e| *e = false);
            for i in 0..m {
                if !f[1 - k][i] {
                    continue;
                }
                if i > 0 && grid[i - 1][j + 1] > grid[i][j] {
                    f[k][i - 1] = true;
                    result = j + 1;
                }
                if grid[i][j + 1] > grid[i][j] {
                    f[k][i] = true;
                    result = j + 1;
                }
                if i < m - 1 && grid[i + 1][j + 1] > grid[i][j] {
                    f[k][i + 1] = true;
                    result = j + 1;
                }
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[2, 4, 3, 5], [5, 4, 9, 3], [3, 4, 2, 11], [10, 9, 13, 15]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(3, Solution::max_moves(grid));
    }

    #[test]
    fn case2() {
        let grid = [[3, 2, 4], [2, 1, 9], [1, 1, 7]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(0, Solution::max_moves(grid));
    }
}
