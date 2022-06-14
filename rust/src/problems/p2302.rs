pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut result = 0;
        let mut sum = 0;
        let mut left = 0;
        for right in 0..nums.len() {
            sum += nums[right] as i64;
            while left <= right && sum * (right - left + 1) as i64 >= k {
                sum -= nums[left] as i64;
                left += 1;
            }
            if right >= left {
                result += (right - left + 1) as i64;
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
        assert_eq!(6, Solution::count_subarrays(vec![2, 1, 4, 3, 5], 10));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::count_subarrays(vec![1, 1, 1], 5));
    }
}
