pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut dp = nums[0];
        let mut result = dp;
        for &n in &nums[1..] {
            dp = n.max(n + dp);
            result = result.max(dp);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(6, Solution::max_sub_array(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![-100000];
        assert_eq!(-100000, Solution::max_sub_array(nums));
    }
}
