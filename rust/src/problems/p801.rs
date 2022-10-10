pub struct Solution;

impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut dp = vec![[0, 0]; n];
        dp[0] = [0, 1];
        for i in 1..n {
            let c1 = nums1[i - 1] < nums1[i] && nums2[i - 1] < nums2[i];
            let c2 = nums2[i - 1] < nums1[i] && nums1[i - 1] < nums2[i];
            if c1 && !c2 {
                dp[i][0] = dp[i - 1][0];
                dp[i][1] = dp[i - 1][1] + 1;
            } else if !c1 && c2 {
                dp[i][0] = dp[i - 1][1];
                dp[i][1] = dp[i - 1][0] + 1;
            } else if c1 && c2 {
                dp[i][0] = dp[i - 1][0].min(dp[i - 1][1]);
                dp[i][1] = dp[i - 1][0].min(dp[i - 1][1]) + 1;
            }
        }
        dp[n - 1][0].min(dp[n - 1][1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_swap(vec![1, 3, 5, 4], vec![1, 2, 3, 7]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            1,
            Solution::min_swap(vec![0, 3, 5, 8, 9], vec![2, 1, 4, 6, 9])
        );
    }
}
