use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn maximum_score(scores: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = scores.len();
        let mut g: Vec<BinaryHeap<(Reverse<i32>, i32)>> =
            (0..n).map(|_| BinaryHeap::new()).collect();
        for e in &edges {
            let entry = &mut g[e[0] as usize];
            entry.push((Reverse(scores[e[1] as usize]), e[1]));
            if entry.len() > 3 {
                entry.pop();
            }
            let entry = &mut g[e[1] as usize];
            entry.push((Reverse(scores[e[0] as usize]), e[0]));
            if entry.len() > 3 {
                entry.pop();
            }
        }

        let mut result = -1;
        for e in edges {
            let s = scores[e[0] as usize] + scores[e[1] as usize];
            for &(Reverse(score_x), x) in &g[e[0] as usize] {
                for &(Reverse(score_y), y) in &g[e[1] as usize] {
                    if x != e[1] && y != e[0] && x != y {
                        result = result.max(s + score_x + score_y);
                    }
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
        let scores = vec![5, 2, 9, 8, 4];
        let edges = [[0, 1], [1, 2], [2, 3], [0, 2], [1, 3], [2, 4]];
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        assert_eq!(24, Solution::maximum_score(scores, edges));
    }

    #[test]
    fn case2() {
        let scores = vec![9, 20, 6, 4, 11, 12];
        let edges = [[0, 3], [5, 3], [2, 4], [1, 3]];
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        assert_eq!(-1, Solution::maximum_score(scores, edges));
    }

    #[test]
    fn case3() {
        let scores = vec![8, 7, 1, 26];
        let edges = [[3, 1], [1, 2], [2, 0], [0, 3], [2, 3], [0, 1]];
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        assert_eq!(42, Solution::maximum_score(scores, edges));
    }

    #[test]
    fn case4() {
        let scores = vec![18, 6, 4, 9, 8, 2];
        let edges = [
            [0, 1],
            [0, 2],
            [0, 3],
            [0, 4],
            [0, 5],
            [1, 2],
            [1, 3],
            [1, 4],
            [1, 5],
            [2, 3],
            [2, 4],
            [2, 5],
            [3, 4],
            [3, 5],
            [4, 5],
        ];
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        assert_eq!(41, Solution::maximum_score(scores, edges));
    }
}
