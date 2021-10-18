pub struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] > nums[mid + 1] {
                right = mid;
            } else {
                left = mid + 1;
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
        assert_eq!(2, Solution::find_peak_element(vec![1, 2, 3, 1]));
    }

    #[test]
    fn case2() {
        let expected = [1, 5];
        let r = Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]);
        assert!(expected.contains(&r));
    }
}
