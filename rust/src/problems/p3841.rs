pub struct Solution;

struct Bit {
    tree: Vec<i32>,
}

impl Bit {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut i: i32, val: i32) {
        let len = self.tree.len() as i32;
        while i < len {
            self.tree[i as usize] ^= val;
            i += i.isolate_lowest_one();
        }
    }

    fn pre(&self, mut i: i32) -> i32 {
        let mut ans = 0;
        while i > 0 {
            ans ^= self.tree[i as usize];
            i &= i - 1;
        }
        ans
    }
}

struct Lca {
    p: Vec<Vec<i32>>,
    depth: Vec<i32>,
    t: Vec<[i32; 2]>,
    xors: Vec<i32>,
}

impl Lca {
    fn new(edges: Vec<Vec<i32>>, s: &[u8]) -> Self {
        let n = edges.len() + 1;
        let m = (usize::BITS - n.leading_zeros()) as usize;

        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }

        let mut xors = vec![0; n];
        xors[0] = 1 << (s[0] - b'a');

        let mut lca = Self {
            p: vec![vec![-1; n]; m],
            depth: vec![0; n],
            t: vec![[0; 2]; n],
            xors,
        };

        lca.dfs(s, &g, 0, -1, &mut 0);

        for i in 0..m - 1 {
            for x in 0..n {
                let y = lca.p[i][x];
                if y != -1 {
                    lca.p[i + 1][x] = lca.p[i][y as usize];
                }
            }
        }

        lca
    }

    fn dfs(&mut self, s: &[u8], g: &[Vec<i32>], x: i32, fa: i32, clock: &mut i32) {
        self.p[0][x as usize] = fa;

        *clock += 1;
        self.t[x as usize][0] = *clock;

        for &y in &g[x as usize] {
            if y == fa {
                continue;
            }

            self.depth[y as usize] = self.depth[x as usize] + 1;
            self.xors[y as usize] = self.xors[x as usize] ^ (1 << (s[y as usize] - b'a'));
            self.dfs(s, g, y, x, clock);
        }

        self.t[x as usize][1] = *clock;
    }

    fn kth_ancestor(&self, mut x: i32, k: i32) -> i32 {
        for i in 0..(32 - k.leading_zeros()) {
            if (k >> i) & 1 == 1 {
                x = self.p[i as usize][x as usize];
                if x == -1 {
                    break;
                }
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

        for p in self.p.iter().rev() {
            let (px, py) = (p[x as usize], p[y as usize]);
            if px != py {
                (x, y) = (px, py);
            }
        }

        self.p[0][x as usize]
    }
}

impl Solution {
    pub fn palindrome_path(
        n: i32,
        edges: Vec<Vec<i32>>,
        s: String,
        queries: Vec<String>,
    ) -> Vec<bool> {
        let mut s = s.into_bytes();
        let lca = Lca::new(edges, &s);
        let mut tree = Bit::new(n as usize);

        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let q = q.split_ascii_whitespace().collect::<Vec<&str>>();
            let x: i32 = q[1].parse().unwrap();

            if q[0] == "update" {
                let c = q[2].as_bytes()[0] - b'a';
                let val = (1 << (s[x as usize] - b'a')) ^ (1 << c);
                s[x as usize] = q[2].as_bytes()[0];

                tree.update(lca.t[x as usize][0], val);
                tree.update(lca.t[x as usize][1] + 1, val);
            } else {
                let y: i32 = q[2].parse().unwrap();
                let p = lca.lca(x, y);
                let res = lca.xors[x as usize]
                    ^ lca.xors[y as usize]
                    ^ tree.pre(lca.t[x as usize][0])
                    ^ tree.pre(lca.t[y as usize][0])
                    ^ (1 << (s[p as usize] - b'a'));
                ans.push(res & (res - 1) == 0);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 3;
        let edges = [[0, 1], [1, 2]].iter().map(|e| e.to_vec()).collect();
        let s = "aac".to_string();
        let queries = ["query 0 2", "update 1 b", "query 0 2"]
            .iter()
            .map(|q| q.to_string())
            .collect();
        assert_eq!(
            vec![true, false],
            Solution::palindrome_path(n, edges, s, queries)
        );
    }

    #[test]
    fn case2() {
        let n = 4;
        let edges = [[0, 1], [0, 2], [0, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let s = "abca".to_string();
        let queries = [
            "query 1 2",
            "update 0 b",
            "query 2 3",
            "update 3 a",
            "query 1 3",
        ]
        .iter()
        .map(|q| q.to_string())
        .collect();
        assert_eq!(
            vec![false, false, true],
            Solution::palindrome_path(n, edges, s, queries)
        );
    }
}
