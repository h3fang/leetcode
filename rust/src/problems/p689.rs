pub struct Solution;

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut sum1 = 0;
        let mut sum2 = 0;
        let mut sum3 = 0;
        let mut max1 = 0;
        let mut max_id1 = 0;
        let mut max12 = 0;
        let mut max_id12 = (0, 0);
        let mut max123 = 0;
        let mut result = (0, 0, 0);

        let k = k as usize;
        for i in 2 * k..nums.len() {
            sum1 += nums[i - 2 * k];
            sum2 += nums[i - k];
            sum3 += nums[i];

            if i >= 3 * k - 1 {
                if sum1 > max1 {
                    max1 = sum1;
                    max_id1 = i + 1 - 3 * k;
                }
                if sum2 + max1 > max12 {
                    max12 = sum2 + max1;
                    max_id12 = (max_id1, i + 1 - 2 * k);
                }
                if sum3 + max12 > max123 {
                    max123 = sum3 + max12;
                    result = (max_id12.0, max_id12.1, i + 1 - k);
                }
                sum1 -= nums[i + 1 - 3 * k];
                sum2 -= nums[i + 1 - 2 * k];
                sum3 -= nums[i + 1 - k];
            }
        }

        vec![result.0 as i32, result.1 as i32, result.2 as i32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 2, 1, 2, 6, 7, 5, 1];
        assert_eq!(vec![0, 3, 5], Solution::max_sum_of_three_subarrays(nums, 2));
    }

    #[test]
    fn case2() {
        let nums = vec![1, 2, 1, 2, 1, 2, 1, 2, 1];
        assert_eq!(vec![0, 2, 4], Solution::max_sum_of_three_subarrays(nums, 2));
    }
}
