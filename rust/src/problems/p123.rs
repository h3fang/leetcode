pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = [-100000; 4];

        for p in prices {
            let dp0_new = dp[0].max(-p);
            dp[3] = dp[3].max(dp[2] + p);
            dp[2] = dp[2].max(dp[1] - p);
            dp[1] = dp[1].max(dp[0] + p);
            dp[0] = dp0_new;
        }

        dp[1].max(dp[3]).max(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::max_profit(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::max_profit(vec![6, 3, 1]));
    }

    #[test]
    fn case4() {
        assert_eq!(0, Solution::max_profit(vec![6]));
    }
}
