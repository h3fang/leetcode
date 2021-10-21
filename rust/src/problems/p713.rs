pub struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }
        let mut result = 0;
        let mut i = 0;
        let mut p = 1i64;
        for (j, n) in nums.iter().enumerate() {
            p *= *n as i64;
            while p >= k as i64 {
                p /= nums[i] as i64;
                i += 1;
            }
            result += j - i + 1;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![10, 5, 2, 6];
        let k = 100;
        assert_eq!(8, Solution::num_subarray_product_less_than_k(nums, k));
    }

    #[test]
    fn case2() {
        let nums = vec![1, 2, 3];
        let k = 0;
        assert_eq!(0, Solution::num_subarray_product_less_than_k(nums, k));
    }
}
