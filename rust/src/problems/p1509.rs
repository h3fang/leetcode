pub struct Solution;

impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 3 {
            return 0;
        }
        nums.sort_unstable();
        (nums[n - 1] - nums[3])
            .min(nums[n - 2] - nums[2])
            .min(nums[n - 3] - nums[1])
            .min(nums[n - 4] - nums[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::min_difference(vec![5, 3, 2, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::min_difference(vec![1, 5, 0, 10, 14]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::min_difference(vec![3, 100, 20]));
    }
}
