pub struct Solution;

impl Solution {
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let n = nums.len();
        let mut presum = vec![0; n + 1];
        for (i, e) in nums.iter().enumerate() {
            presum[i + 1] = presum[i] + e;
        }
        let mut dp = vec![vec![0.0; k + 1]; n + 1];
        for i in 1..n + 1 {
            dp[i][1] = presum[i] as f64 / i as f64;
        }
        for j in 2..=k {
            for i in j..n + 1 {
                for m in j - 1..i {
                    dp[i][j] = dp[i][j]
                        .max(dp[m][j - 1] + (presum[i] - presum[m]) as f64 / ((i - m) as f64));
                }
            }
        }
        dp[n][k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let expected = 20.0;
        let result = Solution::largest_sum_of_averages(vec![9, 1, 2, 3, 9], 3);
        assert!(
            (expected - result).abs() < 1e-5,
            "expected={}, result={}",
            expected,
            result
        );
    }

    #[test]
    fn case2() {
        let expected = 20.5;
        let result = Solution::largest_sum_of_averages(vec![1, 2, 3, 4, 5, 6, 7], 4);
        assert!(
            (expected - result).abs() < 1e-5,
            "expected={}, result={}",
            expected,
            result
        );
    }
}
