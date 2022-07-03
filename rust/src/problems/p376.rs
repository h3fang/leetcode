pub struct Solution;

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return n as i32;
        }
        let mut prev_diff = nums[1] - nums[0];
        let mut result = if prev_diff != 0 { 2 } else { 1 };
        for i in 2..n {
            let diff = nums[i] - nums[i - 1];
            if (diff < 0 && prev_diff >= 0) || (diff > 0 && prev_diff <= 0) {
                result += 1;
                prev_diff = diff;
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
        assert_eq!(6, Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 5]));
    }
}
