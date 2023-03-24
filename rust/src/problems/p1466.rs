pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut g = vec![vec![]; n as usize];
        for c in connections {
            g[c[0] as usize].push((c[1], 1));
            g[c[1] as usize].push((c[0], 0));
        }
        let mut result = 0;
        let mut q = VecDeque::new();
        q.push_back(0);
        let mut visited = vec![false; n as usize];
        visited[0] = true;
        while let Some(i) = q.pop_front() {
            for &(j, s) in &g[i as usize] {
                if !visited[j as usize] {
                    visited[j as usize] = true;
                    result += s;
                    q.push_back(j);
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
        let connections = [[0, 1], [1, 3], [2, 3], [4, 0], [4, 5]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        assert_eq!(3, Solution::min_reorder(6, connections));
    }

    #[test]
    fn case2() {
        let connections = [[1, 0], [1, 2], [3, 2], [3, 4]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        assert_eq!(2, Solution::min_reorder(5, connections));
    }

    #[test]
    fn case3() {
        let connections = [[1, 0], [2, 0]].iter().map(|c| c.to_vec()).collect();
        assert_eq!(0, Solution::min_reorder(3, connections));
    }
}
