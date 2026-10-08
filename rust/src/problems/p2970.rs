pub struct Solution;

impl Solution {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut l = 1;
        while l < n && nums[l as usize] > nums[l as usize - 1] {
            l += 1;
        }
        let mut result = if l < n { l + 1 } else { l };
        for r in (0..n - 1).rev() {
            while l > 0 && nums[l as usize - 1] >= nums[r as usize + 1] {
                l -= 1;
            }
            result += if l <= r { l + 1 } else { l };
            if nums[r as usize] >= nums[r as usize + 1] {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(10, Solution::incremovable_subarray_count(vec![1, 2, 3, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(7, Solution::incremovable_subarray_count(vec![6, 5, 7, 8]));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::incremovable_subarray_count(vec![8, 7, 6, 6]));
    }
}
