pub struct Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] == nums[mid ^ 1] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        nums[left]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 1, 2, 3, 3, 4, 4, 8, 8];
        assert_eq!(2, Solution::single_non_duplicate(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![3, 3, 7, 7, 10, 11, 11];
        assert_eq!(10, Solution::single_non_duplicate(nums));
    }
}
