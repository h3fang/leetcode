pub struct Solution;

impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        fn big_first(nums: &[i32]) -> i32 {
            let mut result = 0;
            for i in (1..nums.len()).step_by(2) {
                if i + 1 == nums.len() {
                    result += (nums[i] - nums[i - 1] + 1).max(0);
                } else {
                    result += (nums[i] - nums[i - 1] + 1)
                        .max(0)
                        .max((nums[i] - nums[i + 1] + 1).max(0));
                }
            }
            result
        }
        if nums.len() <= 1 {
            0
        } else {
            big_first(&nums).min((nums[0] - nums[1] + 1).max(0) + big_first(&nums[1..]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::moves_to_make_zigzag(vec![1, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::moves_to_make_zigzag(vec![9, 6, 1, 6, 2]));
    }
}
