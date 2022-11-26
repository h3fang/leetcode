pub struct Solution;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let mut g = vec![vec![]; n as usize];
        for e in &edges {
            g[e[0] as usize].push((e[1], e[2]));
            g[e[1] as usize].push((e[0], e[2]));
        }
        let mut visited = HashSet::new();
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), 0));

        let mut reachable = HashMap::new();
        let mut result = 0;
        while let Some((Reverse(d), i)) = q.pop() {
            if visited.contains(&i) {
                continue;
            }
            result += 1;
            visited.insert(i);
            for &(next, nodes) in &g[i as usize] {
                reachable.insert((i, next), (max_moves - d).min(nodes));
                if d + nodes < max_moves && !visited.contains(&next) {
                    q.push((Reverse(d + nodes + 1), next));
                }
            }
        }
        for e in edges {
            result += e[2].min(
                reachable.get(&(e[0], e[1])).cloned().unwrap_or(0)
                    + reachable.get(&(e[1], e[0])).cloned().unwrap_or(0),
            );
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1, 10], [0, 2, 1], [1, 2, 2]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(13, Solution::reachable_nodes(edges, 6, 3));
    }

    #[test]
    fn case2() {
        let edges = [[0, 1, 4], [1, 2, 6], [0, 2, 8], [1, 3, 1]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(23, Solution::reachable_nodes(edges, 10, 4));
    }

    #[test]
    fn case3() {
        let edges = [[1, 2, 4], [1, 4, 5], [1, 3, 1], [2, 3, 4], [3, 4, 5]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(1, Solution::reachable_nodes(edges, 17, 5));
    }
}
