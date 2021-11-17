pub struct Solution;

impl Solution {
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
        let xor = nums.iter().fold(0, |acc, n| acc ^ n);
        let last_bit = xor & -xor;
        let mut first = 0;
        for n in nums {
            if n & last_bit > 0 {
                first ^= n;
            }
        }
        vec![first, first ^ xor]
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn case1() {
        let nums = vec![4, 1, 4, 6];
        let mut result = Solution::single_numbers(nums);
        result.sort_unstable();
        assert_eq!(vec![1, 6], result);
    }

    #[test]
    fn case2() {
        let nums = vec![1, 2, 10, 4, 1, 4, 3, 3];
        let mut result = Solution::single_numbers(nums);
        result.sort_unstable();
        assert_eq!(vec![2, 10], result);
    }
}
