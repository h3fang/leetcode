pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut q = VecDeque::new();
        q.push_back((0, 0, 0));
        let mut dist = vec![vec![[-1; 2]; n]; n];
        dist[0][0][0] = 0;
        while let Some((x, y, t)) = q.pop_front() {
            for (x1, y1, t1) in [(x + 1, y, t), (x, y + 1, t), (x, y, t ^ 1)] {
                let (x2, y2) = (x1 + t1, y1 + (1 ^ t1));
                if x2 < n
                    && y2 < n
                    && grid[x1][y1] == 0
                    && grid[x2][y2] == 0
                    && (t == t1 || grid[x1 + 1][y1 + 1] == 0)
                    && dist[x1][y1][t1] == -1
                {
                    dist[x1][y1][t1] = dist[x][y][t] + 1;
                    q.push_back((x1, y1, t1));
                }
            }
        }
        dist[n - 1][n - 2][0]
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn case1() {
        let grid = [
            [0, 0, 0, 0, 0, 1],
            [1, 1, 0, 0, 1, 0],
            [0, 0, 0, 0, 1, 1],
            [0, 0, 1, 0, 1, 0],
            [0, 1, 1, 0, 0, 0],
            [0, 1, 1, 0, 0, 0],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(11, Solution::minimum_moves(grid));
    }

    #[test]
    fn case2() {
        let grid = [
            [0, 0, 1, 1, 1, 1],
            [0, 0, 0, 0, 1, 1],
            [1, 1, 0, 0, 0, 1],
            [1, 1, 1, 0, 0, 1],
            [1, 1, 1, 0, 0, 1],
            [1, 1, 1, 0, 0, 0],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(9, Solution::minimum_moves(grid));
    }
}
