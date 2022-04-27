use std::collections::HashMap;

pub struct Solution;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return;
        }
        match self.rank[px].cmp(&self.rank[py]) {
            std::cmp::Ordering::Less => self.parent[py] = px,
            std::cmp::Ordering::Equal => {
                self.rank[px] += 1;
                self.parent[py] = px;
            }
            std::cmp::Ordering::Greater => self.parent[px] = py,
        }
    }
}

impl Solution {
    pub fn smallest_string_with_swaps(mut s: String, pairs: Vec<Vec<i32>>) -> String {
        let mut uf = UnionFind::new(s.len());
        for p in pairs {
            uf.union(p[0] as usize, p[1] as usize);
        }
        let mut comps: HashMap<usize, Vec<usize>> = HashMap::new();
        for x in 0..s.len() {
            let root = uf.find(x);
            comps.entry(root).or_default().push(x);
        }
        let bytes = unsafe { s.as_bytes_mut() };
        for comp in comps.values() {
            let mut chars = comp.iter().map(|&i| bytes[i]).collect::<Vec<_>>();
            chars.sort_unstable();
            for (&c, &i) in chars.iter().zip(comp) {
                bytes[i] = c;
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "dcab".into();
        let pairs = [[0, 3], [1, 2]];
        let pairs = pairs.iter().map(|p| p.to_vec()).collect();
        assert_eq!("bacd", Solution::smallest_string_with_swaps(s, pairs));
    }

    #[test]
    fn case2() {
        let s = "dcab".into();
        let pairs = [[0, 3], [1, 2], [0, 2]];
        let pairs = pairs.iter().map(|p| p.to_vec()).collect();
        assert_eq!("abcd", Solution::smallest_string_with_swaps(s, pairs));
    }

    #[test]
    fn case3() {
        let s = "cba".into();
        let pairs = [[0, 1], [1, 2]];
        let pairs = pairs.iter().map(|p| p.to_vec()).collect();
        assert_eq!("abc", Solution::smallest_string_with_swaps(s, pairs));
    }
}
