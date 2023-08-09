pub struct Solution;

impl Solution {
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        fn f(nums: &[i32], max_diff: i32) -> i32 {
            let (mut c, mut i) = (0, 0);
            while i + 1 < nums.len() {
                if nums[i + 1] - nums[i] <= max_diff {
                    c += 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            c
        }
        nums.sort_unstable();
        let (mut left, mut right) = (-1, nums[nums.len() - 1] - nums[0]);
        while left + 1 < right {
            let mid = (right - left) / 2 + left;
            let n = f(&nums, mid);
            if n >= p {
                right = mid;
            } else {
                left = mid;
            }
        }
        right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::minimize_max(vec![10, 1, 2, 7, 1, 3], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::minimize_max(vec![4, 2, 1, 2], 1));
    }
}
