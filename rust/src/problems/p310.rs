use std::collections::{HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n < 2 {
            return (0..n).collect();
        }
        let mut g: Vec<HashSet<i32>> = vec![HashSet::new(); n as usize];
        for e in edges {
            g[e[0] as usize].insert(e[1]);
            g[e[1] as usize].insert(e[0]);
        }
        let mut q = VecDeque::new();
        for (i, subtree) in g.iter().enumerate() {
            if subtree.len() == 1 {
                q.push_back(i as i32);
            }
        }
        let mut remaining = n as usize;
        while remaining > 2 {
            let k = q.len();
            remaining -= k;
            for _ in 0..k {
                let node = q.pop_front().unwrap();
                let parent = g[node as usize].drain().next().unwrap();
                g[parent as usize].remove(&node);
                if g[parent as usize].len() == 1 {
                    q.push_back(parent);
                }
            }
        }
        q.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 4;
        let edges = [[1, 0], [1, 2], [1, 3]];
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        assert_eq!(vec![1], Solution::find_min_height_trees(n, edges));
    }

    #[test]
    fn case2() {
        let n = 6;
        let edges = [[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]];
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        assert_eq!(vec![3, 4], Solution::find_min_height_trees(n, edges));
    }

    #[test]
    fn case3() {
        let n = 1;
        let edges = [vec![]; 0];
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        assert_eq!(vec![0], Solution::find_min_height_trees(n, edges));
    }

    #[test]
    fn case4() {
        let n = 2;
        let edges = [[0, 1]];
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        assert_eq!(vec![0, 1], Solution::find_min_height_trees(n, edges));
    }
}
