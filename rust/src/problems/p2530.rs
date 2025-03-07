pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut q = BinaryHeap::from(nums);
        let mut result = 0;
        for _ in 0..k {
            let x = q.pop().unwrap();
            result += x as i64;
            q.push((x + 2) / 3);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(50, Solution::max_kelements(vec![10, 10, 10, 10, 10], 5));
    }

    #[test]
    fn case2() {
        assert_eq!(17, Solution::max_kelements(vec![1, 10, 3, 3, 3], 3));
    }
}
