pub struct Solution;

use std::collections::HashSet;

fn check_rows(grid: &[Vec<i32>]) -> bool {
    let (m, n) = (grid.len(), grid[0].len());

    let total = grid.iter().flatten().map(|&v| v as i64).sum::<i64>();
    let mut nums = HashSet::with_capacity(m * n);
    let (mut sum, mut max) = (0, 0);

    for (i, r) in grid.iter().enumerate().take(m - 1) {
        for &v in r {
            sum += v as i64;
            max = max.max(v);
        }
        if 2 * sum == total {
            return true;
        }
        if 2 * sum > total + max as i64 {
            break;
        }
        for &x in &grid[i] {
            nums.insert(x);
        }
        let Ok(x) = i32::try_from(2 * sum - total) else {
            continue;
        };

        if n == 1 {
            if x == grid[0][0] || x == grid[i][0] {
                return true;
            }
        } else if i == 0 {
            if x == grid[0][0] || x == grid[0][n - 1] {
                return true;
            }
        } else if nums.contains(&x) {
            return true;
        }
    }
    false
}

fn rotate_90cw(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut ans = vec![vec![0; m]; n];
    for (i, r) in grid.into_iter().enumerate() {
        for (j, c) in r.into_iter().enumerate() {
            ans[j][m - 1 - i] = c;
        }
    }
    ans
}

impl Solution {
    pub fn can_partition_grid(mut grid: Vec<Vec<i32>>) -> bool {
        fn check(grid: &mut [Vec<i32>]) -> bool {
            if check_rows(grid) {
                return true;
            }
            grid.reverse();
            check_rows(grid)
        }

        if check(&mut grid) {
            return true;
        }

        let mut grid = rotate_90cw(grid);

        check(&mut grid)
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
