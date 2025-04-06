pub struct Solution;

use std::collections::HashSet;

struct Input {
    nums: Vec<i32>,
    k: i32,
    limit: i32,
    suf: Vec<bool>,
    cache: HashSet<(usize, i32, i32, bool, bool)>,
    ans: i32,
}

impl Input {
    fn dfs(&mut self, i: usize, sum: i32, product: i32, odd: bool, empty: bool) {
        if i == self.nums.len() {
            if !empty && sum == self.k {
                self.ans = self.ans.max(product);
            }
            return;
        }
        let key = (i, sum, product, odd, empty);
        if !self.cache.insert(key) {
            return;
        }
        self.dfs(i + 1, sum, product, odd, empty);
        let x = self.nums[i];
        let mut product = product * x;
        if product > self.limit || product < 0 {
            product = -1;
            if !self.suf[i + 1] {
                return;
            }
        }
        let sum = sum + if odd { -x } else { x };
        self.dfs(i + 1, sum, product, !odd, false);
    }
}

impl Solution {
    pub fn max_product(nums: Vec<i32>, k: i32, limit: i32) -> i32 {
        let n = nums.len();
        let max = *nums.iter().max().unwrap();
        if (k >= 0 && k > (n as i32 + 1) / 2 * max) || (-k > (n as i32) / 2 * max) {
            return -1;
        }
        let mut suf = vec![false; n + 1];
        for (i, &x) in nums.iter().enumerate().rev() {
            suf[i] = suf[i + 1] || x == 0;
        }
        let mut input = Input {
            nums,
            k,
            limit,
            suf,
            cache: HashSet::with_capacity(5000),
            ans: -1,
        };
        input.dfs(0, 0, 1, false, true);
        input.ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::max_product(vec![1, 2, 3], 2, 10));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::max_product(vec![0, 2, 3], -5, 12));
    }

    #[test]
    fn case3() {
        assert_eq!(9, Solution::max_product(vec![2, 2, 3, 3], 0, 9));
    }

    #[test]
    fn case4() {
        assert_eq!(
            100,
            Solution::max_product(
                vec![
                    7, 8, 12, 2, 9, 0, 5, 12, 10, 1, 11, 9, 5, 9, 7, 12, 12, 12, 6, 7, 5, 7, 9, 2,
                    7, 7, 11, 8, 9, 1, 6, 12, 11, 6, 1, 4, 2, 6, 5, 4
                ],
                15,
                100
            )
        );
    }
}
