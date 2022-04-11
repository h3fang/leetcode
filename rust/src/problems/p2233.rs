use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut q = BinaryHeap::new();
        for n in nums {
            q.push(Reverse(n));
        }
        while k > 0 {
            let n = q.pop().unwrap().0;
            q.push(Reverse(n + 1));
            k -= 1;
        }
        let mut result = 1;
        const MOD: i64 = 10_0000_0007;
        for Reverse(n) in q {
            result = (result * n as i64) % MOD;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(20, Solution::maximum_product(vec![0, 4], 5));
    }

    #[test]
    fn case2() {
        assert_eq!(216, Solution::maximum_product(vec![6, 3, 3, 2], 2));
    }
}
