pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn max_points(mut grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut queries = queries.into_iter().enumerate().collect::<Vec<_>>();
        queries.sort_unstable_by_key(|e| e.1);
        let (m, n) = (grid.len(), grid[0].len());
        let mut q: BinaryHeap<(Reverse<i32>, i32, i32)> = BinaryHeap::with_capacity(m * n);
        q.push((Reverse(grid[0][0]), 0, 0));
        let mut ans = vec![0; queries.len()];
        let mut count = 0;
        grid[0][0] = 0;
        for (i, v) in queries {
            while q.peek().is_some_and(|e| e.0.0 < v) {
                let (_, x, y) = q.pop().unwrap();
                count += 1;
                for (x1, y1) in [(x - 1, y), (x, y - 1), (x, y + 1), (x + 1, y)] {
                    if x1 >= 0
                        && y1 >= 0
                        && x1 < m as i32
                        && y1 < n as i32
                        && grid[x1 as usize][y1 as usize] > 0
                    {
                        q.push((Reverse(grid[x1 as usize][y1 as usize]), x1, y1));
                        grid[x1 as usize][y1 as usize] = 0;
                    }
                }
            }
            ans[i] = count;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 2, 3], [2, 5, 7], [3, 5, 1]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        let queries = vec![5, 6, 2];
        assert_eq!(vec![5, 8, 1], Solution::max_points(grid, queries));
    }

    #[test]
    fn case2() {
        let grid = [[5, 2, 1], [1, 1, 2]].iter().map(|q| q.to_vec()).collect();
        let queries = vec![3];
        assert_eq!(vec![0], Solution::max_points(grid, queries));
    }
}
