pub struct Solution;

impl Solution {
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut result = 0;
        let mut odd = false;
        let mut prev = 0;
        while i < nums.len() {
            if !odd {
                prev = nums[i];
            } else {
                while i < nums.len() && nums[i] == prev {
                    i += 1;
                    result += 1;
                }
            }
            odd = !odd;
            i += 1;
        }
        if (nums.len() - result as usize) % 2 != 0 {
            result += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_deletion(vec![1, 1, 2, 3, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_deletion(vec![1, 1, 2, 2, 3, 3]));
    }
}
