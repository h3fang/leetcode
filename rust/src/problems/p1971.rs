pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut g = vec![vec![]; n as usize];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        q.push_back(source);
        visited.insert(source);
        while let Some(c) = q.pop_front() {
            if c == destination {
                return true;
            }
            for &next in &g[c as usize] {
                if visited.contains(&next) {
                    continue;
                }
                q.push_back(next);
                visited.insert(next);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1], [1, 2], [2, 0]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert!(Solution::valid_path(3, edges, 0, 2));
    }

    #[test]
    fn case2() {
        let edges = [[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert!(!Solution::valid_path(6, edges, 0, 5));
    }
}
