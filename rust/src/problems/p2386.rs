pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn k_sum(mut nums: Vec<i32>, k: i32) -> i64 {
        let mut sum = 0;
        for n in nums.iter_mut() {
            if *n < 0 {
                *n = -*n;
            } else {
                sum += *n as i64;
            }
        }
        nums.sort_unstable();
        let mut q = BinaryHeap::new();
        q.push((sum, 0));
        for _ in 0..k - 1 {
            let (s, i) = q.pop().unwrap();
            if i == nums.len() {
                continue;
            }
            q.push((s - nums[i] as i64, i + 1));
            if i > 0 {
                q.push((s + nums[i - 1] as i64 - nums[i] as i64, i + 1));
            }
        }
        q.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::k_sum(vec![2, 4, -2], 5));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::k_sum(vec![1, -2, 3, 4, -10, 12], 16));
    }
}
