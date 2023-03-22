pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut g = vec![vec![]; n as usize + 1];
        for r in roads {
            g[r[0] as usize].push((r[1], r[2]));
            g[r[1] as usize].push((r[0], r[2]));
        }
        let mut result = i32::MAX;
        let mut visited = vec![false; n as usize + 1];
        let mut q = VecDeque::new();
        q.push_back(1);
        visited[1] = true;
        while let Some(x) = q.pop_front() {
            for &e in &g[x as usize] {
                result = result.min(e.1);
                if !visited[e.0 as usize] {
                    visited[e.0 as usize] = true;
                    q.push_back(e.0);
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
        let roads = [[1, 2, 9], [2, 3, 6], [2, 4, 5], [1, 4, 7]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(5, Solution::min_score(4, roads));
    }

    #[test]
    fn case2() {
        let roads = [[1, 2, 2], [1, 3, 4], [3, 4, 7]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(2, Solution::min_score(4, roads));
    }
}
