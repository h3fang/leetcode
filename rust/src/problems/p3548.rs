pub struct Solution;

use std::collections::HashSet;

fn check_rows(grid: &[Vec<i32>]) -> bool {
    let (m, n) = (grid.len(), grid[0].len());
    let mut presum = Vec::with_capacity(m);

    for r in grid {
        let sum: i64 = r.iter().map(|&e| e as i64).sum();
        presum.push(*presum.last().unwrap_or(&0) + sum);
    }

    let total = *presum.last().unwrap();
    let mut nums = HashSet::with_capacity(m * n);

    for (i, &s) in presum.iter().enumerate().take(m - 1) {
        if 2 * s == total {
            return true;
        }
        for &x in &grid[i] {
            nums.insert(x);
        }
        let x = (2 * s - total) as i32;
        if i == 0 {
            if n > 1 && x == grid[0][0] || x == grid[0][n - 1] {
                return true;
            }
        } else if n == 1 {
            if x == grid[0][0] || x == grid[i][0] {
                return true;
            }
        } else if nums.contains(&x) {
            return true;
        }
    }
    false
}

fn check_cols(grid: &[Vec<i32>]) -> bool {
    let (m, n) = (grid.len(), grid[0].len());
    let mut presum = Vec::with_capacity(n);

    for j in 0..grid[0].len() {
        let sum: i64 = grid.iter().map(|r| r[j] as i64).sum();
        presum.push(*presum.last().unwrap_or(&0) + sum);
    }

    let total = *presum.last().unwrap();
    let mut nums = HashSet::with_capacity(m * n);

    for (j, &s) in presum.iter().enumerate().take(n - 1) {
        if 2 * s == total {
            return true;
        }
        for r in grid {
            nums.insert(r[j]);
        }
        let x = (2 * s - total) as i32;
        if j == 0 {
            if m > 1 && x == grid[0][j] || x == grid[m - 1][j] {
                return true;
            }
        } else if m == 1 {
            if x == grid[0][0] || x == grid[0][j] {
                return true;
            }
        } else if nums.contains(&x) {
            return true;
        }
    }
    false
}

impl Solution {
    pub fn can_partition_grid(mut grid: Vec<Vec<i32>>) -> bool {
        if check_cols(&grid) || check_rows(&grid) {
            return true;
        }
        grid.reverse();
        if check_rows(&grid) {
            return true;
        }
        grid.iter_mut().for_each(|r| r.reverse());
        check_cols(&grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 4], [2, 3]].iter().map(|r| r.to_vec()).collect();
        assert!(Solution::can_partition_grid(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 2], [3, 4]].iter().map(|r| r.to_vec()).collect();
        assert!(Solution::can_partition_grid(grid));
    }

    #[test]
    fn case3() {
        let grid = [[1, 2, 4], [2, 3, 5]].iter().map(|r| r.to_vec()).collect();
        assert!(!Solution::can_partition_grid(grid));
    }
}
