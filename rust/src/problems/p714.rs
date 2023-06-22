pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut buy = -prices[0];
        let mut sell = 0;
        for p in prices {
            let prev = sell;
            sell = sell.max(buy + p - fee);
            buy = buy.max(prev - p);
        }
        sell
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(8, Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::max_profit(vec![1, 3, 7, 5, 10, 3], 3));
    }
}
