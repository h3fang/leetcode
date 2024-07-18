pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn minimum_time(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32> {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push((e[1], e[2]));
            g[e[1] as usize].push((e[0], e[2]));
        }
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), 0));
        let mut result = vec![-1; n];
        result[0] = 0;
        while let Some((Reverse(d), x)) = q.pop() {
            if d > result[x] {
                continue;
            }
            for &(y, w) in &g[x] {
                if (result[y as usize] == -1 || d + w < result[y as usize])
                    && d + w < disappear[y as usize]
                {
                    result[y as usize] = d + w;
                    q.push((Reverse(d + w), y as usize));
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
        let edges = [[0, 1, 2], [1, 2, 1], [0, 2, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(
            vec![0, -1, 4],
            Solution::minimum_time(3, edges, vec![1, 1, 5])
        );
    }

    #[test]
    fn case2() {
        let edges = [[0, 1, 2], [1, 2, 1], [0, 2, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(
            vec![0, 2, 3],
            Solution::minimum_time(3, edges, vec![1, 3, 5])
        );
    }

    #[test]
    fn case3() {
        let edges = [[0, 1, 1]].iter().map(|e| e.to_vec()).collect();
        assert_eq!(vec![0, -1], Solution::minimum_time(2, edges, vec![1, 1]));
    }
}
