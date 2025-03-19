pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let max = amount + 1;
        let mut dp = vec![max; amount as usize + 1];
        dp[0] = 0;
        let amount = amount as usize;
        for i in 0..=amount {
            if dp[i] == max {
                continue;
            }
            for c in &coins {
                let j = i + *c as usize;
                if j <= amount {
                    dp[j] = dp[j].min(dp[i] + 1);
                }
            }
        }
        if dp[amount] == max { -1 } else { dp[amount] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let coins = vec![1, 2, 5];
        let amount = 11;
        assert_eq!(3, Solution::coin_change(coins, amount));
    }

    #[test]
    fn case2() {
        let coins = vec![2];
        let amount = 3;
        assert_eq!(-1, Solution::coin_change(coins, amount));
    }

    #[test]
    fn case3() {
        let coins = vec![1];
        let amount = 0;
        assert_eq!(0, Solution::coin_change(coins, amount));
    }

    #[test]
    fn case4() {
        let coins = vec![1];
        let amount = 1;
        assert_eq!(1, Solution::coin_change(coins, amount));
    }

    #[test]
    fn case5() {
        let coins = vec![1];
        let amount = 2;
        assert_eq!(2, Solution::coin_change(coins, amount));
    }
}
