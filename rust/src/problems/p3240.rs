pub struct Solution;

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut result = 0;
        for i in 0..m / 2 {
            for j in 0..n / 2 {
                let sum = grid[i][j]
                    + grid[m - 1 - i][j]
                    + grid[i][n - 1 - j]
                    + grid[m - 1 - i][n - 1 - j];
                result += sum.min(4 - sum);
            }
        }
        if m % 2 == 1 && n % 2 == 1 {
            result += grid[m / 2][n / 2];
        }

        let (mut ones, mut diff) = (0, 0);
        if m % 2 == 1 {
            let row = &grid[m / 2];
            for (j, &x) in row.iter().enumerate().take(n / 2) {
                if x != row[n - 1 - j] {
                    diff += 1;
                } else {
                    ones += 2 * x;
                }
            }
        }
        if n % 2 == 1 {
            for i in 0..m / 2 {
                if grid[i][n / 2] != grid[m - 1 - i][n / 2] {
                    diff += 1;
                } else {
                    ones += 2 * grid[i][n / 2];
                }
            }
        }
        if diff > 0 {
            result + diff
        } else {
            result + ones % 4
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(3, Solution::min_flips(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0, 1], [0, 1], [0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(2, Solution::min_flips(grid));
    }

    #[test]
    fn case3() {
        let grid = [[1], [1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(2, Solution::min_flips(grid));
    }
}
