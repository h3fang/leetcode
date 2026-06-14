pub struct Solution;

use std::sync::OnceLock;

const MOD: i32 = 10_0000_0007;
static POW2: OnceLock<Vec<i32>> = OnceLock::new();

fn init_pow2() -> Vec<i32> {
    let mut pow2 = vec![0; 10_0000];
    pow2[0] = 1;
    for i in 1..pow2.len() {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
    }
    pow2
}

pub struct Lca {
    depth: Vec<i32>,
    pa: Vec<Vec<i32>>,
}

impl Lca {
    fn new(edges: Vec<Vec<i32>>) -> Self {
        let n = edges.len() + 1;
        let m = 64 - n.leading_zeros() as usize;

        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize - 1].push(e[1] - 1);
            g[e[1] as usize - 1].push(e[0] - 1);
        }

        let mut depth = vec![0; n];
        let mut pa = vec![vec![-1; m]; n];

        dfs(&g, &mut depth, &mut pa, 0, -1);

        for i in 0..m - 1 {
            for x in 0..n {
                let p = pa[x][i];
                if p != -1 {
                    pa[x][i + 1] = pa[p as usize][i];
                }
            }
        }

        Self { depth, pa }
    }

    fn kth_ancestor(&self, mut x: i32, k: i32) -> i32 {
        let bits = 32 - k.leading_zeros() as usize;
        for i in 0..bits {
            if (k >> i) & 1 == 1 {
                x = self.pa[x as usize][i];
            }
        }
        x
    }

    fn lca(&self, mut x: i32, mut y: i32) -> i32 {
        if self.depth[x as usize] > self.depth[y as usize] {
            (x, y) = (y, x);
        }

        y = self.kth_ancestor(y, self.depth[y as usize] - self.depth[x as usize]);

        if y == x {
            return x;
        }

        for i in (0..self.pa[x as usize].len()).rev() {
            let px = self.pa[x as usize][i];
            let py = self.pa[y as usize][i];
            if px != py {
                (x, y) = (px, py);
            }
        }

        self.pa[x as usize][0]
    }

    fn distance(&self, x: i32, y: i32) -> i32 {
        self.depth[x as usize] + self.depth[y as usize] - self.depth[self.lca(x, y) as usize] * 2
    }
}

fn dfs(g: &[Vec<i32>], depth: &mut [i32], pa: &mut [Vec<i32>], x: i32, p: i32) {
    pa[x as usize][0] = p;
    for &y in &g[x as usize] {
        if y == p {
            continue;
        }
        depth[y as usize] = depth[x as usize] + 1;
        dfs(g, depth, pa, y, x);
    }
}

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let pow2 = POW2.get_or_init(init_pow2);

        let lca = Lca::new(edges);

        queries
            .into_iter()
            .map(|q| {
                let (x, y) = (q[0] - 1, q[1] - 1);
                if x != y {
                    pow2[lca.distance(x, y) as usize - 1]
                } else {
                    0
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[1, 2]].iter().map(|e| e.to_vec()).collect();
        let queries = [[1, 1], [1, 2]].iter().map(|e| e.to_vec()).collect();
        assert_eq!(vec![0, 1], Solution::assign_edge_weights(edges, queries));
    }

    #[test]
    fn case2() {
        let edges = [[1, 2], [1, 3], [3, 4], [3, 5]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let queries = [[1, 4], [3, 4], [2, 5]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(vec![2, 1, 4], Solution::assign_edge_weights(edges, queries));
    }
}
