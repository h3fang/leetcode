pub struct Solution;

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    sets: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            sets: n,
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
                self.size[py] += self.size[px];
                self.parent[px] = py;
            }
            _ => {
                self.size[px] += self.size[py];
                self.parent[py] = px;
            }
        }
        self.sets -= 1;
    }
}

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let n = grid.len();
        let mut dsu = Dsu::new(n * n * 4);
        for (i, r) in grid.iter().enumerate() {
            for (j, b) in r.bytes().enumerate() {
                let k = (i * n + j) * 4;
                match b {
                    b'/' => {
                        dsu.union(k, k + 3);
                        dsu.union(k + 1, k + 2);
                    }
                    b'\\' => {
                        dsu.union(k, k + 1);
                        dsu.union(k + 2, k + 3);
                    }
                    b' ' => {
                        dsu.union(k, k + 1);
                        dsu.union(k, k + 2);
                        dsu.union(k, k + 3);
                    }
                    _ => {}
                }
                if j < n - 1 {
                    dsu.union(k + 1, k + 7);
                }
                if i < n - 1 {
                    dsu.union(k + 2, k + n * 4);
                }
            }
        }
        dsu.sets as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [" /", "/ "].iter().map(|r| r.to_string()).collect();
        assert_eq!(2, Solution::regions_by_slashes(grid));
    }

    #[test]
    fn case2() {
        let grid = [" /", "  "].iter().map(|r| r.to_string()).collect();
        assert_eq!(1, Solution::regions_by_slashes(grid));
    }

    #[test]
    fn case3() {
        let grid = ["/\\", "\\/"].iter().map(|r| r.to_string()).collect();
        assert_eq!(5, Solution::regions_by_slashes(grid));
    }
}
