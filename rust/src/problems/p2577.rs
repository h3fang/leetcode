pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1;
        }
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), 0, 0));
        let mut visited = vec![false; m as usize * n as usize];
        while let Some((Reverse(t), i, j)) = q.pop() {
            if (i, j) == (m - 1, n - 1) {
                return t;
            }
            if visited[(i * n + j) as usize] {
                continue;
            }
            visited[(i * n + j) as usize] = true;
            for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                if i1 < 0 || j1 < 0 || i1 >= m || j1 >= n || visited[(i1 * n + j1) as usize] {
                    continue;
                }
                let t1 = grid[i1 as usize][j1 as usize];

                if t + 1 >= t1 {
                    q.push((Reverse(t + 1), i1, j1));
                } else {
                    q.push((Reverse(t1 + 1 - (t1 - t) % 2), i1, j1));
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[0, 1, 3, 2], [5, 1, 2, 5], [4, 3, 8, 6]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(7, Solution::minimum_time(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0, 2, 4], [3, 2, 1], [1, 0, 4]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(-1, Solution::minimum_time(grid));
    }
}
