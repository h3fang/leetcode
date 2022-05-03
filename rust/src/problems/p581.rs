pub struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left = 0;
        let mut right = -1;
        let mut max = nums[0];
        let mut min = nums[n - 1];
        for i in 0..n {
            if nums[i] < max {
                right = i as i32;
            } else {
                max = nums[i];
            }

            if nums[n - 1 - i] > min {
                left = (n - 1 - i) as i32;
            } else {
                min = nums[n - 1 - i];
            }
        }
        right - left + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            5,
            Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::find_unsorted_subarray(vec![2, 4, 4, 8]));
    }
}
