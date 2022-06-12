pub struct Solution;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut prev = [-1; 10001];
        let mut left = 0;
        let mut sum = 0;
        let mut result = 0;
        for (i, &n) in nums.iter().enumerate() {
            sum += n;
            if prev[n as usize] >= left {
                for j in left..prev[n as usize] + 1 {
                    sum -= nums[j as usize];
                }
                left = prev[n as usize] + 1;
            }
            result = result.max(sum);
            prev[n as usize] = i as i32;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(17, Solution::maximum_unique_subarray(vec![4, 2, 4, 5, 6]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            8,
            Solution::maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5])
        );
    }
}
