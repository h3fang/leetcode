pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dist = vec![vec![i32::MAX; n]; n];
        let mut q = BinaryHeap::with_capacity(n * n);
        dist[0][0] = grid[0][0];
        q.push((Reverse(grid[0][0]), 0, 0));
        while let Some((Reverse(d), i, j)) = q.pop() {
            if (i as usize, j as usize) == (n - 1, n - 1) {
                return d;
            }
            if d > dist[i as usize][j as usize] {
                continue;
            }
            for (i1, j1) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
                if i1 < 0 || j1 < 0 || i1 as usize == n || j1 as usize == n {
                    continue;
                }
                let d1 = d.max(grid[i1 as usize][j1 as usize]);
                if d1 < dist[i1 as usize][j1 as usize] {
                    dist[i1 as usize][j1 as usize] = d1;
                    q.push((Reverse(d1), i1, j1));
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[0, 2], [1, 3]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(3, Solution::swim_in_water(grid));
    }

    #[test]
    fn case2() {
        let grid = [
            [0, 1, 2, 3, 4],
            [24, 23, 22, 21, 5],
            [12, 13, 14, 15, 16],
            [11, 17, 18, 19, 20],
            [10, 9, 8, 7, 6],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(16, Solution::swim_in_water(grid));
    }
}
