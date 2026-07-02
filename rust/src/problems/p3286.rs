pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dist = vec![vec![i32::MAX; n]; m];
        dist[0][0] = grid[0][0];
        let mut q = VecDeque::with_capacity(m * n);
        q.push_back((0, 0));
        while let Some((i, j)) = q.pop_front() {
            if dist[i as usize][j as usize] >= health {
                return false;
            }

            if (i, j) == (m as i32 - 1, n as i32 - 1) {
                return true;
            }

            for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                if i1 < 0 || i1 >= m as i32 || j1 < 0 || j1 >= n as i32 {
                    continue;
                }
                let cost = grid[i1 as usize][j1 as usize];
                let d1 = dist[i as usize][j as usize] + cost;
                if d1 < dist[i1 as usize][j1 as usize] {
                    dist[i1 as usize][j1 as usize] = d1;
                    if cost == 0 {
                        q.push_front((i1, j1));
                    } else {
                        q.push_back((i1, j1));
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[0, 1, 0, 0, 0], [0, 1, 0, 1, 0], [0, 0, 0, 1, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert!(Solution::find_safe_walk(grid, 1));
    }

    #[test]
    fn case2() {
        let grid = [
            [0, 1, 1, 0, 0, 0],
            [1, 0, 1, 0, 0, 0],
            [0, 1, 1, 1, 0, 1],
            [0, 0, 1, 0, 1, 0],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert!(!Solution::find_safe_walk(grid, 3));
    }

    #[test]
    fn case3() {
        let grid = [[1, 1, 1], [1, 0, 1], [1, 1, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert!(Solution::find_safe_walk(grid, 5));
    }
}
