use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let mut g = vec![vec![]; n as usize + 1];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }

        let mut dist = vec![[0, 0]; n as usize + 1];
        let mut queue = BinaryHeap::new();
        queue.push((0, 1));
        while let Some((curr, node)) = queue.pop() {
            if node == n && dist[n as usize][1] > 0 {
                return dist[n as usize][1];
            }

            let curr = -curr;

            let next_time = if (curr / change) % 2 == 0 {
                curr + time
            } else {
                (curr / change + 1) * change + time
            };

            for &next in &g[node as usize] {
                let d = dist.get_mut(next as usize).unwrap();
                if d[0] == 0 {
                    d[0] = next_time;
                    queue.push((-next_time, next));
                } else if d[1] == 0 && next_time > d[0] {
                    d[1] = next_time;
                    queue.push((-next_time, next));
                }
            }
        }
        panic!("impossible");
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
