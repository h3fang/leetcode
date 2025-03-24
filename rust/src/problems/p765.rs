pub struct Solution;

struct Dsu {
    parent: Vec<usize>,
    size: Vec<i32>,
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
        self.sets -= 1;
    }
}

impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let n = row.len() / 2;
        let mut dsu = Dsu::new(n);
        for c in row.chunks(2) {
            dsu.union(c[0] as usize / 2, c[1] as usize / 2);
        }
        (n - dsu.sets) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_swaps_couples(vec![0, 2, 1, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_swaps_couples(vec![3, 2, 0, 1]));
    }
}
