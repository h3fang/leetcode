pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        let mut i = 0;
        while i <= right {
            while i <= right && nums[i as usize] == 2 {
                nums.swap(i as usize, right as usize);
                right -= 1;
            }
            if nums[i as usize] == 0 {
                nums.swap(i as usize, left);
                left += 1;
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        let expected = vec![0, 0, 1, 1, 2, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(expected, nums);
    }

    #[test]
    fn case2() {
        let mut nums = vec![2, 0, 1];
        let expected = vec![0, 1, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(expected, nums);
    }

    #[test]
    fn case3() {
        let mut nums = vec![2];
        let expected = vec![2];
        Solution::sort_colors(&mut nums);
        assert_eq!(expected, nums);
    }

    #[test]
    fn case4() {
        let mut nums = vec![1, 2, 0];
        let expected = vec![0, 1, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(expected, nums);
    }
}
