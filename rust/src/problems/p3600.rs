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
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let (px, py) = (self.find(x), self.find(y));
        if px == py {
            return false;
        }

        match self.size[x].cmp(&self.size[y]) {
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

        true
    }
}

impl Solution {
    pub fn max_stability(n: i32, mut edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let (n, k) = (n as usize, k as usize);
        let (mut must, mut all) = (Dsu::new(n), Dsu::new(n));
        let mut min = i32::MAX;
        for e in &edges {
            let (x, y, s, m) = (e[0] as usize, e[1] as usize, e[2], e[3]);
            if m == 1 {
                if !must.union(x, y) {
                    return -1;
                }
                min = min.min(s);
            }
            all.union(x, y);
        }

        if all.groups > 1 {
            return -1;
        }

        let mut left = must.groups - 1;
        if left == 0 {
            return min;
        }

        edges.sort_unstable_by_key(|e| -e[2]);

        for e in &edges {
            let (x, y, s, m) = (e[0] as usize, e[1] as usize, e[2], e[3]);
            if m == 0 && must.union(x, y) {
                min = min.min(if left > k { s } else { s * 2 });
                left -= 1;
                if left == 0 {
                    break;
                }
            }
        }

        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let (n, k) = (3, 1);
        let edges = [[0, 1, 2, 1], [1, 2, 3, 0]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(2, Solution::max_stability(n, edges, k));
    }

    #[test]
    fn case2() {
        let (n, k) = (3, 2);
        let edges = [[0, 1, 4, 0], [1, 2, 3, 0], [0, 2, 1, 0]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(6, Solution::max_stability(n, edges, k));
    }

    #[test]
    fn case3() {
        let (n, k) = (3, 0);
        let edges = [[0, 1, 1, 1], [1, 2, 1, 1], [2, 0, 1, 1]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(-1, Solution::max_stability(n, edges, k));
    }
}
