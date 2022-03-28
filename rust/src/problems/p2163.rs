use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let n = nums.len() / 3;
        let mut q = BinaryHeap::new();
        let mut sum = 0i64;
        for &e in nums.iter().take(n) {
            q.push(e);
            sum += e as i64;
        }
        let mut p1 = Vec::with_capacity(n + 1);
        p1.push(sum);
        for &e in nums.iter().skip(n).take(n) {
            q.push(e);
            sum += e as i64;
            let e = q.pop().unwrap();
            sum -= e as i64;
            p1.push(sum);
        }

        let mut q = BinaryHeap::new();
        sum = 0i64;
        for &e in nums.iter().skip(2 * n) {
            q.push(Reverse(e));
            sum += e as i64;
        }
        let mut result = p1[n] - sum;
        for (i, &e) in nums.iter().skip(n).take(n).enumerate().rev() {
            q.push(Reverse(e));
            sum += e as i64;
            let e = q.pop().unwrap();
            sum -= e.0 as i64;
            result = result.min(p1[i] - sum);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(-1, Solution::minimum_difference(vec![3, 1, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::minimum_difference(vec![7, 9, 5, 8, 1, 3]));
    }
}
