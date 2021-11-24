use std::collections::HashMap;

pub struct Solution;

struct DisjointSetUnion {
    parents: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSetUnion {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            size: (0..n).map(|_| 1).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] != x {
            self.parents[x] = self.find(self.parents[x]);
        }
        self.parents[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return;
        }
        match self.size[px].cmp(&self.size[py]) {
            std::cmp::Ordering::Less => {
                self.parents[px] = py;
                self.size[py] += self.size[px];
            }
            _ => {
                self.parents[py] = px;
                self.size[px] += self.size[py];
            }
        }
    }

    fn size(&mut self, x: usize) -> usize {
        let px = self.find(x);
        self.size[px]
    }
}

impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dsu = DisjointSetUnion::new(n);
        let mut fs: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, a) in nums.iter().enumerate() {
            let mut a = *a;
            let mut x = 2;
            while x * x <= a {
                if a % x == 0 {
                    fs.entry(x).or_default().push(i);
                    a /= x;
                    while a % x == 0 {
                        a /= x;
                    }
                }
                x += 1;
            }
            if a > 1 {
                fs.entry(a).or_default().push(i);
            }
        }

        for indices in fs.values() {
            for w in indices.windows(2) {
                dsu.union(w[0], w[1]);
            }
        }
        (0..n).map(|i| dsu.size(i)).max().unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![4, 6, 15, 35];
        assert_eq!(4, Solution::largest_component_size(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![20, 50, 9, 63];
        assert_eq!(2, Solution::largest_component_size(nums));
    }

    #[test]
    fn case3() {
        let nums = vec![2, 3, 6, 7, 4, 12, 21, 39];
        assert_eq!(8, Solution::largest_component_size(nums));
    }
}
