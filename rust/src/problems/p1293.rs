pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        if m == 1 && n == 1 {
            return 0;
        }
        let k = (m + n - 3).min(k);
        let mut q = VecDeque::new();
        let mut visited = vec![vec![vec![false; k as usize + 1]; n as usize]; m as usize];
        q.push_back((0, 0, 0, 0));
        visited[0][0][0] = true;
        while let Some((i, j, dist, obs)) = q.pop_front() {
            if i == m - 1 && j == n - 1 {
                return dist;
            }
            for (i1, j1) in [(i - 1, j), (i, j - 1), (i + 1, j), (i, j + 1)] {
                if i1 < 0 || j1 < 0 || i1 == m || j1 == n {
                    continue;
                }
                if grid[i1 as usize][j1 as usize] == 0
                    && !visited[i1 as usize][j1 as usize][obs as usize]
                {
                    q.push_back((i1, j1, dist + 1, obs));
                    visited[i1 as usize][j1 as usize][obs as usize] = true;
                } else if grid[i1 as usize][j1 as usize] == 1
                    && obs < k
                    && !visited[i1 as usize][j1 as usize][obs as usize + 1]
                {
                    q.push_back((i1, j1, dist + 1, obs + 1));
                    visited[i1 as usize][j1 as usize][obs as usize + 1] = true;
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
        let grid = [[0, 0, 0], [1, 1, 0], [0, 0, 0], [0, 1, 1], [0, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let k = 1;
        assert_eq!(6, Solution::shortest_path(grid, k));
    }

    #[test]
    fn case2() {
        let grid = [[0, 1, 1], [1, 1, 1], [1, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let k = 1;
        assert_eq!(-1, Solution::shortest_path(grid, k));
    }
}
