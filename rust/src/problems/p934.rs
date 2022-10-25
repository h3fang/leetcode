pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len() as i32;
        for (i, r) in grid.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                if *c == 0 {
                    continue;
                }
                let mut island = vec![];
                let mut q = VecDeque::new();
                q.push_back((i, j));
                grid[i][j] = 2;
                while let Some((i, j)) = q.pop_front() {
                    island.push((i, j));
                    for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let i1 = i as i32 + di;
                        let j1 = j as i32 + dj;
                        if i1 < 0 || i1 == n || j1 < 0 || j1 == n {
                            continue;
                        }
                        let i1 = i1 as usize;
                        let j1 = j1 as usize;
                        if grid[i1][j1] == 1 {
                            grid[i1][j1] = 2;
                            q.push_back((i1, j1));
                        }
                    }
                }

                let mut steps = 0;

                loop {
                    let mut next = vec![];
                    for (i, j) in island {
                        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                            let i1 = i as i32 + di;
                            let j1 = j as i32 + dj;
                            if i1 < 0 || i1 == n || j1 < 0 || j1 == n {
                                continue;
                            }
                            let i1 = i1 as usize;
                            let j1 = j1 as usize;
                            if grid[i1][j1] == 1 {
                                return steps;
                            } else if grid[i1][j1] == 0 {
                                grid[i1][j1] = 2;
                                next.push((i1, j1));
                            }
                        }
                    }
                    island = next;
                    steps += 1;
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
        let grid = [[0, 1], [1, 0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(1, Solution::shortest_bridge(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0, 1, 0], [0, 0, 0], [0, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(2, Solution::shortest_bridge(grid));
    }

    #[test]
    fn case3() {
        let grid = [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(1, Solution::shortest_bridge(grid));
    }
}
