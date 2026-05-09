pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (m, n, k) = (grid.len(), grid[0].len(), k as usize);

        let mut ring = VecDeque::with_capacity((m + n) * 2);
        let layers = m.min(n) / 2;
        for d in 0..layers {
            ring.extend(&grid[d][d..n - d]);
            for r in grid[d + 1..m - 1 - d].iter_mut() {
                ring.push_back(r[n - 1 - d]);
            }
            ring.extend(grid[m - 1 - d][d..n - d].iter().rev());
            for r in grid[d + 1..m - 1 - d].iter_mut().rev() {
                ring.push_back(r[d]);
            }

            ring.rotate_left(k % ring.len());

            for e in grid[d][d..n - d].iter_mut() {
                *e = ring.pop_front().unwrap();
            }
            for r in grid[d + 1..m - 1 - d].iter_mut() {
                r[n - 1 - d] = ring.pop_front().unwrap();
            }
            for e in grid[m - 1 - d][d..n - d].iter_mut().rev() {
                *e = ring.pop_front().unwrap();
            }
            for r in grid[d + 1..m - 1 - d].iter_mut().rev() {
                r[d] = ring.pop_front().unwrap();
            }
        }

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[40, 10], [30, 20]].iter().map(|r| r.to_vec()).collect();
        let expected = [[10, 20], [40, 30]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::rotate_grid(grid, 1));
    }

    #[test]
    fn case2() {
        let grid = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        let expected = [
            [3, 4, 8, 12],
            [2, 11, 10, 16],
            [1, 7, 6, 15],
            [5, 9, 13, 14],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect::<Vec<_>>();
        assert_eq!(expected, Solution::rotate_grid(grid, 2));
    }
}
