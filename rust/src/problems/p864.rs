pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let mut start = (0, 0);
        let mut k = 0;
        for (i, r) in grid.iter().enumerate() {
            for (j, &b) in r.as_bytes().iter().enumerate() {
                match b {
                    b'@' => start = (i as i32, j as i32),
                    x if x.is_ascii_lowercase() => k += 1,
                    _ => {}
                }
            }
        }
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        q.push_back((start, 0u8, 0));
        visited.insert((start, 0));
        while let Some(((i, j), state, dist)) = q.pop_front() {
            if state.count_ones() == k {
                return dist;
            }
            for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                if i1 < 0 || j1 < 0 || i1 == m || j1 == n {
                    continue;
                }
                let b = grid[i1 as usize].as_bytes()[j1 as usize];
                if b == b'#' || (b.is_ascii_uppercase() && (state & (1 << (b - b'A')) == 0)) {
                    continue;
                }
                let s = if b.is_ascii_lowercase() {
                    state | 1 << (b - b'a')
                } else {
                    state
                };
                if !visited.contains(&((i1, j1), s)) {
                    q.push_back(((i1, j1), s, dist + 1));
                    visited.insert(((i1, j1), s));
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
        let grid = ["@.a.#", "###.#", "b.A.B"]
            .iter()
            .map(|r| r.to_string())
            .collect();
        assert_eq!(8, Solution::shortest_path_all_keys(grid));
    }

    #[test]
    fn case2() {
        let grid = ["@..aA", "..B#.", "....b"]
            .iter()
            .map(|r| r.to_string())
            .collect();
        assert_eq!(6, Solution::shortest_path_all_keys(grid));
    }

    #[test]
    fn case3() {
        let grid = ["@Aa"].iter().map(|r| r.to_string()).collect();
        assert_eq!(-1, Solution::shortest_path_all_keys(grid));
    }
}
