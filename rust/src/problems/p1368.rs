pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dist = vec![i32::MAX; m * n];
        let mut visited = vec![false; m * n];
        let mut q = VecDeque::with_capacity(m * n);
        q.push_back((0, 0, 0));
        dist[0] = 0;
        while let Some((i, j, c)) = q.pop_front() {
            if visited[i as usize * n + j as usize] {
                continue;
            }
            visited[i as usize * n + j as usize] = true;
            for (d, (i1, j1)) in [(i, j + 1), (i, j - 1), (i + 1, j), (i - 1, j)]
                .into_iter()
                .enumerate()
            {
                if i1 < 0 || j1 < 0 || i1 == m as i32 || j1 == n as i32 {
                    continue;
                }
                let c1 = c + i32::from(d as i32 + 1 != grid[i as usize][j as usize]);
                if c1 < dist[i1 as usize * n + j1 as usize] {
                    dist[i1 as usize * n + j1 as usize] = c1;
                    if c1 == c {
                        q.push_front((i1, j1, c1));
                    } else {
                        q.push_back((i1, j1, c1));
                    }
                }
            }
        }
        dist[m * n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 1, 1, 1], [2, 2, 2, 2], [1, 1, 1, 1], [2, 2, 2, 2]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(3, Solution::min_cost(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 1, 3], [3, 2, 2], [1, 1, 4]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(0, Solution::min_cost(grid));
    }

    #[test]
    fn case3() {
        let grid = [[1, 2], [4, 3]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(1, Solution::min_cost(grid));
    }

    #[test]
    fn case4() {
        let grid = [[2, 2, 2], [2, 2, 2]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(3, Solution::min_cost(grid));
    }

    #[test]
    fn case5() {
        let grid = [[4]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(0, Solution::min_cost(grid));
    }
}
