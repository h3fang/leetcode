pub struct Solution;

impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut result = 1;
        let mut or = 0;
        let mut left = 0;
        for (right, &x) in nums.iter().enumerate() {
            while or & x > 0 {
                or &= !nums[left];
                left += 1;
            }
            or |= x;
            result = result.max(right - left + 1);
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::longest_nice_subarray(vec![1, 3, 8, 48, 10]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::longest_nice_subarray(vec![3, 1, 5, 11, 13]));
    }
}
