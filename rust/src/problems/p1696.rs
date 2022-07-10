pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = nums.len();
        let mut dp = vec![0; n];
        dp[0] = nums[0];
        let mut q = VecDeque::new();
        q.push_back(0);
        for i in 1..n {
            while !q.is_empty() && i - q.front().unwrap() > k {
                q.pop_front();
            }
            let j = *q.front().unwrap();
            dp[i] = dp[j] + nums[i];
            while !q.is_empty() && dp[*q.back().unwrap()] < dp[i] {
                q.pop_back();
            }
            q.push_back(i);
        }
        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(7, Solution::max_result(vec![1, -1, -2, 4, -7, 3], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(17, Solution::max_result(vec![10, -5, -2, 4, 0, 3], 3));
    }
}
