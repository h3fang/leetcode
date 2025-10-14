pub struct Solution;

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let (mut max, mut pre, mut cur) = (0, 0, 0);
        for (i, &x) in nums.iter().enumerate() {
            cur += 1;
            if i == n - 1 || x >= nums[i + 1] {
                max = max.max(cur / 2).max(cur.min(pre));
                pre = cur;
                cur = 0;
            }
        }
        max >= k
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
