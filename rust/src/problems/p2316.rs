use std::collections::HashSet;

pub struct Solution;

struct Dsu {
    parent: Vec<i32>,
    size: Vec<i32>,
}

impl Dsu {
    fn new(n: i32) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n as usize],
        }
    }
    fn find(&mut self, x: i32) -> i32 {
        if self.parent[x as usize] != x {
            self.parent[x as usize] = self.find(self.parent[x as usize]);
        }
        self.parent[x as usize]
    }

    fn union(&mut self, x: i32, y: i32) {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return;
        }
        if self.size[px as usize] < self.size[py as usize] {
            self.parent[px as usize] = py;
            self.size[py as usize] += self.size[px as usize];
        } else {
            self.parent[py as usize] = px;
            self.size[px as usize] += self.size[py as usize];
        }
    }
}

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut dsu = Dsu::new(n);
        for e in &edges {
            dsu.union(e[0], e[1]);
        }
        let mut result = 0;
        let mut roots = HashSet::new();
        let mut sizes = Vec::with_capacity(n as usize);
        for x in 0..n {
            let px = dsu.find(x);
            if !roots.contains(&px) {
                roots.insert(px);
                sizes.push(dsu.size[px as usize]);
            }
        }

        for s in sizes {
            result += s as i64 * (n - s) as i64;
        }

        result / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 3;
        let edges = [[0, 1], [0, 2], [1, 2]];
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        assert_eq!(0, Solution::count_pairs(n, edges));
    }

    #[test]
    fn case2() {
        let n = 7;
        let edges = [[0, 2], [0, 5], [2, 4], [1, 6], [5, 4]];
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        assert_eq!(14, Solution::count_pairs(n, edges));
    }
}
