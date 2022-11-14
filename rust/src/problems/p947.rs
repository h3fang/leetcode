pub struct Solution;

use std::collections::{HashMap, HashSet};

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
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
    }
}

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut rows: HashMap<i32, usize> = HashMap::new();
        let mut cols: HashMap<i32, usize> = HashMap::new();
        let mut dsu = Dsu::new(stones.len());
        for (i, s) in stones.iter().enumerate() {
            if let Some(&r) = rows.get(&s[0]) {
                dsu.union(i, r);
            } else {
                rows.insert(s[0], i);
            }
            if let Some(&r) = cols.get(&s[1]) {
                dsu.union(i, r);
            } else {
                cols.insert(s[1], i);
            }
        }
        let s = (0..stones.len())
            .map(|i| dsu.find(i))
            .collect::<HashSet<_>>();
        (stones.len() - s.len()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let stones = [[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(5, Solution::remove_stones(stones));
    }

    #[test]
    fn case2() {
        let stones = [[0, 0], [0, 2], [1, 1], [2, 0], [2, 2]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(3, Solution::remove_stones(stones));
    }

    #[test]
    fn case3() {
        let stones = [[0, 0]].iter().map(|e| e.to_vec()).collect();
        assert_eq!(0, Solution::remove_stones(stones));
    }
}
