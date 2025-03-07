pub struct Solution;

impl Solution {
    pub fn buy_choco(mut prices: Vec<i32>, money: i32) -> i32 {
        prices.sort_unstable();
        let p = prices[0] + prices[1];
        if p > money {
            money
        } else {
            money - p
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::buy_choco(vec![1, 2, 2], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::buy_choco(vec![3, 2, 3], 3));
    }
}
