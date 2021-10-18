pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[left] <= nums[right] {
                return nums[left];
            } else if nums[mid] < nums[left] {
                left = 1;
                right = mid;
            } else {
                left = mid + 1;
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
        let nums = vec![3, 4, 5, 1, 2];
        assert_eq!(1, Solution::find_min(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![3];
        assert_eq!(3, Solution::find_min(nums));
    }

    #[test]
    fn case3() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(0, Solution::find_min(nums));
    }

    #[test]
    fn case4() {
        let nums = vec![11, 13, 15, 17];
        assert_eq!(11, Solution::find_min(nums));
    }

    #[test]
    fn case5() {
        let nums = vec![2, 1];
        assert_eq!(1, Solution::find_min(nums));
    }
}
