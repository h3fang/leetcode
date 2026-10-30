pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        let mut g = vec![vec![]; n as usize];
        for e in edges {
            g[e[0] as usize].push((e[1], e[2]));
        }

        let labels = labels.as_bytes();
        let mut q = BinaryHeap::with_capacity(n as usize);
        let mut dist = vec![vec![i32::MAX; k as usize + 1]; n as usize];

        q.push((Reverse(0), 0, 1));
        dist[0][1] = 0;

        while let Some((Reverse(cost), x, cnt)) = q.pop() {
            if x == n - 1 {
                return cost;
            }

            if cost > dist[x as usize][cnt as usize] {
                continue;
            }

            for &(y, w) in &g[x as usize] {
                let cost1 = cost + w;
                let cnt1 = if labels[x as usize] == labels[y as usize] {
                    cnt + 1
                } else {
                    1
                };
                if cnt1 <= k && cost1 < dist[y as usize][cnt1 as usize] {
                    dist[y as usize][cnt1 as usize] = cost1;
                    q.push((Reverse(cost1), y, cnt1));
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
        let n = 3;
        let edges = [[0, 1, 1], [1, 2, 1], [0, 2, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let labels = "aab".to_string();
        let k = 1;
        assert_eq!(3, Solution::shortest_path(n, edges, labels, k));
    }

    #[test]
    fn case2() {
        let n = 3;
        let edges = [[0, 1, 1], [1, 2, 1], [0, 2, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let labels = "aab".to_string();
        let k = 2;
        assert_eq!(2, Solution::shortest_path(n, edges, labels, k));
    }

    #[test]
    fn case3() {
        let n = 3;
        let edges = [[0, 1, 1], [1, 2, 1]].iter().map(|e| e.to_vec()).collect();
        let labels = "aaa".to_string();
        let k = 2;
        assert_eq!(-1, Solution::shortest_path(n, edges, labels, k));
    }
}
