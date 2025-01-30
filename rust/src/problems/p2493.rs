pub struct Solution;

use std::collections::VecDeque;

fn bfs(g: &[Vec<i32>], i: i32) -> i32 {
    let mut d = 0;
    let mut vis = vec![false; g.len()];
    vis[i as usize] = true;
    let mut q = VecDeque::with_capacity(g.len());
    q.push_back(i);
    while !q.is_empty() {
        let n = q.len();
        for _ in 0..n {
            let i = q.pop_front().unwrap();
            for &j in &g[i as usize] {
                if !vis[j as usize] {
                    vis[j as usize] = true;
                    q.push_back(j);
                }
            }
        }
        d += 1;
    }
    d
}

fn find_bipartite(
    g: &[Vec<i32>],
    colors: &mut [i32],
    nodes: &mut Vec<i32>,
    i: i32,
    c: i32,
) -> bool {
    nodes.push(i);
    colors[i as usize] = c;
    for &j in &g[i as usize] {
        if colors[j as usize] == c
            || (colors[j as usize] == 0 && !find_bipartite(g, colors, nodes, j, -c))
        {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize - 1].push(e[1] - 1);
            g[e[1] as usize - 1].push(e[0] - 1);
        }
        let mut colors = vec![0; n];
        let mut result = 0;
        for i in 0..n {
            if colors[i] != 0 {
                continue;
            }
            let mut nodes = Vec::with_capacity(g.len());
            if !find_bipartite(&g, &mut colors, &mut nodes, i as i32, 1) {
                return -1;
            }
            result += nodes.into_iter().map(|i| bfs(&g, i)).max().unwrap();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[1, 2], [1, 4], [1, 5], [2, 6], [2, 3], [4, 6]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(4, Solution::magnificent_sets(6, edges));
    }

    #[test]
    fn case2() {
        let edges = [[1, 2], [2, 3], [3, 1]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(-1, Solution::magnificent_sets(3, edges));
    }
}
