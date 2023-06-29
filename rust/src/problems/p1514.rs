pub struct Solution;

use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(PartialEq)]
struct Float(f64);

impl Eq for Float {}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let mut g = vec![vec![]; n as usize];
        for (e, p) in edges.into_iter().zip(succ_prob) {
            g[e[0] as usize].push((e[1], p));
            g[e[1] as usize].push((e[0], p));
        }
        let mut q = BinaryHeap::new();
        q.push((Float(1.0), start));
        let mut probs = vec![0.0; n as usize];
        probs[start as usize] = 1.0;
        while let Some((Float(p), i)) = q.pop() {
            if i == end {
                return p;
            }
            if p < probs[i as usize] {
                continue;
            }
            for &(j, p1) in &g[i as usize] {
                if p * p1 > probs[j as usize] {
                    probs[j as usize] = p * p1;
                    q.push((Float(p * p1), j));
                }
            }
        }
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5, "a = {a}, b = {b}");
    }

    #[test]
    fn case1() {
        let edges = [[0, 1], [1, 2], [0, 2]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let succ_prob = vec![0.5, 0.5, 0.2];
        assert_close(0.25, Solution::max_probability(3, edges, succ_prob, 0, 2));
    }

    #[test]
    fn case2() {
        let edges = [[0, 1], [1, 2], [0, 2]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let succ_prob = vec![0.5, 0.5, 0.3];
        assert_close(0.3, Solution::max_probability(3, edges, succ_prob, 0, 2));
    }

    #[test]
    fn case3() {
        let edges = [[0, 1]].iter().map(|e| e.to_vec()).collect();
        let succ_prob = vec![0.5];
        assert_close(0.0, Solution::max_probability(3, edges, succ_prob, 0, 2));
    }
}
