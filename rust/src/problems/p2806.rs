pub struct Solution;

impl Solution {
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        let d = purchase_amount % 10;
        100 - (purchase_amount - d + if d >= 5 { 10 } else { 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(90, Solution::account_balance_after_purchase(9));
    }

    #[test]
    fn case2() {
        assert_eq!(80, Solution::account_balance_after_purchase(15));
    }
}
