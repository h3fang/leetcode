pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push((e[1], e[2]));
            g[e[1] as usize].push((e[0], 2 * e[2]));
        }
        let mut q = BinaryHeap::with_capacity(n);
        let mut dist = vec![i32::MAX; n];
        dist[0] = 0;
        q.push((Reverse(0), 0));

        while let Some((Reverse(d), i)) = q.pop() {
            if i == n - 1 {
                return d;
            }
            if d > dist[i] {
                continue;
            }
            for &(j, w) in &g[i] {
                if d + w < dist[j as usize] {
                    dist[j as usize] = d + w;
                    q.push((Reverse(d + w), j as usize));
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
        let edges = [[0, 1, 3], [3, 1, 1], [2, 3, 4], [0, 2, 2]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(5, Solution::min_cost(4, edges));
    }

    #[test]
    fn case2() {
        let edges = [[0, 2, 1], [2, 1, 1], [1, 3, 1], [2, 3, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(3, Solution::min_cost(4, edges));
    }
}
