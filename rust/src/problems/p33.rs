pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let last = nums[n - 1];
        let check = |i: usize| {
            if nums[i] > last {
                target > last && nums[i] >= target
            } else {
                target > last || nums[i] >= target
            }
        };

        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let m = (right - left) / 2 + left;
            if check(m) {
                right = m;
            } else {
                left = m + 1;
            }
        }
        if nums[left] == target {
            left as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::search(vec![1], 0));
    }
}
