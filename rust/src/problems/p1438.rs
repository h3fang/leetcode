pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let (mut result, mut l) = (0, 0);
        let (mut max, mut min) = (
            VecDeque::with_capacity(nums.len()),
            VecDeque::with_capacity(nums.len()),
        );
        for (r, &x) in nums.iter().enumerate() {
            while !max.is_empty() && nums[*max.back().unwrap()] <= x {
                max.pop_back();
            }
            while !min.is_empty() && nums[*min.back().unwrap()] >= x {
                min.pop_back();
            }
            max.push_back(r);
            min.push_back(r);
            while !max.is_empty()
                && nums[*max.front().unwrap()] - nums[*min.front().unwrap()] > limit
            {
                l += 1;
                if *max.front().unwrap() < l {
                    max.pop_front();
                }
                if *min.front().unwrap() < l {
                    min.pop_front();
                }
            }
            result = result.max((r - l + 1) as i32);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::longest_subarray(vec![8, 2, 4, 7], 4));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::longest_subarray(vec![10, 1, 2, 4, 7, 2], 5));
    }

    #[test]
    fn case3() {
        assert_eq!(
            3,
            Solution::longest_subarray(vec![4, 2, 2, 2, 4, 4, 2, 2], 0)
        );
    }
}
