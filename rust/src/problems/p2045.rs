use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n + 1];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }

        let mut dist = vec![[i32::MAX; 2]; n + 1];
        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), 1));
        while let Some((curr, node)) = queue.pop() {
            if node == n && dist[n][1] < i32::MAX {
                break;
            }

            let curr = curr.0;

            for &next in &g[node] {
                let next = next as usize;
                if curr + 1 < dist[next][0] {
                    dist[next][0] = curr + 1;
                    queue.push((Reverse(curr + 1), next));
                } else if curr + 1 > dist[next][0] && curr + 1 < dist[next][1] {
                    dist[next][1] = curr + 1;
                    queue.push((Reverse(curr + 1), next));
                }
            }
        }
        let mut result = 0;
        for _ in 0..dist[n][1] {
            if result % (2 * change) >= change {
                result += 2 * change - result % (2 * change);
            }
            result += time;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 5;
        let edges = [[1, 2], [1, 3], [1, 4], [3, 4], [4, 5]];
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        let time = 3;
        let change = 5;
        assert_eq!(13, Solution::second_minimum(n, edges, time, change));
    }

    #[test]
    fn case2() {
        let n = 2;
        let edges = [[1, 2]];
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        let time = 3;
        let change = 2;
        assert_eq!(11, Solution::second_minimum(n, edges, time, change));
    }
}
