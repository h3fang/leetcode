pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = nums.len() + 1;
        let mut pre_sum = vec![0; nums.len() + 1];
        for (i, &n) in nums.iter().enumerate() {
            pre_sum[i + 1] = pre_sum[i] + n as i64;
        }
        let k = k as i64;
        let mut q = VecDeque::new();
        for (i, &s) in pre_sum.iter().enumerate() {
            while !q.is_empty() && s - pre_sum[*q.front().unwrap()] >= k {
                result = result.min(i - q.pop_front().unwrap());
            }
            while !q.is_empty() && pre_sum[*q.back().unwrap()] >= s {
                q.pop_back();
            }
            q.push_back(i);
        }
        if result < nums.len() + 1 {
            result as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::shortest_subarray(vec![1], 1));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::shortest_subarray(vec![1, 2], 4));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::shortest_subarray(vec![2, -1, 2], 3));
    }
}
