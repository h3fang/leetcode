pub struct Solution;

impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.insert(0, 1);
        nums.push(1);
        let mut dp = vec![vec![0; n + 2]; n + 2];
        for start in (0..=(n - 1)).rev() {
            for end in start + 2..=n + 1 {
                let mut best = 0;
                for f in start + 1..end {
                    let coins = dp[start][f] + dp[f][end] + nums[start] * nums[f] * nums[end];
                    best = best.max(coins);
                }
                dp[start][end] = best;
            }
        }
        dp[0][n + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(167, Solution::max_coins(vec![3, 1, 5, 8]));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::max_coins(vec![1, 5]));
    }
}
