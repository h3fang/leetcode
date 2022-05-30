use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let mut cost = vec![i32::MAX; (m * n) as usize];
        let mut q = VecDeque::new();
        cost[0] = 0;
        q.push_back((0, 0));
        while let Some((i, j)) = q.pop_front() {
            for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                if i1 < 0 || j1 < 0 || i1 == m || j1 == n {
                    continue;
                }
                let idx = (i1 * n + j1) as usize;
                let cell = grid[i1 as usize][j1 as usize];
                let c_next = cost[(i * n + j) as usize] + cell;
                if c_next < cost[idx] {
                    cost[idx] = c_next;
                    if cell == 1 {
                        q.push_back((i1, j1));
                    } else {
                        q.push_front((i1, j1));
                    }
                }
            }
        }
        cost[(m * n - 1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[0, 1, 1], [1, 1, 0], [1, 1, 0]];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(2, Solution::minimum_obstacles(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0, 1, 0, 0, 0], [0, 1, 0, 1, 0], [0, 0, 0, 1, 0]];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(0, Solution::minimum_obstacles(grid));
    }
}
