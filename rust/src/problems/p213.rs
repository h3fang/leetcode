pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }

        fn rob_row(nums: &[i32]) -> i32 {
            let mut dp = [0, 0];
            for k in nums {
                let r = k + dp[0];
                dp[0] = dp[0].max(dp[1]);
                dp[1] = r;
            }
            dp[0].max(dp[1])
        }

        rob_row(&nums[1..]).max(rob_row(&nums[..n - 1]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::rob(vec![2, 3, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::rob(vec![1, 2, 3]));
    }
}
