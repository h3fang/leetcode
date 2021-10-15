pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut sold, mut bought, mut cooldown) = (i32::MIN, i32::MIN, 0);
        for p in prices {
            let cooldown_new = cooldown.max(sold);
            sold = sold.max(bought + p);
            bought = bought.max(cooldown - p);
            cooldown = cooldown_new;
        }

        sold.max(cooldown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let prices = vec![1, 2, 3, 0, 2];
        assert_eq!(3, Solution::max_profit(prices));
    }

    #[test]
    fn case2() {
        let prices = vec![1];
        assert_eq!(0, Solution::max_profit(prices));
    }
}
