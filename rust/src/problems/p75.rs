pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut i = 0;
        while left <= right {
            match nums[i].cmp(&1) {
                std::cmp::Ordering::Less => {
                    nums.swap(left, i);
                    left += 1;
                    i += 1;
                }
                std::cmp::Ordering::Equal => {
                    i += 1;
                }
                std::cmp::Ordering::Greater => {
                    nums.swap(right, i);
                    if right == 0 {
                        break;
                    }
                    right -= 1;
                }
            }
            if i > right {
                break;
            }
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
