pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut ans = 1024;

        let mut vis = HashSet::with_capacity(m * n * 1024);
        let mut q = Vec::with_capacity(m * n * 1024);
        q.push((0i16, 0i16, 0i16));

        let (m, n) = (m as i16, n as i16);

        while let Some((i, j, xor)) = q.pop() {
            let xor = xor ^ grid[i as usize][j as usize] as i16;

            if (i, j) == (m - 1, n - 1) && xor < ans {
                ans = xor;
                if ans == 0 {
                    break;
                }
                continue;
            }

            if i + 1 < m && vis.insert((i + 1, j, xor)) {
                q.push((i + 1, j, xor));
            }
            if j + 1 < n && vis.insert((i, j + 1, xor)) {
                q.push((i, j + 1, xor));
            }
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 2], [3, 4]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(6, Solution::min_cost(grid));
    }

    #[test]
    fn case2() {
        let grid = [[6, 7], [5, 8]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(9, Solution::min_cost(grid));
    }

    #[test]
    fn case3() {
        let grid = [[2, 7, 5]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(0, Solution::min_cost(grid));
    }
}
