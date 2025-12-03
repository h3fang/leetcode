pub struct Solution;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let (n, mut l, mut r, mut min, mut result) = (nums.len(), k, k, nums[k], nums[k]);
        while l > 0 || r < n - 1 {
            let a = if l > 0 { nums[l - 1] } else { 0 };
            let b = if r < n - 1 { nums[r + 1] } else { 0 };
            if a < b {
                r += 1;
                min = min.min(nums[r]);
            } else {
                l -= 1;
                min = min.min(nums[l]);
            }
            result = result.max(min * (r - l + 1) as i32);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(15, Solution::maximum_score(vec![1, 4, 3, 7, 4, 5], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(20, Solution::maximum_score(vec![5, 5, 4, 5, 4, 1, 1, 1], 0));
    }
}
