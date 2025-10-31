pub struct Solution;

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut row = 0;
        for r in &grid {
            for (i, &x) in r.iter().enumerate().take(n / 2) {
                if x != r[n - 1 - i] {
                    row += 1;
                }
            }
        }

        let mut col = 0;

        #[allow(clippy::needless_range_loop)]
        for j in 0..n {
            for i in 0..m / 2 {
                if grid[i][j] != grid[m - 1 - i][j] {
                    col += 1;
                }
            }
        }

        row.min(col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 0, 0], [0, 0, 0], [0, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(2, Solution::min_flips(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0, 1], [0, 1], [0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(1, Solution::min_flips(grid));
    }

    #[test]
    fn case3() {
        let grid = [[1], [0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(0, Solution::min_flips(grid));
    }
}
