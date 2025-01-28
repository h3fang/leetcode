use rand::prelude::*;
use std::collections::HashMap;

pub struct Solution {
    m: usize,
    n: usize,
    total: usize,
    map: HashMap<usize, usize>,
    rng: ThreadRng,
}

impl Solution {
    pub fn new(m: i32, n: i32) -> Self {
        let m = m as usize;
        let n = n as usize;
        Self {
            m,
            n,
            total: m * n,
            map: HashMap::new(),
            rng: rand::rng(),
        }
    }

    pub fn flip(&mut self) -> Vec<i32> {
        let i = self.rng.random_range(0..self.total);
        self.total -= 1;
        let x = self.map.get(&i).cloned().unwrap_or(i);
        let r = vec![(x / self.n) as i32, (x % self.n) as i32];
        if x != self.total {
            self.map
                .insert(i, self.map.get(&self.total).cloned().unwrap_or(self.total));
        }

        r
    }

    pub fn reset(&mut self) {
        self.map.clear();
        self.total = self.m * self.n;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn case1() {
        let m = 5;
        let n = 10;
        let mut s = Solution::new(m, n);
        for _ in 0..100 {
            let mut flipped: HashSet<(i32, i32)> = HashSet::new();
            for _ in 0..1000.min(m * n) {
                let r = s.flip();
                assert!(flipped.insert((r[0], r[1])));
            }
            s.reset();
        }
    }
}
