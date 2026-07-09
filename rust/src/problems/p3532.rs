pub struct Solution;

use std::cmp::Ordering;

struct Dsu {
    parent: Vec<usize>,
    size: Vec<u32>,
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
            Ordering::Less => {
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
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut dsu = Dsu::new(n as usize);
        let mut l = 0;
        for (r, &x) in nums.iter().enumerate() {
            while l < r && (nums[l] - x).abs() > max_diff {
                l += 1;
            }

            dsu.union(l, r);
        }

        queries
            .into_iter()
            .map(|q| dsu.find(q[0] as usize) == dsu.find(q[1] as usize))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 3];
        let queries = [[0, 0], [0, 1]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(
            vec![true, false],
            Solution::path_existence_queries(2, nums, 1, queries)
        );
    }

    #[test]
    fn case2() {
        let nums = vec![2, 5, 6, 8];
        let queries = [[0, 1], [0, 2], [1, 3], [2, 3]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![false, false, true, true],
            Solution::path_existence_queries(4, nums, 2, queries)
        );
    }
}
