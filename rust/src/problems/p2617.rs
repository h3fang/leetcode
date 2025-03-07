pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn minimum_visited_cells(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut f = vec![vec![i32::MAX / 2; n]; m];
        f[0][0] = 1;
        let mut row = vec![BinaryHeap::<(Reverse<i32>, usize)>::new(); m];
        let mut col = vec![BinaryHeap::<(Reverse<i32>, usize)>::new(); n];
        for (i, r) in row.iter_mut().enumerate() {
            for (j, c) in col.iter_mut().enumerate() {
                while r.peek().is_some_and(|e| (e.1 + grid[i][e.1] as usize) < j) {
                    r.pop();
                }
                if let Some(&(_, j1)) = r.peek() {
                    f[i][j] = f[i][j].min(f[i][j1] + 1);
                }

                while c.peek().is_some_and(|e| (e.1 + grid[e.1][j] as usize) < i) {
                    c.pop();
                }
                if let Some(&(_, i1)) = c.peek() {
                    f[i][j] = f[i][j].min(f[i1][j] + 1);
                }

                if f[i][j] != -1 {
                    r.push((Reverse(f[i][j]), j));
                    c.push((Reverse(f[i][j]), i));
                }
            }
        }
        if f[m - 1][n - 1] == i32::MAX / 2 {
            -1
        } else {
            f[m - 1][n - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[3, 4, 2, 1], [4, 2, 3, 1], [2, 1, 0, 0], [2, 4, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(4, Solution::minimum_visited_cells(grid));
    }

    #[test]
    fn case2() {
        let grid = [[3, 4, 2, 1], [4, 2, 1, 1], [2, 1, 1, 0], [3, 4, 1, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(3, Solution::minimum_visited_cells(grid));
    }

    #[test]
    fn case3() {
        let grid = [[2, 1, 0], [1, 0, 0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(-1, Solution::minimum_visited_cells(grid));
    }
}
