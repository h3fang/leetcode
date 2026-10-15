pub struct Solution;

use std::collections::HashMap;

struct SegmentTree {
    sum: Vec<i64>,
    todo: Vec<i64>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        Self {
            sum: vec![0; n * 4],
            todo: vec![0; n * 4],
        }
    }

    fn update(&mut self, i: usize, l: usize, r: usize, delta: i64) {
        self.sum[i] += delta * (r - l + 1) as i64;
        self.todo[i] += delta;
    }

    fn query_and_add_one(&mut self, i: usize, l: usize, r: usize, lb: usize, rb: usize) -> i64 {
        if lb <= l && r <= rb {
            let ans = self.sum[i];
            self.update(i, l, r, 1);
            return ans;
        }

        let m = l.midpoint(r);
        let lazy = self.todo[i];
        if lazy != 0 {
            self.update(i * 2, l, m, lazy);
            self.update(i * 2 + 1, m + 1, r, lazy);
            self.todo[i] = 0;
        }

        let mut ans = 0;

        if lb <= m {
            ans += self.query_and_add_one(i * 2, l, m, lb, rb);
        }

        if m < rb {
            ans += self.query_and_add_one(i * 2 + 1, m + 1, r, lb, rb);
        }

        self.sum[i] = self.sum[i * 2] + self.sum[i * 2 + 1];

        ans
    }
}

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut tree = SegmentTree::new(n);
        let (mut sum, mut ans) = (0, 0);
        let mut last = HashMap::with_capacity(n * 2);
        for (i, x) in nums.into_iter().enumerate() {
            let j = last.get(&x).copied().unwrap_or(0);
            sum += tree.query_and_add_one(1, 1, n, j as usize + 1, i + 1) * 2 + i as i64 + 1 - j;
            ans += sum;
            last.insert(x, i as i64 + 1);
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(15, Solution::sum_counts(vec![1, 2, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::sum_counts(vec![2, 2]));
    }
}
