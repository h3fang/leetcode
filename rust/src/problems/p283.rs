pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut a = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(a, i);
                a += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        let expected = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(expected, nums);
    }

    #[test]
    fn case2() {
        let mut nums = vec![0];
        let expected = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(expected, nums);
    }

    #[test]
    fn case3() {
        let mut nums = vec![1];
        let expected = vec![1];
        Solution::move_zeroes(&mut nums);
        assert_eq!(expected, nums);
    }

    #[test]
    fn case4() {
        let mut nums = vec![0, 1, 0, 3, 0, 11, 12, 0];
        let expected = vec![1, 3, 11, 12, 0, 0, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(expected, nums);
    }

    #[test]
    fn case5() {
        let mut nums = vec![1, 2];
        let expected = vec![1, 2];
        Solution::move_zeroes(&mut nums);
        assert_eq!(expected, nums);
    }

    #[test]
    fn case6() {
        let mut nums = vec![1, 2, 3, 1];
        let expected = vec![1, 2, 3, 1];
        Solution::move_zeroes(&mut nums);
        assert_eq!(expected, nums);
    }
}
