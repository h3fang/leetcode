pub struct Solution;

impl Solution {
    pub fn minimize_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        (nums[n - 1] - nums[2])
            .min(nums[n - 3] - nums[0])
            .min(nums[n - 2] - nums[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::minimize_sum(vec![1, 4, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::minimize_sum(vec![1, 4, 7, 8, 5]));
    }
}
