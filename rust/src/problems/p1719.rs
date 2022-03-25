pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
        let mut adj: HashMap<i32, HashSet<i32>> = HashMap::new();
        for item in pairs {
            adj.entry(item[0]).or_default().insert(item[1]);
            adj.entry(item[1]).or_default().insert(item[0]);
        }

        let mut root = -1;
        let tmp = adj.len();
        for (node, neighbours) in &adj {
            if neighbours.len() == tmp - 1 {
                root = *node;
                break;
            }
        }
        if root == -1 {
            return 0;
        }
        let mut res = 1;
        for (node, neighbours) in &adj {
            if *node == root {
                continue;
            }
            let curr_degree = neighbours.len();
            let mut parent = -1;
            let mut parent_degree = usize::MAX;
            for neighbor in neighbours {
                let len = adj[neighbor].len();
                if len < parent_degree && len >= curr_degree {
                    parent = *neighbor;
                    parent_degree = len;
                }
            }
            let adj_parent = adj.get(&parent).unwrap();
            for neighbour in neighbours {
                if *neighbour == parent {
                    continue;
                }
                if !adj_parent.contains(neighbour) {
                    return 0;
                }
            }
            if parent_degree == curr_degree {
                res = 2;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let pairs = [[1, 2], [2, 3]];
        let pairs = pairs.iter().map(|p| p.to_vec()).collect();
        assert_eq!(1, Solution::check_ways(pairs));
    }

    #[test]
    fn case2() {
        let pairs = [[1, 2], [2, 3], [1, 3]];
        let pairs = pairs.iter().map(|p| p.to_vec()).collect();
        assert_eq!(2, Solution::check_ways(pairs));
    }

    #[test]
    fn case3() {
        let pairs = [[1, 2], [2, 3], [2, 4], [1, 5]];
        let pairs = pairs.iter().map(|p| p.to_vec()).collect();
        assert_eq!(0, Solution::check_ways(pairs));
    }
}
