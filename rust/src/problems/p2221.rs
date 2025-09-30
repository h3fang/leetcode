pub struct Solution;

impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        let mut right = nums.len() - 1;
        while right > 0 {
            for i in 0..right {
                nums[i] = (nums[i] + nums[i + 1]) % 10;
            }
            right -= 1;
        }
        nums[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(8, Solution::triangular_sum(vec![1, 2, 3, 4, 5]));
    }
}
