pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut result = usize::MAX;
        let mut s = 0u32;

        for (right, n) in nums.iter().enumerate() {
            s += *n as u32;
            while s >= target as u32 {
                result = result.min(right - left + 1);
                s -= nums[left] as u32;
                left += 1;
            }
        }

        if result < usize::MAX {
            result as i32
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let target = 7;
        let nums = vec![2, 3, 1, 2, 4, 3];
        assert_eq!(2, Solution::min_sub_array_len(target, nums));
    }

    #[test]
    fn case2() {
        let target = 4;
        let nums = vec![1, 4, 4];
        assert_eq!(1, Solution::min_sub_array_len(target, nums));
    }

    #[test]
    fn case3() {
        let target = 11;
        let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(0, Solution::min_sub_array_len(target, nums));
    }
}
