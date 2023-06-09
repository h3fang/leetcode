pub struct Solution;

fn dijkstra(
    g: &[Vec<usize>],
    edges: &mut [Vec<i32>],
    dist: &mut [[i64; 2]],
    destination: i32,
    delta: i64,
    pass: usize,
) {
    let n = g.len();
    let mut vis = vec![false; n];
    loop {
        let mut u = usize::MAX;
        for i in 0..n {
            if !vis[i] && (u == usize::MAX || dist[i][pass] < dist[u][pass]) {
                u = i;
            }
        }
        if u == destination as usize {
            return;
        }
        vis[u] = true;
        for v in 0..n {
            let i = g[u][v];
            if !vis[v] && i != usize::MAX {
                let mut w = edges[i][2];
                if w == -1 {
                    if pass == 0 {
                        w = 1;
                    } else {
                        let m = (delta + dist[v][0] - dist[u][1]) as i32;
                        if m > w {
                            w = m;
                            edges[i][2] = w;
                        }
                    }
                }
                dist[v][pass] = dist[v][pass].min(dist[u][pass] + w as i64);
            }
        }
    }
}

impl Solution {
    pub fn modified_graph_edges(
        n: i32,
        mut edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
        target: i32,
    ) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut g = vec![vec![usize::MAX; n]; n];
        for (i, e) in edges.iter().enumerate() {
            g[e[0] as usize][e[1] as usize] = i;
            g[e[1] as usize][e[0] as usize] = i;
        }
        let mut dis = vec![[i64::MAX / 2; 2]; n];
        dis[source as usize] = [0; 2];
        dijkstra(&g, &mut edges, &mut dis, destination, 0, 0);
        let delta = target as i64 - dis[destination as usize][0];
        if delta < 0 {
            return vec![];
        }
        dijkstra(&g, &mut edges, &mut dis, destination, delta, 1);
        if dis[destination as usize][1] < target as i64 {
            return vec![];
        }
        edges.iter_mut().for_each(|e| e[2] = e[2].max(1));
        edges
    }
}

#[cfg(test)]
mod tests {
    use std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap},
    };

    use super::*;

    fn check(
        n: i32,
        edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
        target: i32,
        result: &[Vec<i32>],
    ) -> bool {
        let mut m = HashMap::new();
        for e in edges {
            m.insert((e[0], e[1]), e[2]);
        }
        let n = n as usize;
        let mut g = vec![vec![0; n]; n];
        for e in result {
            if !m.get(&(e[0], e[1])).is_some_and(|&w| {
                (w > 0 && e[2] == w) || (w == -1 && (1..=20_0000_0000).contains(&e[2]))
            }) {
                return false;
            }
            g[e[0] as usize][e[1] as usize] = e[2];
            g[e[1] as usize][e[0] as usize] = e[2];
        }
        let mut dis = vec![i32::MAX; n];
        let mut q = BinaryHeap::new();
        dis[source as usize] = 0;
        q.push((Reverse(0), source as usize));
        while let Some((Reverse(d), i)) = q.pop() {
            if i == destination as usize {
                return d == target;
            }
            if d > dis[i] {
                continue;
            }
            for (j, &w) in g[i as usize].iter().enumerate() {
                if w == 0 {
                    continue;
                }
                if dis[i] + w < dis[j] {
                    dis[j] = dis[i] + w;
                    q.push((Reverse(dis[j]), j));
                }
            }
        }
        false
    }

    #[test]
    fn case1() {
        let (n, source, destination, target) = (5, 0, 1, 5);
        let edges = [[4, 1, -1], [2, 0, -1], [0, 3, -1], [4, 3, -1]]
            .iter()
            .map(|e| e.to_vec())
            .collect::<Vec<_>>();
        let result = Solution::modified_graph_edges(n, edges.clone(), source, destination, target);
        assert!(check(n, edges, source, destination, target, &result));
    }

    #[test]
    fn case2() {
        let (n, source, destination, target) = (3, 0, 2, 6);
        let edges = [[0, 1, -1], [0, 2, 5]].iter().map(|e| e.to_vec()).collect();
        let result = Solution::modified_graph_edges(n, edges, source, destination, target);
        assert!(result.is_empty());
    }
}
