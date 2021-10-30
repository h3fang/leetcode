pub struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut dp = 0;
        let mut s = dp;
        for i in 2..nums.len() {
            if nums[i] - nums[i - 1] == nums[i - 1] - nums[i - 2] {
                dp += 1;
                s += dp;
            } else {
                dp = 0;
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::number_of_arithmetic_slices(vec![1, 2]));
    }
}
