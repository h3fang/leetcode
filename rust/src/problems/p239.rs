use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len() - k as usize + 1);
        if nums.is_empty() {
            return result;
        }
        let mut s = VecDeque::new();
        for (i, &n) in nums.iter().enumerate() {
            while let Some(&f) = s.front() {
                if n >= nums[f] {
                    s.pop_front();
                } else {
                    break;
                }
            }
            s.push_front(i);
            if i >= k as usize {
                while *s.back().unwrap() <= i - k as usize {
                    s.pop_back();
                }
            }
            if i >= k as usize - 1 {
                result.push(nums[*s.back().unwrap()]);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        assert_eq!(
            vec![3, 3, 5, 5, 6, 7],
            Solution::max_sliding_window(nums, k)
        );
    }

    #[test]
    fn case2() {
        let nums = vec![1];
        let k = 1;
        assert_eq!(vec![1], Solution::max_sliding_window(nums, k));
    }

    #[test]
    fn case3() {
        let nums = vec![9, 11];
        let k = 2;
        assert_eq!(vec![11], Solution::max_sliding_window(nums, k));
    }

    #[test]
    fn case4() {
        let nums = vec![1, -1];
        let k = 1;
        assert_eq!(vec![1, -1], Solution::max_sliding_window(nums, k));
    }
}
