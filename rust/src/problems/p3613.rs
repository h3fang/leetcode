pub struct Solution;

struct Dsu {
    parent: Vec<usize>,
    size: Vec<i32>,
    groups: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            groups: n,
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, x: usize, y: usize) {
        let (px, py) = (self.find(x), self.find(y));
        if px == py {
            return;
        }
        match self.size[px].cmp(&self.size[py]) {
            std::cmp::Ordering::Less => {
                self.parent[px] = py;
                self.size[py] += self.size[px];
            }
            _ => {
                self.parent[py] = px;
                self.size[px] += self.size[py];
            }
        }
        self.groups -= 1;
    }
}

impl Solution {
    pub fn min_cost(n: i32, mut edges: Vec<Vec<i32>>, k: i32) -> i32 {
        if k == n {
            return 0;
        }
        edges.sort_unstable_by_key(|e| e[2]);
        let (n, k) = (n as usize, k as usize);
        let mut dsu = Dsu::new(n);
        for e in edges {
            dsu.union(e[0] as usize, e[1] as usize);
            if dsu.groups <= k {
                return e[2];
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 5;
        let edges = [[0, 1, 4], [1, 2, 3], [1, 3, 2], [3, 4, 6]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let k = 2;
        assert_eq!(4, Solution::min_cost(n, edges, k));
    }

    #[test]
    fn case2() {
        let n = 4;
        let edges = [[0, 1, 5], [1, 2, 5], [2, 3, 5]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let k = 1;
        assert_eq!(5, Solution::min_cost(n, edges, k));
    }
}
