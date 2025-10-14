pub struct Solution;

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let (n, k) = (nums.len(), k as usize);
        let mut left = vec![1; n];
        for i in 1..n {
            if nums[i - 1] < nums[i] {
                left[i] = left[i - 1] + 1;
            }
        }
        let mut right = 1;
        for i in (1..n).rev() {
            if i + 1 < n && nums[i] < nums[i + 1] {
                right += 1;
            } else {
                right = 1;
            }
            if right >= k && left[i - 1] >= k {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::has_increasing_subarrays(
            vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1],
            3
        ));
    }

    #[test]
    fn case2() {
        assert!(!Solution::has_increasing_subarrays(
            vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7],
            5
        ));
    }
}
