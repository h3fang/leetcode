pub struct Solution;

struct Lca {
    dis: Vec<i32>,
    depth: Vec<i32>,
    parents: Vec<Vec<i32>>,
}

impl Lca {
    fn new(g: &[Vec<(i32, i32)>]) -> Self {
        let n = g.len();
        let m = 64 - n.leading_zeros() as usize;

        let mut s = Self {
            dis: vec![0; n],
            depth: vec![0; n],
            parents: vec![vec![-1; m]; n],
        };

        s.dfs(g, 0, -1);

        for i in 0..m - 1 {
            for x in 0..n {
                let p = s.parents[x][i];
                if p != -1 {
                    s.parents[x][i + 1] = s.parents[p as usize][i];
                }
            }
        }

        s
    }

    fn dfs(&mut self, g: &[Vec<(i32, i32)>], x: i32, p: i32) {
        self.parents[x as usize][0] = p;
        let d = self.depth[x as usize] + 1;
        for &(y, w) in &g[x as usize] {
            if y != p {
                self.depth[y as usize] = d;
                self.dis[y as usize] = self.dis[x as usize] + w;
                self.dfs(g, y, x);
            }
        }
    }

    fn kth_ancestor(&self, mut x: i32, mut k: i32) -> i32 {
        while k > 0 {
            x = self.parents[x as usize][k.trailing_zeros() as usize];
            k &= k - 1;
        }
        x
    }

    fn lca(&self, mut x: i32, mut y: i32) -> i32 {
        if self.depth[x as usize] > self.depth[y as usize] {
            (x, y) = (y, x);
        }
        y = self.kth_ancestor(y, self.depth[y as usize] - self.depth[x as usize]);
        if x == y {
            return x;
        }
        for i in (0..self.parents[x as usize].len()).rev() {
            let (px, py) = (self.parents[x as usize][i], self.parents[y as usize][i]);
            if px != py {
                (x, y) = (px, py);
            }
        }
        self.parents[x as usize][0]
    }

    fn distance(&self, x: i32, y: i32) -> i32 {
        self.dis[x as usize] + self.dis[y as usize] - self.dis[self.lca(x, y) as usize] * 2
    }
}

impl Solution {
    pub fn minimum_weight(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push((e[1], e[2]));
            g[e[1] as usize].push((e[0], e[2]));
        }

        let lca = Lca::new(&g);
        queries
            .into_iter()
            .map(|q| {
                (lca.distance(q[0], q[1]) + lca.distance(q[1], q[2]) + lca.distance(q[2], q[0])) / 2
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1, 2], [1, 2, 3], [1, 3, 5], [1, 4, 4], [2, 5, 6]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        let queries = [[2, 3, 4], [0, 2, 5]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(vec![12, 11], Solution::minimum_weight(edges, queries));
    }

    #[test]
    fn case2() {
        let edges = [[1, 0, 8], [0, 2, 7]].iter().map(|q| q.to_vec()).collect();
        let queries = [[0, 1, 2]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(vec![15], Solution::minimum_weight(edges, queries));
    }
}
