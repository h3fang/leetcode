pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn min_operations_queries(
        n: i32,
        edges: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let m = (i32::BITS - n.leading_zeros()) as usize;
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push((e[1], e[2] - 1));
            g[e[1] as usize].push((e[0], e[2] - 1));
        }
        let mut freq = vec![[0; 26]; n];
        let mut depth = vec![0; n];
        let mut f = vec![vec![0; m]; n];
        let mut p = vec![0; n];
        let mut q = VecDeque::new();
        q.push_back(0);
        while let Some(i) = q.pop_front() {
            f[i][0] = p[i];
            for j in 1..m {
                f[i][j] = f[f[i][j - 1]][j - 1];
            }
            for &(j, w) in &g[i] {
                let j = j as usize;
                if j != p[i] {
                    p[j] = i;
                    freq[j] = freq[i];
                    freq[j][w as usize] += 1;
                    depth[j] = depth[i] + 1;
                    q.push_back(j);
                }
            }
        }

        let lca = |mut x: usize, mut y: usize| -> usize {
            let (dx, dy) = (depth[x], depth[y]);
            if dx < dy {
                (x, y) = (y, x);
            }
            for j in (0..m).rev() {
                if depth[x] - depth[y] >= 1 << j {
                    x = f[x][j];
                }
            }
            for j in (0..m).rev() {
                if f[x][j] != f[y][j] {
                    (x, y) = (f[x][j], f[y][j]);
                }
            }
            if x != y {
                x = p[x];
            }
            x
        };

        queries
            .into_iter()
            .map(|q| {
                let (u, v) = (q[0] as usize, q[1] as usize);
                let x = lca(u, v);
                let most = (0..26)
                    .map(|j| freq[u][j] + freq[v][j] - 2 * freq[x][j])
                    .max()
                    .unwrap();
                depth[u] + depth[v] - 2 * depth[x] - most
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [
            [0, 1, 1],
            [1, 2, 1],
            [2, 3, 1],
            [3, 4, 2],
            [4, 5, 2],
            [5, 6, 2],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        let queries = [[0, 3], [3, 6], [2, 6], [0, 6]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![0, 0, 1, 3],
            Solution::min_operations_queries(7, edges, queries)
        );
    }

    #[test]
    fn case2() {
        let edges = [
            [1, 2, 6],
            [1, 3, 4],
            [2, 4, 6],
            [2, 5, 3],
            [3, 6, 6],
            [3, 0, 8],
            [7, 0, 2],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        let queries = [[4, 6], [0, 4], [6, 5], [7, 4]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![1, 2, 2, 3],
            Solution::min_operations_queries(8, edges, queries)
        );
    }
}
