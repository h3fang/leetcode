pub struct Solution;

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
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

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let (px, py) = (self.find(x), self.find(y));
        if px == py {
            return;
        }
        match self.size[px].cmp(&self.size[py]) {
            std::cmp::Ordering::Less => {
                self.parent[px] = py;
                self.size[py] += self.size[py];
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
    pub fn min_time(n: i32, mut edges: Vec<Vec<i32>>, k: i32) -> i32 {
        if edges.is_empty() {
            return 0;
        }
        edges.sort_unstable_by_key(|e| -e[2]);

        let mut dsu = Dsu::new(n as usize);
        for e in edges {
            dsu.union(e[0] as usize, e[1] as usize);
            if dsu.groups < k as usize {
                return e[2];
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 2;
        let edges = [[0, 1, 3]].iter().map(|e| e.to_vec()).collect();
        let k = 2;
        assert_eq!(3, Solution::min_time(n, edges, k));
    }

    #[test]
    fn case2() {
        let n = 3;
        let edges = [[0, 1, 2], [1, 2, 4]].iter().map(|e| e.to_vec()).collect();
        let k = 3;
        assert_eq!(4, Solution::min_time(n, edges, k));
    }

    #[test]
    fn case3() {
        let n = 3;
        let edges = [[0, 2, 5]].iter().map(|e| e.to_vec()).collect();
        let k = 2;
        assert_eq!(0, Solution::min_time(n, edges, k));
    }
}
