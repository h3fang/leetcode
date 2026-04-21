pub struct Solution;

use std::collections::HashMap;

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
    }
}

impl Solution {
    pub fn minimum_hamming_distance(
        source: Vec<i32>,
        target: Vec<i32>,
        allowed_swaps: Vec<Vec<i32>>,
    ) -> i32 {
        let n = source.len();
        let mut dsu = Dsu::new(n);
        for swap in allowed_swaps {
            dsu.union(swap[0] as usize, swap[1] as usize);
        }

        let mut groups: HashMap<usize, HashMap<i32, i32>> = HashMap::with_capacity(n);
        for (i, x) in source.into_iter().enumerate() {
            let root = dsu.find(i);
            *groups.entry(root).or_default().entry(x).or_default() += 1;
        }

        let mut ans = 0;

        for (i, x) in target.into_iter().enumerate() {
            let root = dsu.find(i);
            if let Some(g) = groups.get_mut(&root)
                && let Some(e) = g.get_mut(&x)
                && *e > 0
            {
                *e -= 1;
            } else {
                ans += 1;
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
        let allowed_swaps = [[0, 1], [2, 3]].iter().map(|s| s.to_vec()).collect();
        assert_eq!(
            1,
            Solution::minimum_hamming_distance(vec![1, 2, 3, 4], vec![2, 1, 4, 5], allowed_swaps)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::minimum_hamming_distance(vec![1, 2, 3, 4], vec![1, 3, 2, 4], vec![])
        );
    }

    #[test]
    fn case3() {
        let allowed_swaps = [[0, 4], [4, 2], [1, 3], [1, 4]]
            .iter()
            .map(|s| s.to_vec())
            .collect();
        assert_eq!(
            0,
            Solution::minimum_hamming_distance(
                vec![5, 1, 2, 4, 3],
                vec![1, 5, 4, 2, 3],
                allowed_swaps
            )
        );
    }
}
