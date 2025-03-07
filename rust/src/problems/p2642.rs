use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Graph {
    g: Vec<Vec<(i32, i32)>>,
}

impl Graph {
    pub fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut g = vec![vec![]; n as usize];
        for e in edges {
            g[e[0] as usize].push((e[1], e[2]));
        }
        Self { g }
    }

    pub fn add_edge(&mut self, e: Vec<i32>) {
        self.g[e[0] as usize].push((e[1], e[2]));
    }

    pub fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let n = self.g.len();
        let (n1, n2) = (node1 as usize, node2 as usize);
        let mut visited = vec![false; n];
        let mut dist = vec![i32::MAX; n];
        dist[n1] = 0;
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), n1));
        while let Some((Reverse(c), i)) = q.pop() {
            if i == n2 {
                return c;
            }
            visited[i] = true;
            for &(j, w) in &self.g[i] {
                let j = j as usize;
                if visited[j] {
                    continue;
                }
                if c + w < dist[j] {
                    dist[j] = c + w;
                    q.push((Reverse(c + w), j));
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
        let edges = [[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let mut g = Graph::new(4, edges);
        assert_eq!(6, g.shortest_path(3, 2));
        assert_eq!(-1, g.shortest_path(0, 3));
        g.add_edge(vec![1, 3, 4]);
        assert_eq!(6, g.shortest_path(0, 3));
    }
}
