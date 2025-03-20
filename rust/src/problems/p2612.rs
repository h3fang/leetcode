pub struct Solution;

use std::collections::VecDeque;

struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
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
        if px != py {
            self.parent[px] = py;
        }
    }
}

impl Solution {
    pub fn min_reverse_operations(n: i32, p: i32, banned: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans = vec![-1; n as usize];
        let mut dsu = Dsu::new(n as usize + 2);
        dsu.union(p as usize, p as usize + 2);
        for b in banned {
            let b = b as usize;
            dsu.union(b, b + 2);
        }
        ans[p as usize] = 0;
        let mut q = VecDeque::with_capacity(n as usize);
        q.push_back(p);
        let mut ops = 0;
        while !q.is_empty() {
            ops += 1;
            let m = q.len();
            for _ in 0..m {
                let i = q.pop_front().unwrap();
                let min = (i - k + 1).max(k - i - 1);
                let max = (i + k - 1).min(2 * n - k - i - 1);
                let mut j = dsu.find(min as usize);
                while j <= max as usize {
                    ans[j] = ops;
                    q.push_back(j as i32);
                    dsu.union(j, max as usize + 2);
                    j = dsu.find(j + 2);
                }
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
        assert_eq!(
            vec![0, -1, -1, 1],
            Solution::min_reverse_operations(4, 0, vec![1, 2], 4)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![0, -1, -1, -1, -1],
            Solution::min_reverse_operations(5, 0, vec![2, 4], 3)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![-1, -1, 0, -1],
            Solution::min_reverse_operations(4, 2, vec![0, 1, 3], 1)
        );
    }
}
