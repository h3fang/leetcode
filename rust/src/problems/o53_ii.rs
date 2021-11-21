pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = (left + right) / 2;
            if nums[mid] == mid as i32 {
                left = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                right = mid - 1;
            }
        }
        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![0, 1, 3];
        assert_eq!(2, Solution::missing_number(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 9];
        assert_eq!(8, Solution::missing_number(nums));
    }

    #[test]
    fn case3() {
        let nums = vec![0, 1, 2, 3, 4, 5, 6];
        assert_eq!(7, Solution::missing_number(nums));
    }

    #[test]
    fn case4() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(0, Solution::missing_number(nums));
    }
}
