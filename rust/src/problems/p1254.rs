pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        fn bfs(i: usize, j: usize, grid: &[Vec<i32>], visited: &mut [Vec<bool>]) -> bool {
            let m = grid.len();
            let n = grid[0].len();
            let mut q = VecDeque::new();
            q.push_back((i as i32, j as i32));
            visited[i][j] = true;
            let mut is_closed = true;
            while let Some((i, j)) = q.pop_front() {
                for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                    if i1 < 0 || j1 < 0 || i1 == m as i32 || j1 == n as i32 {
                        is_closed = false;
                    } else if grid[i1 as usize][j1 as usize] == 0
                        && !visited[i1 as usize][j1 as usize]
                    {
                        visited[i1 as usize][j1 as usize] = true;
                        q.push_back((i1, j1));
                    }
                }
            }
            is_closed
        }
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut result = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 && !visited[i][j] && bfs(i, j, &grid, &mut visited) {
                    result += 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [
            [1, 1, 1, 1, 1, 1, 1, 0],
            [1, 0, 0, 0, 0, 1, 1, 0],
            [1, 0, 1, 0, 1, 1, 1, 0],
            [1, 0, 0, 0, 0, 1, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 0],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(2, Solution::closed_island(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0, 0, 1, 0, 0], [0, 1, 0, 1, 0], [0, 1, 1, 1, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(1, Solution::closed_island(grid));
    }

    #[test]
    fn case3() {
        let grid = [
            [1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0, 0, 1],
            [1, 0, 1, 1, 1, 0, 1],
            [1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(2, Solution::closed_island(grid));
    }
}
